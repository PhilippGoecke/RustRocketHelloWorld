#[macro_use] extern crate rocket;

#[get("/?<name>")]
fn index(name: Option<&str>) -> String {
  let version = rocket::Config::release_profile();
  format!("Hello, {}!<br>(Rocket v{})", name.unwrap_or("world"), version)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
