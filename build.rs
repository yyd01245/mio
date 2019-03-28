// extern crate gcc;
extern crate cc;

use std::env;

#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

// And this function only gets compiled if the target OS is *not* linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

#[cfg(target_env = "musl")]
fn are_you_on_musl() {
    println!("You are running musl!");
}

// And this function only gets compiled if the target OS is *not* linux
#[cfg(not(target_os = "musl"))]
fn are_you_on_musl() {
    println!("You are *not* running musl!");
}

// #[cfg(target_env="musl")]
// fn main() {
//     println!("Target Env is MUSL");
//     let var = env::var("CARGO_CFG_TARGET_ENV").unwrap();
//     println!("ENV: {}", var);
// }

// #[cfg(target_env="gnu")]
fn main() {
    println!("TARGET ENV IS GNU");
    are_you_on_linux();
    are_you_on_musl();
     if cfg!(target_os = "macos") || cfg!(target_os = "ios") {
         println!("Think Different!");
     }
    let var = env::var("CARGO_CFG_TARGET_ENV").unwrap();
    println!("ENV: {}", var);
}