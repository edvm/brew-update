extern crate clap;

use clap::{App, Arg};

fn run_brew_commands (action: &str) {
    let mut commands = vec![
        "/opt/homebrew/bin/brew update",
        "/opt/homebrew/bin/brew upgrade",
        "/opt/homebrew/bin/brew cleanup -s",
    ];

    if action == "purge" {
        commands.insert(0, "/opt/homebrew/bin/brew cleanup --prune=all");
    }

    for command in commands {
        println!("Running command: {}", command);
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("failed to execute process");
        if !output.status.success() {
            println!("failed to execute process: {}", String::from_utf8_lossy(&output.stderr));
            std::process::exit(1);
        }
    }
}

fn main() {
    let possible_actions = vec!["update", "purge"];
    let matches = App::new("brew-update")
        .version("0.1.0")
        .author("edvm <edvm@fedoraproject.org>")
        .about("Brew update easy mode")
        .arg(
            Arg::with_name("action")
                .help("action to perform")
                .required(true)
                .possible_values(&possible_actions)
        )
        .get_matches();

    let action = matches.value_of("action").unwrap();

    if !possible_actions.contains(&action) {
        println!("Invalid action: {}", action);
        std::process::exit(1);
    }

    run_brew_commands(action);
    println!("Done!")

}
