mod prepare_build;
mod utils;
mod verify_project;

use switch_statement;
use colored::Colorize;
use std::fs;
use crate::verify_project::verify_project;
use crate::prepare_build::construct_connectiq_project;
use crate::utils::manifest_utils::generate_manifest;

const BUILD_COMMAND: &str = "build";

fn main() {
    let command: String;
    match std::env::args().nth(1) {
        None => {
            eprintln!("{}", "No command was passed to Kumitateru. Exiting...".bright_red().bold());
            std::process::exit(1);
        }
        Some(_command) => {
            command = _command;
        }
    }

    // Check what command was in here
    switch_statement::switch! { command;
        BUILD_COMMAND => {
            println!("Building project...");
            println!("{} {}", "Step 1:".bold().bright_green(), "Verify project structure");
            verify_project();
            println!("{} {}", "Step 2:".bold().bright_green(), "Assemble a ConnectIQ Project");
            generate_manifest(fs::read_to_string("kumitateru.toml").unwrap());
            construct_connectiq_project();
        },
        _ => println!("No command found."),
    }
}

