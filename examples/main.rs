use clihistory::{CliHistory, CliHistorySettings};

fn main() {
    // Setup
    let mut settings = CliHistorySettings::new();
    settings.set_label("Enter some text:");
    settings.set_max_size(100);
    settings.set_die_on_exit();
    settings.set_log_to_file("history.txt");
    settings.set_max_size_log_file(100);

    // Initialize
    let mut cli_history = CliHistory::new(&settings);

    // Start input prompt handler
    let input: String = cli_history.launch_navigator(|command: &str| {
        dbg!(command); // Current command typed
    });

    // Get the collected data
    let history: &mut Vec<String> = cli_history.get_history();

    println!();

    // Display the full command history
    for (mut idx, element) in history.iter().enumerate() {
        idx += 1;
        println!("History Element Nr. {} = {}", idx, element);
    }

    println!();

    dbg!(input); // Selected value
}