<h1><code>webster-dictionary (spellthis) 🦀</code></h1>

  A college assignment dictionary and spell checker.

### Installing Rust
Go to [rust-lang](https://www.rust-lang.org/tools/install) and follow the instructions.

### How to use
use the command:
```bash
cargo run define [WORD]
```
to search for the definition of word.

and:
```bash
cargo run check [FILE]
```
to use the checker. (it doesn't offer suggestions)
as an example, run:
```bash
cargo run check test.txt
```
there's also a help sub-command:
```bash
cargo run help
```
