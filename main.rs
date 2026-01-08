#[macro_use] extern crate rocket;

#[get("/?<name>")]
fn index(name: Option<&str>) -> String {
  format!("Hello, {}!", name.unwrap_or("world"))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
