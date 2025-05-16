# CliHistory - Library For Easy Input Prompt History Navigation

```
CliHistory was developed to simplify the handling of the 
command prompt. By providing easy use of a custom prompt, 
giving your application the simplest solution for analyzing CLI
commands and a handler for them to store everything typed to a history.
Either direct access to the core history or instant access 
to the last typed command, or later analysis is no longer a problem. 
It starts as a fun side project for my other big project, but immediately 
grows with each improvements and future plans. 

Hopefully this project can help you too!

This project is currently under highly active developement. If you have suggestions
for improvements or want to contribute, feel free to leave a note!
```

### Requirements
- crate: [console, v0.15.11](https://docs.rs/crate/console/latest)

### Usage
- We currently don't use crates.io as contributor, so for now we do it manually! 
- Add a `[lib]` section to your projects `Cargo.toml` and specify CliHistory
```
[lib]
name = "clihistory"
path = "src/lib.rs"
```
- Clone the repository
```
git clone https://github.com/f42h/clihistory.git
```
- Add the `clihistory/src/*.rs` files to your projects `src` directory

### How to
- Import CliHistory and CliHistorySettings
```rust
use clihistory::{CliHistory, CliHistorySettings};
```

- Specify settings for CliHistory
```rust
let mut settings = CliHistorySettings::new(); 
```

##### Available Settings:
- Set a custom prompt
```rust
pub fn set_label(&mut self, label: &'a str)
``` 

- Specify how many entries the history can contain until it will be cleared
```rust
// Default: 500
pub fn set_max_size(&mut self, max_size: usize)
```

- Save commands to history file
```rust
pub fn set_log_to_file(&mut self, file_path: &'a str)
```

- Specify how many entries will be written to history file 
```rust
// Default: 500
pub fn set_max_size_log_file(&mut self, max_size: usize)
```

- Tell the navigator to stop when receiving the "exit" command
```rust
pub fn set_die_on_exit(&mut self)
```

##### Initialize CliHistory:
- Add settings to CliHistory
```rust
pub fn new(settings: &'a CliHistorySettings<'a>) -> Self 
```

### A Full Example
```rust
use clihistory::CliHistory;

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
```

### Output
```
CliHistoryPrompt: ls
[src/main.rs:7:9] command = "ls"
CliHistoryPrompt: ip a
[src/main.rs:7:9] command = "ip a"
CliHistoryPrompt: dir
[src/main.rs:7:9] command = "dir"
CliHistoryPrompt: ip a  // This was selected with the arrow up key
[src/main.rs:7:9] command = "ip a"

History Element Nr. 1 = ls
History Element Nr. 2 = ip a
History Element Nr. 3 = dir

[src/main.rs:21:5] input = "ip a"
```

# License
#### This project is published under the [MIT](https://github.com/f42h/clihistory/blob/master/LICENSE) license.