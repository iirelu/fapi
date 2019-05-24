#![feature(proc_macro_hygiene, decl_macro)]
extern crate reqwest;
extern crate select;
extern crate regex;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod fa_req;

use rocket_contrib::json::JsonValue;
use fa_req::FaReq;
use rocket::State;
use rocket::http::Status;
use select::document::Document;
use select::predicate;

const NOT_FOUND_TEXT: &'static str =
    "The submission you are trying to find is not in our database.";

#[get("/submission/<id>")]
fn submission(id: u64, fa_req: State<FaReq>) -> Result<JsonValue, Status> {
    use select::predicate::{Attr, Class};

    let body = fa_req.get_submission_page(id)
        .map_err(|_| Status::NotFound)?;
    let doc = Document::from_read(body)
        .map_err(|_| Status::InternalServerError)?;

    for t in doc.find(predicate::Text) {
        if t.text() == NOT_FOUND_TEXT {
            return Err(Status::NotFound)
        }
    }

    let img = doc.find(Attr("id", "submissionImg")).next()
        .and_then(|img| img.attr("src"))
        .ok_or(Status::InternalServerError)?;

    let avatar = doc.find(Class("avatar")).nth(1)
        .and_then(|img| img.attr("src"))
        .ok_or(Status::InternalServerError)?;

    let attribution = dbg!(doc.find(Class("information")).next())
        .ok_or(Status::InternalServerError)?
        .children().collect::<Vec<_>>();

    let title = attribution.get(1)
        .ok_or(Status::InternalServerError)?.text();

    let author = attribution.get(3)
        .ok_or(Status::InternalServerError)?.text();

    let rating = if doc.find(Attr("alt", "Adult rating")).next().is_some() {
        "adult"
    } else if doc.find(Attr("alt", "Mature rating")).next().is_some() {
        "mature"
    } else if doc.find(Attr("alt", "General rating")).next().is_some() {
        "general"
    } else {
        "unknown"
    };

    Ok(json!({
        "image_url": format!("https:{}", img),
        "avatar": format!("https:{}", avatar),
        "title": title,
        "author": author,
        "rating": rating,
    }))
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "not found",
    })
}

#[catch(500)]
fn internal_error() -> JsonValue {
    json!({
        "status": "internal error",
    })
}

fn main() {
    use rocket::Config;
    use rocket::config::Environment;
    let config = Config::build(Environment::Development)
        .address("127.0.0.1")
        .port(3081)
        .unwrap();
    rocket::custom(config)
        .manage(FaReq::new())
        .register(catchers![not_found, internal_error])
        .mount("/", routes![submission])
        .launch();
}
