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

use std::io::Write;
use console::{Key, Term};

use super::hooks::Hooks;

struct CharCollection {
    data: Vec<char>,
    idx: usize, 
}

impl CharCollection {
    fn new() -> Self {
        CharCollection { 
            data: Vec::new(), 
            idx: 0 
        }
    }

    fn del_last(&mut self) {
        if self.idx > 0 {
            self.data.remove(self.idx - 1);
            self.idx -= 1;
        }
    } 
}

fn cursor_manage(collection: &mut CharCollection, cursor: char) {
    collection.data.retain(|&c| c != cursor); // Remove previous cursor from input vector
                
    if collection.idx <= collection.data.len() { 
        // Insert new cursor in new position
        collection.data.insert(collection.idx.clone(), cursor);
    }
}

pub(crate) fn prompt(label: String, last_char: char) -> Option<String> {
    let mut collection = CharCollection::new();
    let mut term = Term::stdout();
    let mut command: String = String::new();

    term.hide_cursor().unwrap();
    let cursor = '|';

    collection.data.insert(0, last_char);
    collection.idx += 1;

    loop {
        let key = match term.read_key() {
            Ok(key) => key,
            Err(_) => {
                return None;
            }
        };

        if key == Key::Enter {
            break;
        } else if key == Key::Char(' ') {
            // Insert space
            collection.data.insert(collection.idx, ' ');
            collection.idx += 1;
        } else if let Some(c) = Hooks::get_char(key.clone()) {
            collection.data.insert(collection.idx, c);
            collection.idx += 1;
        } else if key == Key::ArrowLeft {
            if collection.idx > 0 {
                collection.idx -= 1;

                cursor_manage(&mut collection, cursor);
                term.move_cursor_left(1).unwrap(); // Move to the next char
            }
        } else if key == Key::ArrowRight {
            if collection.idx < collection.data.len() {
                collection.idx += 1;        

                cursor_manage(&mut collection, cursor);
                term.move_cursor_right(1).unwrap();
            }
        } else if key == Key::Backspace {
            collection.del_last();
        }

        command.clear();
        term.clear_line().unwrap();

        command = collection.data.iter().collect::<String>();
        term.write(&format!("{} {}", label, command).as_bytes()).unwrap();

        term.flush().unwrap();
    }

    // Remove cursor from result
    Some(command.replace(cursor, ""))
}


