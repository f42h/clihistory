extern crate console;

use console::{Term, Key};
use std::io::{self, BufRead, Write};

// History navigation instructions
#[derive(Debug, PartialEq)]
pub enum KeyHandle {
    None,
    ArrowKeyUp(bool),
    ArrowKeyDown(bool),
    EnterKey(bool)
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
            Key::Enter => {
                self.handle = KeyHandle::EnterKey(true);
            }
            _ => { // Reset only the keys that are not pressed
                self.handle = KeyHandle::ArrowKeyUp(false);
                self.handle = KeyHandle::ArrowKeyDown(false);
                self.handle = KeyHandle::EnterKey(false);
            }
        }
    }

    pub fn is_arrow_up(&self) -> bool {
        self.handle == KeyHandle::ArrowKeyUp(true)
    }

    pub fn is_arrow_down(&self) -> bool {
        self.handle == KeyHandle::ArrowKeyDown(true)
    }

    fn is_enter(&self) -> bool {
        self.handle == KeyHandle::EnterKey(true)
    }
}

struct InputData {
    data: String,
    len: usize
}

impl InputData {
    fn new(data: String, len: usize) -> Self {
        InputData { 
            data, 
            len 
        }
    }
}

pub struct CliHistory {
    label: &'static str, // Customizable input label 
    history: Vec<String>, // Data pool
    idx: usize, // History pool data index
    die_on_exit: bool // Immediate exit the main loop of history navigator
}

impl CliHistory {
    pub fn new(label: &'static str, die_on_exit: bool) -> Self {
        CliHistory {
            label, // The input label for the final prompt
            history: Vec::new(), // Command pool
            idx: 0, // Need to navigate through the input history
            die_on_exit
        }
    }

    fn set_label(&self) {
        // Show the user that we want some input!! 
        print!("\r{} ", self.label);
        io::stdout().flush().unwrap();
    }

    fn get_label(&self) -> &'static str {
        self.label
    }

    fn stdin_string(&self) -> String {
        let stdin = io::stdin();
        let mut input = String::new();

        // Read from stdin and append the input to out history
        stdin.lock().read_line(&mut input).expect("Failed to read from stdin");
        input.trim().to_string() 
    }

    fn launch_prompt(&self) -> String {
        // Ask the user for input..
        self.set_label();
        self.stdin_string()
    }

    fn value_add_history(&mut self, value: &str) {
        self.history.push(value.to_string()); // Add element to history
        self.idx = self.history.len(); // Update the index
    }

    pub fn get_history(&self) -> &Vec<String> {
        &self.history // The user wants the history, we give access to the the history..
    }

    fn history_iter_up(&mut self) -> Option<&String> {
        if self.idx > 0 {
            self.idx -= 1; // Update index by arrow key up
            return self.history.get(self.idx) // Fetch data by the new index
        }

        None
    }

    fn history_iter_down(&mut self) -> Option<&String> {
        if self.idx < self.history.len() {
            self.idx += 1; // Update index by arrow key down
            return self.history.get(self.idx.saturating_sub(1)) 
        }

        None
    }

    pub fn history_fill(cli_history: &mut CliHistory) -> String {
        // Launch prompt
        let input = cli_history.launch_prompt();
        // Fill history pool
        // Add everything typed to the input history
        cli_history.value_add_history(&input);
    
        input // Give control over the last input value
    }

    fn check_hook_enter(term: &Term, hooks: &mut Hooks) -> bool {
        if let Ok(key) = term.read_key() {
            hooks.update(key);
            hooks.is_enter()
        } else {
            false
        }
    }

    fn print_prompt_history(&self, term: &mut Term, input: &String, data_len: usize) {
        term.write(format!("\r{} {}{:>len$}", self.get_label(), input, " ", len=data_len).as_bytes()).unwrap();
        term.flush().unwrap();
    }

    pub fn launch_navigator(&mut self) -> String {
        let mut term = Term::stdout();
        let mut hooks = Hooks::new();
        let mut input = String::new(); // Return the value selected by the user.

        'outer: loop {
            input.clear();
            input = self.launch_prompt();

            if !input.is_empty() {
                self.value_add_history(&input);
            }

            if self.die_on_exit && input == "exit".to_string() {
                break 'outer;
            }

            'inner: loop {
                term.write(format!("\r{} ", self.get_label()).as_bytes()).unwrap();
                
                if let Ok(key) = term.read_key() {
                    hooks.update(key); // Update the key state!

                    if hooks.is_arrow_up() {
                        // Arrow up key was pressed: navigate from history last index to first
                        if let Some(command) = self.history_iter_up() {
                            let input_data = InputData::new(command.to_string(), input.chars().count());
                            input = input_data.data.clone();

                            if !input_data.data.is_empty() {
                                self.print_prompt_history(&mut term, &input, input_data.len);

                                if CliHistory::check_hook_enter(&term, &mut hooks) {
                                    break 'outer;
                                }
                            }
                        }
                    } else if hooks.is_arrow_down() {
                        // Arrow down key was pressed: navigate from history first index to last
                        if let Some(command) = self.history_iter_down() {
                            let input_data = InputData::new(command.to_string(), input.chars().count());
                            input = input_data.data.clone();

                            if !input_data.data.is_empty() {
                                self.print_prompt_history(&mut term, &input, input_data.len);

                                if CliHistory::check_hook_enter(&term, &mut hooks) {
                                    break 'outer;
                                }
                            }
                        }
                    } else {
                        break 'inner;
                    }

                    if self.die_on_exit && input == "exit".to_string() {
                        break 'outer;
                    }

                    term.flush().unwrap();
                }
            }
        }

        input
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt() {
        // Ensure the user is able to set the prompt lable and get the input data
        let prompt = CliHistory::new("myprompt:", false);
        let input = prompt.launch_prompt();

        println!("Value read from stdin: {}", input);
    }

    #[test] 
    fn test_history_fill() {
        let mut cli_history = CliHistory::new("CliHistoryPrompt:", false);
        
        loop {
            let get_history_value = CliHistory::history_fill(&mut cli_history);
            if get_history_value == "exit".to_string() {
                break;
            }

            println!("Value entered: {}", get_history_value);
        }

        dbg!(cli_history.get_history());
    }

    #[test] 
    fn test_navigator() {
        // Initialize CliHistory with a custom prompt, in this case: CliHistoryPrompt:
        let mut cli_history = CliHistory::new("CliHistoryPrompt:", false);
        
        loop {
            let get_history_value = CliHistory::history_fill(&mut cli_history);
            if get_history_value == "exit".to_string() { 
                break;
            }

            println!("Value entered: {}", get_history_value); 
        }

        dbg!(cli_history.get_history());

        let get_value = cli_history.launch_navigator();
        println!("Final value: {}", get_value);
    }

    #[test]
    fn test_general() {
        let mut cli_history = CliHistory::new("CliHistoryPrompt:", true);
        let input = cli_history.launch_navigator();
        dbg!(input);
    }
}
