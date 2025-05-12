# CliHistory - Library For Easy Input Prompt History Navigation

```
CliHistory was developed to simplify the handling of the 
command prompt. By providing easy use of a custom prompt, 
yet giving the user the simplest solution for analyzing Cli 
commands, a handler for them to store everything typed to a history.
Either direct access to the core history or instand access 
to the last typed command, or later analysis is no longer a problem. 
It starts as a fun side project for my other big project, but immediately 
grows with each customization and future plans. 

Hopefully this project can help you too!

This project is currently under highly active developement. If you have suggestions
for improvements or want to contribute, feel free to leave a note!
```

### Requirements
- crate: [console, v0.15.11](https://docs.rs/crate/console/latest)

### Usage
- We currently don't use crate as contributor, so for now we do it manually! 
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
- Add the `lib.rs` file to your `src` directory

### How to
- Import CliHistory
```rust
use clihistory::CliHistory;
```

#### Implement a new structure for CliHistory and specify the following settings:
```rust
pub fn new(label: &'static str, die_on_exit: bool) -> Self
```

- `label`: Specify your custom input prompt label
- `die_on_exit`: Define if the user is able to quit the CLI due to the exit command

#### To access retrieve the current command typed, we need to define a callback
```rust
let callback = |command: &str| {
    dbg!(command); // Print the last command typed!!
};
```

#### We need to launch the navigator to start the core history functionality
- Set the callback as parameter!
```rust
let input = cli_history.launch_navigator(callback);
dbg!(input); // Selected value
```

#### Retrieve and iterate through the history
```rust
// Access our command pool
let history = cli_history.get_history();

// Loop through the values previously typed
for (idx, element) in history.iter().enumerate() {
    println!("{}: {}", idx + 1, element);
}
```

### A Full Example
```rust
use clihistory::CliHistory;

fn main() {
    // Initialize CliHistory with a custom prompt, in this case: CliHistoryPrompt:
    let mut cli_history = CliHistory::new("CliHistoryPrompt:", true);

    // Define callback to retrieve the current command typed
    let callback = |command: &str| {
        dbg!(command);
    };

    // The navigator will let the user navigate through the input history
    // with the up/down arrow keys. Once a value from the history is selected,
    // We will immediately return the selected value.
    let input = cli_history.launch_navigator(callback);

    dbg!(input); // Selected value

    let history = cli_history.get_history();

    // Display the full command history with index
    for (idx, element) in history.iter().enumerate() {
        println!("{}: {}", idx + 1, element);
    }
}
```

# License
#### This project is published under the [MIT](https://github.com/f42h/clihistory/blob/master/LICENSE) license.