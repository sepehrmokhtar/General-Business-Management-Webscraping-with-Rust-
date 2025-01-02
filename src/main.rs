#[macro_use] extern crate rocket;

use rocket::form::Form;
use rocket::serde::{json::Json, Deserialize, Serialize};
use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::collections::HashMap;

#[derive(FromForm)]
struct UrlForm {
    url: String,
}

#[derive(Serialize)]
struct ScrapedData {
    titles: Vec<String>,
}

#[post("/scrape", data = "<url_form>")]
fn scrape(url_form: Form<UrlForm>) -> Json<ScrapedData> {
    let url = &url_form.url;
    let response = get(url).expect("Failed to send request");
    assert!(response.status().is_success());
    let body = response.text().expect("Failed to read response body");
    
    let document = Html::parse_document(&body);
    let selector = Selector::parse("h1, h2, h3").expect("Failed to parse selector");

    let mut titles = Vec::new();
    for element in document.select(&selector) {
        titles.push(element.text().collect::<Vec<_>>().join(" "));
    }

    Json(ScrapedData { titles })
}

#[get("/")]
fn index() -> &'static str {
    r#"
    <!DOCTYPE html>
    <html>
        <head>
            <title>Web Scraper</title>
        </head>
        <body>
            <h1>Web Scraper</h1>
            <form action="/scrape" method="post">
                <label for="url">Enter URL:</label>
                <input type="text" id="url" name="url">
                <input type="submit" value="Scrape">
            </form>
        </body>
    </html>
    "#
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![scrape])
}