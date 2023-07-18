# Don't Panic!
A guide designed for both deep learners and systems programmers. Meant to be followed several times at
deepening levels.
The material is comprised of 6 modules.

* Intro to the course, the different ways to use the material, intro to Rust and wgpu.
* Memory hierarchies and computational graphs
* Parallelization, interactivity, events and GUIs
* Types, energy usage and inference, quantization, sparsity and pruning of neural networks
* Introduction to profiling, optimization use cases on topics such as model
training and quantization, graphics, computer vision
* How to create real time systems, frameworks for the different fields and project proposals

## TODO

* Try rust-nexttest to solve the testing issue
* Find the right benchmarking and performance tools (blessed.rs)
* Look into friendlier error handling? Perhaps logging instead of panicing to
get students used to logging. Introduce anyhow for better error handling?
* [Loom](https://docs.rs/loom/latest/loom/)
* Come up with a different name for levels 1/2/3/4, also should the levels be described in a matrix?
* Should there be an introduction to basic computer architecture somewhere?

## References and additional reading

[High Performance Machine Learning](https://engineering.nyu.edu/sites/default/files/2022-01/ECE_GY_9143_S22.pdf)  
[High Performance Machine Learning](https://www.cs.columbia.edu/wp-content/uploads/2022/08/HPML-Fall2022-columbia.pdf)  
[Flash Attention](https://github.com/HazyResearch/flash-attention)  
[Branchless Programming](https://www.youtube.com/watch?v=g-WPhYREFjk)  
[The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html)  
[Learn wgpu](https://sotrh.github.io/learn-wgpu/)  
[Install Rust](https://www.rust-lang.org/tools/install)  
[wgpu](https://wgpu.rs/)  
[ShaderToy](https://www.shadertoy.com/)  
[Inigo Quilez](https://iquilezles.org/articles/)  
[ORB-SLAM](https://arxiv.org/abs/1502.00956)  
[ORB-SLAM2](https://arxiv.org/abs/1610.06475)  
[Z-order curves](https://www.nocentino.com/Nocentino10.pdf)  
[Linearised Trees on the GPU](https://developer.nvidia.com/blog/thinking-parallel-part-iii-tree-construction-gpu/)  
[Vivienne Sze - Energy Efficient AI](https://www.youtube.com/watch?v=WbLQqPw_n88)  
[Visual Computing - Stanford](https://gfxcourses.stanford.edu/cs348k/spring23)  
[Parallel Computing - Stanford](https://gfxcourses.stanford.edu/cs149/fall21)  
[Rust Profiling](https://nnethercote.github.io/perf-book/profiling.html)  
[RenderDoc](https://renderdoc.org/)  
[Book of Shaders](https://thebookofshaders.com/)  
[Scratchapixel](https://www.scratchapixel.com/)  
[Ray Tracing in One Weekend](https://raytracing.github.io/)  
[Physically Based Rendering](https://www.pbrt.org/)  
[Crafting Interpreters](https://craftinginterpreters.com/)  
[Programming Rust](https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/)  
[Godbolt](https://godbolt.org/)  
[Advent of Code](https://adventofcode.com/)  
