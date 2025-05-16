/* 
* MIT License
* 
* Copyright (c) 2025 f42h
* 
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
* 
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
* 
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/

extern crate console;

use console::Term;
use std::io::{self, stdout, Write};

mod data;
use data::InputData;

mod hooks;
use hooks::Hooks;

mod prompt;
use prompt::prompt;

mod log;
use log::LogStream;

pub struct CliHistorySettings<'a> {
    label: &'a str,
    max_size: usize,
    max_size_log_file: usize,
    die_on_exit: bool,
    log_file_path: &'a str,
}

impl<'a> CliHistorySettings<'a> {
    pub fn new() -> Self {
        CliHistorySettings { 
            label: "CliHistoryPrompt: ",
            max_size: 500,
            max_size_log_file: 500,
            die_on_exit: false,
            log_file_path: "",
        }
    }

    pub fn set_label(&mut self, label: &'a str) {
        self.label = label;
    }

    pub fn set_max_size(&mut self, max_size: usize) {
        self.max_size = max_size;
    }

    pub fn set_die_on_exit(&mut self) {
        self.die_on_exit = true;
    }

    pub fn set_max_size_log_file(&mut self, max_size: usize) {
        self.max_size_log_file = max_size
    } 

    pub fn set_log_to_file(&mut self, file_path: &'a str) {
        self.log_file_path = file_path;
    } 
}

pub struct CliHistory<'a> {
    history: Vec<String>, // Data pool
    idx: usize, // History pool data index
    settings: &'a CliHistorySettings<'a>
}

impl<'a> CliHistory<'a> {
    pub fn new(settings: &'a CliHistorySettings<'a>) -> Self {
        CliHistory {
            history: Vec::new(), // Command pool
            idx: 0, // Need to navigate through the input history
            settings: settings
        }
    }

    fn set_label(&self, ignore: bool) {
        // Show the user that we want some input!! 
        if ignore {
            print!("{} ", self.settings.label);
        } else {
            print!("\r{} ", self.settings.label);
        }

        io::stdout().flush().unwrap();
    }

    fn get_label(&self) -> String {
        self.settings.label.to_string()
    }

    fn launch_prompt(&self, ignore: bool, no_lable: bool, last_char: char) -> String {
        if !no_lable {
            // Ignore the prompt label
            self.set_label(ignore);
        }

        // Ask the user for input..
        if let Some(input) = prompt(self.settings.label.to_string(), last_char) {
            input
        } else {
            String::new()
        }
    }

    fn value_add_history(&mut self, value: &str) {
        if self.idx == self.settings.max_size {
            self.history = Vec::new()
        }

        self.history.push(value.to_string()); // Add element to history
        self.idx = self.history.len(); // Update the index
    }

    pub fn get_history(&mut self) -> &mut Vec<String> {
        &mut self.history 
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
        let mut file_stream = LogStream::new(self.settings.log_file_path.to_string());
        let mut log_count = 0;

        if let Err(err) = file_stream.create_log_file() {
            term.write_line(&format!("Error creating {}: {}", self.settings.log_file_path, err)).unwrap();
        }

        'outer: loop {
            input.clear();

            if let Some(c) = last_char {
                if switch {
                    // Ignore the prompt label to avoid visual feedback issues..
                    input = self.launch_prompt(true, true, c);
                } else {
                    // Display full prompt
                    input = self.launch_prompt(true, false, c);
                }
            }

            term.flush().unwrap();

            if !input.is_empty() {
                self.value_add_history(&input);
                
                if log_count <= self.settings.max_size_log_file {
                    if let Err(err) = file_stream.append_log_file(input.as_str()) {
                        term.write_line(&format!("Error creating {}: {}", self.settings.log_file_path, err)).unwrap();
                    }
                    
                    log_count += 1;
                }

                callback(&input);
            }

            if self.settings.die_on_exit && input == "exit".to_string() {
                // Initialized with die_on_exit set to true
                stdout().flush().unwrap();
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
                                // Write input data to stdout
                                self.print_prompt_history(&mut term, &input, input_data.len);

                                if CliHistory::check_hook_enter(&term, &mut hooks) {
                                    callback(&input); // Send input to caller
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
                                // Write input data to stdout
                                self.print_prompt_history(&mut term, &input, input_data.len);

                                if CliHistory::check_hook_enter(&term, &mut hooks) {
                                    callback(&input); // Send input to caller
                                    break 'outer;
                                }
                            }
                        }
                    } else if hooks.is_enter() {
                        break 'outer;
                    } else {
                        // read_key() always ate the first char of the next command
                        // so we need to determine what was typed to "restore" the input
                        // eaten by read_key
                        if let Some(pressed_char) = Hooks::get_char(key) {
                            term.write(pressed_char.to_string().as_bytes()).unwrap();

                            last_char = Some(pressed_char);
                            switch = true
                        }

                        break 'inner;
                    }

                    if self.settings.die_on_exit && input == "exit".to_string() {
                        stdout().flush().unwrap();
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
    fn test_general() {
        let mut settings = CliHistorySettings::new();
        settings.set_label("Enter some text: ");
        settings.set_max_size(100);
        settings.set_die_on_exit();

        let mut cli_history = CliHistory::new(&settings);
        let callback = |command: &str| {
            dbg!(command);
        };

        let input = cli_history.launch_navigator(callback);
        dbg!(input);
    }
}
