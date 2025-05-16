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

use std::io;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

pub(super) struct LogStream {
    path: String,
    stream: Option<File>,
}

impl LogStream {
    pub(super) fn new(path: String) -> Self {
        LogStream { 
            path,
            stream: None, 
        }
    }

    pub(super) fn create_log_file(&mut self) -> Result<(), io::Error> {
        // Create history log file if it doesn't already exist 
        // and share the open stream within this implementation
        self.stream = Some(OpenOptions::new()
            .write(true)
            .append(true)
            .create(true) 
            .open(&self.path)?);

        Ok(())
    }
    
    pub(super) fn append_log_file(&mut self, content: &str) -> Result<(), io::Error> {
        if let Some(ref mut stream) = self.stream {
            // Ensure we can write to the file stream and append the content if able
            stream.write_all(format!("{}\n", content).as_bytes())?;
            stream.flush()?;
        }

        Ok(())
    }
}