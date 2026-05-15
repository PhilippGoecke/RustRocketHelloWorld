#[macro_use] extern crate rocket;

use std::process::Command;

#[get("/?<name>")]
fn index(name: Option<&str>) -> String {
    let greeting = format!("Hello, {}!", name.unwrap_or("World"));

    let rust_version = Command::new("rustc")
        .arg("--version")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let cargo_version = Command::new("cargo")
        .arg("--version")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let rocket_version = format!("rocket {}", format!("{:?}", rocket::Config::default()).lines().next().unwrap_or(""));
    // Prefer the compile-time crate version of rocket via Cargo metadata env var.
    let rocket_version = option_env!("CARGO_PKG_DEPENDENCIES_ROCKET")
        .map(|v| format!("rocket {}", v))
        .unwrap_or(rocket_version);

    format!(
        "{}\nrustc: {}\ncargo: {}\n{}",
        greeting, rust_version, cargo_version, rocket_version
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
