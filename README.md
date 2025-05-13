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
- Import CliHistory
```rust
use clihistory::CliHistory;
```

#### Initialize CliHistory:
```rust
pub fn new(label: &'static str, die_on_exit: bool) -> Self
```

- `label`: Specify a custom input prompt label
- `die_on_exit`: Define if the user is able to quit the CLI due to the "exit" command

#### To access retrieve the current command typed, we need to define a callback
- Create a new variable to get the current command typed
```rust
let callback = |command: &str| {
    dbg!(command); // This will display every command typed
};
```

#### We need to launch the navigator to start the core history functionality
```rust
pub fn launch_navigator<CommandCallback>(&mut self, callback: CommandCallback) -> String 
```

```rust
// Execute the navigator to collect all commands typed
let input = cli_history.launch_navigator(callback);
dbg!(input);
```

#### Retrieve and iterate through the history
```rust
// If we need the full history, we can get it with get_history()
let history = cli_history.get_history();

for element in history {
    println!("{}", element);
}
```

### A Full Example
```rust
use clihistory::CliHistory;

fn main() {
    // Initialize CliHistory with a custom prompt and specify if you 
    // want the main operation to stop if the user inputs the "exit" command
    let mut cli_history = CliHistory::new("CliHistoryPrompt:", true);

    // The navigator will let the you navigate through the input history
    // with the up/down arrow keys. Once a value from the history is selected,
    // CliHistory will immediately return the selected value.
    let input: String = cli_history.launch_navigator(|command: &str| {
        dbg!(command);
    });

    // Get the collected stdin data
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