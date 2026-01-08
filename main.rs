#[macro_use] extern crate rocket;

#[get("/?<name>")]
fn index(name: Option<&str>) -> String {
  use rocket::config::Config;
  format!("Hello, {}!<br>(Rocket v{})", name.unwrap_or("world"), rocket::version().unwrap_or("unknown"))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
