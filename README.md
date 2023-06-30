# The Real-Timers Guide to the Computational Galaxy
A guide designed for both deep learners and systems programmers. Meant to be followed several times at deepening levels. The material is comprised of 6 modules.
* Intro to the course, the different ways to use the material, intro to Rust and wgpu.
* Memory hierarchies and computational graphs
* Parallelization, interactivity, events and GUIs
* Types, energy usage and inference, quantization, sparsity and pruning of neural networks
* Introduction to profiling, optimization use cases on topics such as model training and quantization, graphics, computer vision
* How to create real time systems, frameworks for the different fields and project proposals


## TODO
* Try rust-nexttest to solve the testing issue
* Find the right benchmarking and performance tools (blessed.rs)
* Look into friendlier error handling? Perhaps logging instead of panicing to get students used to logging. Introduce anyhow for better error handling?
* [Loom](https://docs.rs/loom/latest/loom/)





## Installation
* [Install Rust](https://www.rust-lang.org/tools/install). A version of Rust supporting edition 2021 is needed.
* git clone this code
* In the command line write ```cargo run --release```. This might take a while.
* For IDE, I prefere VS Code with the extensions rust-analyzer, CodeLLDB, Rust Syntax, WGSL, wgsl-analyzer and optionally Dracula For Rust Theme.

## Testing
On some computers the GPU tests will currently fail unless being run with ```cargo test -- --test-threads=1```
Even then it might fail. You can just try a few more times or try to run tests individually. It is because of the queue and device being acquired several times.
This is likely to happen less on bigger GPU's.

## Modules

### Intro
<details>

* Why should you use this material
* How to use the materials as a student and a teacher.
* Which level is for who and what does it require
#### Intro to Rust
* Project setup
* How to compile
* What are types
* Borrow checker - shared and mutable references
* Move, Copy and Clone
* Iterators
* (Iterators - slightly more detailed)
* Option, Result - everywhere
* Enums & Match statements
* (Smart pointers)
* (Traits)
* (Clippy)
* (fmt)
</details>

### Memory Hierarchies and Computational Graphs
<details>

#### Intro and Getting Started
* Intro to the code framework
* Compilers
* Frequent commands and FAQ
#### The CPU and memory hierarchies
* Intro to memory hierarchies
* (Virtualized memory)
* Pointers, Heap and Stack, Dynamic Arrays
* The CPU-side memory hierarchies
* Pipelines and branch prediction
* Inlining
* (Pointers and smart pointers)
* (Garbage collectors)
#### Computational Graphs (Low code)
* Intro to computational graphs - overview of immediate, graph and compiled graph
* The network we want to support
* What's in a tensor
* Data dependencies and control dependencies
* (Intermediate representations)
* (Compiler verifications and the restrict keyword)
* Testing the correctness of the nodes
* (Graph representations)
* (Perspective to render graphs)
#### Intro to GPU's
* Brief intro to GPU's
* (Shared memory, warp shuffling and distributed shared memory)
* (Synchronization)
* (SPIR-V & GLSL/HLSL)
* Intro to wgpu
* (Setup of wgpu)
#### Immediate GPU computation
* Building the first compute node
* GPU's in greater detail
* Pipelining, Warp Divergence, Occupancy and Overlap
* Building the remaining compute nodes
* Testing the whole thing in immediate mode
* (Caching shaders)
#### Building a Computational Graph
* Seeing the CPU-GPU memory hierarchies
* Transfers
* Building a computational graph
* Testing the computational graph
#### Building a Computational Graph Compiler
* Seeing the GPU memory hierarchy - caches, shared memory and RAM
* Graph compilers and OP codes
* Swapping operators for fused versions
* Building a graph compiler
* Testing the graph compiler
* (Metaprogramming - Shaders are just strings!)
* (Decomposing to OP codes)
* (A toy example with OP codes)
* (Additional ideas for compiler optimization, buffer reusage, matrix reusage)
#### Closing Remarks
* Comparing CPU, immediate, immediate with shader caching, computational graph and compiled computational graph.
* How does this relate to torch.compile?
* Where to go from here?
#### (Exercises - do at least 1)
* (Implement a version of the linear layer functions which uses shared memory and tiling)
* (Implement the tree reduction version of the sum function and add it to the softmax function. Also compare the single pass and the tree reduction performance graphs. [Reference](https://developer.download.nvidia.com/assets/cuda/files/reduction.pdf))
* (Implement a max pooling operator in all levels and implement tests)
* (Implement a convolution operator in all levels and implement tests)
* (Add reusable buffers to the computational graph system)
* (Extend the computational graph with inplace operation for the ReLU operator)
</details>

### Parallelism, interactivity, events and GUIs
<details>

* Data parallelism, work stealing - rayon
* Data parallelism, non-work stealing - crossbeam
* Mutex
* Async
* Atomic
* Threads
* GPU
* (Sparsity)
* (Random Access and Monte Carlo (Gyro Dropout))
* (Branchless programming)
* (SIMD)
* (Sorting)
* Channels
* Events
* Key and Mouse events
* Event Loops
* (GUIs & egui)
* (Examine egui-winit-wgpu template)
* (Graph representations - pointers and indices)
* (Trees using indices)
* (Parallel work on graphs)
#### (Specializations/Exercise - Pick items worth a total of 3 points or more, write a 10+ lines interpretation of each item)
* (1 - Data-oriented design - Entity component systems)
* (1 - Array of Structs, Structs of Arrays, Auto-Vectorization)
* (1 - Linearized octrees)
* (2 - Sorting kernels in divergent workloads - Wavefront path tracing)
* (4 - ORB-SLAM - design and a warning about trying to code it)
* (4 - Nanite)
* (1 - PyTorch - Data-Distributed-Parallelism)
* (1 - PyTorch - Model-Distributed-Parallelism)
* (2 - Shadertoy)
* (1 - Gyro Dropout - MLSys 2022)
* (1 - Hierarchical Frustum Culling)
* (2 - Flash Attention)
* (2 - Custom memory allocators)
* (2 - [JAX](https://jax.readthedocs.io/en/latest/notebooks/Common_Gotchas_in_JAX.html))

#### (Exercise)
* Describe the base architecture of the egui-winit-wgpu template. Expand on the template and program some things (needs suggestions) using some of the primitives introduced in the module
</details>


### Types, energy usage and inference, quantization, sparsity and pruning of neural networks
<details>


* Floats
* Float precision
* (Fast inverse square root)
* Integers
* (Bit tricks)
* (Basic compression)
* Energy usage
* (Batch based data processing)
* Inference
* Quantization
* Sparsity
* Pruning
* (Tensor Cores)
* (Using integers instead of strings in hash tables)
#### (Specializations - Group discussion and presentation)
* (Packing bits for atomic operators)
* (Inverse depth buffers)
* (Bittricks, packing normals and colors)
* (Morton codes / Z-order curves, tiling and GPU textures)
* (Calculating compression precision in a lossy point cloud compression scheme)
* (DLSS)
* (Real-Time Texture Decompression and Upsampling)
* (2:4 sparsity with Tensor Cores)


#### (Exercise)
* (Find a suitable model and inference library. Perform inference. Optimize the model and inference process. Can you do inference on one thread, training on another and substitute in the new model? ADD SUGGESTED MODELS)

</details>

### Introduction to profiling, Optimization use cases
<details>

* Profilers (PyTorch, web, GPU, general)
#### Specializations
* Training a neural network
* Optimizing a neural network for inference
* Running Yolo
* Optimizing a point cloud renderer
* Optimizing a path tracer

#### (Exercise)
* (Try out the profilers relevant to your own system with some sample programs.)
</details>

### How to create real time systems, good frameworks for the different fields and project proposals
<details>

* Starting with a simple prototype
* Identify your components
* Single threaded correct implementation -> Testing to avoid regression
* Optimize
#### Tips and tricks in real time systems
* (memcpy)
* (Hot loops, event loops)
* (Allocations in a hot loop)
* (System calls - hoist out of the hot loop)
* (Logging and printing)
* (Bindings - PyO3 and cxx)
* (Walk, don't run, testing for correctness before optimization)
* (Don't use abbreviations)
* (Don't use postfix incrementation++)
* (When to care about software engineering and when to care about performance)
* (Don't use a string key/identifier or integer, when a type safe enum will do the job)
* (Hard coding types)
* (Cognitive load, and delaying errors to after the first draft - deliberate development vs. debugging)
* (Prefer stateless programming, minimize stateful programming (functional inspiration))
* (Implicit casting)
* (Compression)
* (Know your system - mobile, laptop, desktop, integrated memory, which GPU)
* (Use version control even for solo development)
* (Am I copying/cloning things that don't need to be copied?)
* (Check/validate everything before the hot loop)
* (Anything that can be immutable, should be immutable - aliasing!)
* (Testing and Seeding RNG's)
* (Timing real-time systems and how to escape or offload compute)
#### Components - libraries/frameworks
[blessed](blessed.rs)  
[rayon](https://github.com/rayon-rs/rayon)  
[egui](https://github.com/emilk/egui)  
[wonnx](https://github.com/webonnx/wonnx)  
[tch](https://github.com/LaurentMazare/tch-rs)  
[winit](https://github.com/rust-windowing/winit)  
[cv](https://github.com/rust-cv/cv)  
[ultraviolet](https://github.com/fu5ha/ultraviolet)  
[arewelearningyet](https://www.arewelearningyet.com/neural-networks/)  
[burn](https://github.com/burn-rs/burn)  

#### Specializations - Project proposals
Virtual 3D scanner for a point cloud dataset  
EEG system  
Change the latent variables in a network using GUI, optimize the network  
Point cloud renderer  
Real-time style transfer on a web cam feed  
Rendering fractals influenced by a web cam feed  
Eye tracking -> Present to screen and read from web cam -> feature extraction -> classifier -> intervention signal -> reading app (Wolfgang Fuhl, PISTOL, fixation detection)  
Bird classification from sound / Real-time classification of sound (Xeno-canto database)  
Who is talking? Real-time classification of sound  
Are you dyslexic? Eye tracking classifier  
Cognitive load tracker - Eyes & pupil dilation and online estimation of signal strength (pupils vs. sound for the hearing impaired)  



#### What makes for a good project?
* What is your concept/project?
* Which concepts from the previous material do you think are relevant to your project and why?
* Preprocessing your data?
* How do you adapt to your chosen/available platform?
* Which libraries did you choose for this problem?
* How fast did you get to your minimum viable product?
* Which steps did you take from there and why?
* How did you determine which parts of your system to optimize?
* What else would you like to do with your system?
</details>


## References and additional reading
<details>

[Introduction to High Performance Machine Learning](https://engineering.nyu.edu/sites/default/files/2022-01/ECE_GY_9143_S22.pdf)  
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
</details>