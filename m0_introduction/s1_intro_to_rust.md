# Introducing Rust
Why Rust?  
Rust has specifically been chosen as the guide needed a systems level programming
language which was easy to setup and use on Windows, Linux and macOS.  
The setup time needed to be less than 10 minutes and the chosen language needed an
easy-to-use preferably integrated package manager.  
The options considered were C, C++ and Rust. C and C++ contained too many footguns.  
The guide was not supposed to be a guide to learning either of those languages.  
Rust's very helpful compiler is likely to be a boon for guiding users towards sensible code.  
The process of having to program in such a memory focused, modern, compiled language
will turn what is otherwise an implicit, unspoken process inside out,
forcing the user to think about what good code is, where is my memory,
which elements has access, and so on.

# Setup

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

# Projects
## Project setup
## How to compile
## Frequent commands and FAQ
## \*Clippy
## \*fmt
