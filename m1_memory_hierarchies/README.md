# Memory Hierarchies, Computational Graphs and Compilers
## Intro and Getting Started

* Intro to the code framework
* Compilers
* Frequent commands and FAQ

## The CPU and memory hierarchies

* Intro to memory hierarchies
* (Virtualized memory)
* Pointers, Heap and Stack, Dynamic Arrays
* The CPU-side memory hierarchies
* Pipelines and branch prediction
* Inlining
* (Pointers and smart pointers)
* (Garbage collectors)

## Computational Graphs (Low code)

* Intro to computational graphs - overview of immediate, graph and compiled graph
* The network we want to support
* What's in a tensor
* Data dependencies and control dependencies
* (Intermediate representations)
* (Compiler verifications and the restrict keyword)
* Testing the correctness of the nodes
* (Graph representations)
* (Perspective to render graphs)

## Intro to GPU's

* Brief intro to GPU's
* (Shared memory, warp shuffling and distributed shared memory)
* (Synchronization)
* (SPIR-V & GLSL/HLSL)
* Intro to wgpu
* (Setup of wgpu)

## Immediate GPU computation

* Building the first compute node
* GPU's in greater detail
* Pipelining, Warp Divergence, Occupancy and Overlap
* Building the remaining compute nodes
* Testing the whole thing in immediate mode
* (Caching shaders)

## Building a Computational Graph

* Seeing the CPU-GPU memory hierarchies
* Transfers
* Building a computational graph
* Testing the computational graph

## Building a Computational Graph Compiler

* Seeing the GPU memory hierarchy - caches, shared memory and RAM
* Graph compilers and OP codes
* Swapping operators for fused versions
* Building a graph compiler
* Testing the graph compiler
* (Metaprogramming - Shaders are just strings!)
* (Decomposing to OP codes)
* (A toy example with OP codes)
* (Additional ideas for compiler optimization, buffer reusage, matrix reusage)

## Closing Remarks

* Comparing CPU, immediate, immediate with shader caching, computational graph and compiled computational graph.
* How does this relate to torch.compile?
* Where to go from here?

## (Exercises - do at least 1)

* (Implement a version of the linear layer functions which uses shared memory and tiling)
* (Implement the tree reduction version of the sum function and add it to the softmax function.
Also compare the single pass and the tree reduction performance graphs. [Reference](https://developer.download.nvidia.com/assets/cuda/files/reduction.pdf))
* (Implement a max pooling operator in all levels and implement tests)
* (Implement a convolution operator in all levels and implement tests)
* (Add reusable buffers to the computational graph system)
* (Extend the computational graph with inplace operation for the ReLU operator)
