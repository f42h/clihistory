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

use console::Key;

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
            _ => { 
                // Reset the key states
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

    pub fn is_enter(&self) -> bool {
        self.handle == KeyHandle::EnterKey(true)
    }

    pub fn get_char(key: Key) -> Option<char> {
        for i in 0..26 { 
            // Iterate through the alphabet to determine the first pressed key
            let c = (b'a' + i) as char;
            if key == Key::Char(c) {
                return Some(c)
            }
        }

        None
    }
}