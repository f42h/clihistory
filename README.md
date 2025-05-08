# CliHistory - Library For Easy Input Prompt History Navigation

### Usage

### Example
```rust
use clihistory::CliHistory;

fn main() {
    // Initialize CliHistory with a custom prompt, in this case: CliHistoryPrompt:
    let mut cli_history = CliHistory::new("CliHistoryPrompt:", true);

    loop {
        // The navigator will let the user navigate through the input history
        // with the up/down arrow keys. Once a value from the history is selected,
        // We will immediately return the selected value.
        let input = cli_history.launch_navigator();

        dbg!(input);
    }
}
```