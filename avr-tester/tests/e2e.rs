#![feature(exit_status_error)]

#[path = "e2e/uart_eval.rs"]
mod uart_eval;

#[path = "e2e/uart_rot13.rs"]
mod uart_rot13;

mod prelude {
    pub use super::build;
    pub use avr_tester::*;
    pub use rand::prelude::*;
}

use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub fn build(test: &str) -> PathBuf {
    eprintln!("Building firmware");

    let path = Path::new("tests").join("e2e").join("sources");

    Command::new("cargo")
        .arg("build")
        .arg("--release")
        .current_dir(path.join(test))
        .status()
        .expect("Couldn't build example")
        .exit_ok()
        .expect("Couldn't build example");

    eprintln!("Starting test");

    path.join("target")
        .join("atmega328p")
        .join("release")
        .join(format!("e2e-{}.elf", test))
}
