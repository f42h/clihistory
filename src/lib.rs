use console::{Term, Key};
use std::io::{self, BufRead, Write};

// History navigation instructions
#[derive(Debug, PartialEq)]
pub enum KeyHandle {
    None,
    ArrowKeyUp(bool),
    ArrowKeyDown(bool),
}

// Keyboard Hook handling
pub struct Hooks {
    handle: KeyHandle,
}

impl Hooks {
    pub fn new() -> Self {
        Hooks {
            handle: KeyHandle::None,
        }
    }

    // Update the current arrow key state 
    pub fn update(&mut self, key: Key) {
        match key {
            Key::ArrowUp => {
                self.handle = KeyHandle::ArrowKeyUp(true);
            }
            Key::ArrowDown => {
                self.handle = KeyHandle::ArrowKeyDown(true);
            }
            _ => { // Reset!
                self.handle = KeyHandle::ArrowKeyUp(false);
                self.handle = KeyHandle::ArrowKeyDown(false);
            }
        }
    }

    pub fn is_arrow_up(&self) -> bool {
        self.handle == KeyHandle::ArrowKeyUp(true)
    }

    pub fn is_arrow_down(&self) -> bool {
        self.handle == KeyHandle::ArrowKeyDown(true)
    }
}

pub struct CliHistory {
    label: &'static str,
    history: Vec<String>,
    idx: usize,
}

impl CliHistory {
    pub fn new(label: &'static str) -> Self {
        CliHistory {
            label, // The input label for the final prompt
            history: Vec::new(), // Command pool
            idx: 0, // Need to navigate through the input history
        }
    }

    fn set_label(&self) {
        // Show the user that we want some input!! 
        print!("{} ", self.label);
        io::stdout().flush().unwrap();
    }

    fn stdin_string(&self) -> String {
        let stdin = io::stdin();
        let mut input = String::new();

        // Read from stdin and append the input to out history
        stdin.lock().read_line(&mut input).expect("Failed to read from stdin");
        input.trim().to_string() 
    }

    pub fn launch_prompt(&self) -> String {
        // Ask the user for input..
        self.set_label();
        self.stdin_string()
    }

    pub fn value_add_history(&mut self, value: &str) {
        self.history.push(value.to_string()); // Add element to history
        self.idx = self.history.len(); // Update the index
    }

    pub fn get_history(&self) -> &Vec<String> {
        &self.history // The user wants the history, we give access to the the history..
    }

    pub fn history_iter_up(&mut self) -> Option<&String> {
        if self.idx > 0 {
            self.idx -= 1; // Update index by arrow key up
            return self.history.get(self.idx) // Fetch data by the new index
        }

        None
    }

    pub fn history_iter_down(&mut self) -> Option<&String> {
        if self.idx < self.history.len() {
            self.idx += 1; // Update index by arrow key down
            return self.history.get(self.idx.saturating_sub(1)) // We don't want and index overflow..
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt() {
        // Ensure the user is able to set the prompt lable and get the input data
        let prompt = CliHistory::new("myprompt:");
        let input = prompt.launch_prompt();

        println!("Value read from stdin: {}", input);
    }

    #[test]
    fn test_add_history() {
        let mut prompt = CliHistory::new("myprompt:");

        loop {
            // Collect data over and over again.. until EXIT!!
            let input = prompt.launch_prompt();
            println!("Value read from stdin: {}", input);

            if input.trim() == "exit" {
                // Don't want to type anymore...........
                break;
            }

            // Add everything typed to the input history
            prompt.value_add_history(&input);
        }

        let history = prompt.get_history();
        dbg!(history.clone()); // Show me what i typed!!
    }

    #[test]
    fn test_key_pressed() {
        let term = Term::stdout();
        let mut hooks = Hooks::new();
        let mut cli_history = CliHistory::new("myprompt:");

        // Damn they're legends..
        cli_history.value_add_history("Eaten Back To Life");
        cli_history.value_add_history("Red Before Black");
        cli_history.value_add_history("Chaos Horrific");
        cli_history.value_add_history("Tomb Of The Mutilated");
        // And so much more perfect work..

        loop {
            if let Ok(key) = term.read_key() {
                hooks.update(key); // Update the key state!

                if hooks.is_arrow_up() {
                    if let Some(command) = cli_history.history_iter_up() {
                        term.write_line(&format!("ArrowUp: {}", command)).unwrap();
                    }
                } else if hooks.is_arrow_down() {
                    if let Some(command) = cli_history.history_iter_down() {
                        term.write_line(&format!("ArrowDown: {}", command)).unwrap();
                    }
                }

                term.flush().unwrap();
            }
        }
    }
}
