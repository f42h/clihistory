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

    fn get_char(key: Key) -> Option<char> {
        for i in 0..26 { 
            // We want to iterate through the alphabet to determine which key might pressed
            let c = (b'a' + i) as char;
            if key == Key::Char(c) {
                return Some(c)
            }
        }

        None
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

    fn set_label(&self, ignore: bool) {
        // Show the user that we want some input!! 
        if ignore {
            print!("{} ", self.label);
        } else {
            print!("\r{} ", self.label);
        }

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

    fn launch_prompt(&self, ignore: bool, no_lable: bool) -> String {
        // Ask the user for input..
        if !no_lable {
            self.set_label(ignore);
        }

        self.stdin_string()
    }

    fn value_add_history(&mut self, value: &str) {
        self.history.push(value.to_string()); // Add element to history
        self.idx = self.history.len(); // Update the index
    }

    pub fn get_history(&mut self) -> &mut Vec<String> {
        &mut self.history // The user wants the history, we give access to the the history..
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
        let input = cli_history.launch_prompt(true, false);
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

    pub fn launch_navigator<CommandCallback>(&mut self, callback: CommandCallback) -> String 
    where CommandCallback: Fn(&str) {
        let mut term = Term::stdout();
        let mut hooks = Hooks::new();
        let mut input = String::new(); // Return the value selected by the user.
        let mut switch = false;
        let mut last_char: Option<char> = None;

        'outer: loop {
            input.clear();

            if switch {
                // Ignore the prompt label to avoid visual feedback issues..
                input = self.launch_prompt(true, true);
                
                if !last_char.is_none() {
                    if let Some(c) = last_char {
                        // Construct the new input with the first char 
                        input = format!("{}{}", c, input);
                    }
                }
            } else {
                // Display full prompt
                input = self.launch_prompt(true, false);
            }

            if !input.is_empty() {
                self.value_add_history(&input);
                callback(&input);
            }

            if self.die_on_exit && input == "exit".to_string() {
                break 'outer;
            }

            'inner: loop {
                term.write(format!("\r{} ", self.get_label()).as_bytes()).unwrap();
                
                if let Ok(key) = term.read_key() {
                    hooks.update(key.clone()); // Update the key state!

                    if hooks.is_arrow_up() {
                        // Arrow up key was pressed: navigate from history last index to first
                        if let Some(command) = self.history_iter_up() {
                            let input_data = InputData::new(command.to_string(), input.chars().count());
                            input = input_data.data.clone();

                            if !input_data.data.is_empty() {
                                self.print_prompt_history(&mut term, &input, input_data.len);
                                callback(&input);

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
                                callback(&input);

                                if CliHistory::check_hook_enter(&term, &mut hooks) {
                                    break 'outer;
                                }
                            }
                        }
                    } else if hooks.is_enter() {
                        break 'outer;
                    } else {
                        if let Some(pressed_char) = Hooks::get_char(key) {
                            term.write(pressed_char.to_string().as_bytes()).unwrap();

                            last_char = Some(pressed_char);
                            switch = true
                        }

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
        let input = prompt.launch_prompt(true, false);

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
    fn test_general() {
        let mut cli_history = CliHistory::new("CliHistoryPrompt:", true);
        let callback = |command: &str| {
            dbg!(command);
        };

        let input = cli_history.launch_navigator(callback);
        dbg!(input);
    }
}
