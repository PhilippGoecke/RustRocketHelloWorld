#[macro_use] extern crate rocket;

#[get("/?<name>")]
use rocket::config::Config;
fn index(name: Option<&str>) -> String {
  let version = "0.5.0";
  format!("Hello, {}!<br>(Rocket v{})", name.unwrap_or("world"), Config::release())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
