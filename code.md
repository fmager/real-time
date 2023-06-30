## Installation

* [Install Rust](https://www.rust-lang.org/tools/install). A version of Rust supporting edition 2021 is needed.
* git clone this code
* In the command line write ```cargo run --release```. This might take a while.
* For IDE, I prefere VS Code with the extensions rust-analyzer, CodeLLDB, Rust Syntax,
WGSL, wgsl-analyzer and optionally Dracula For Rust Theme.

## Testing
On some computers the GPU tests will currently fail unless being run with ```cargo test -- --test-threads=1```
Even then it might fail. You can just try a few more times or try to run tests individually.
It is because of the queue and device being acquired several times.
This is likely to happen less on bigger GPU's.
