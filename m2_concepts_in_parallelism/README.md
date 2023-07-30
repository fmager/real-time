# Concepts in Parallelism

* Threads
* Async
* Events
* Mutex
* Atomic
* Channels
* Data parallelism, work stealing - rayon
* Data parallelism, non-work stealing - crossbeam
* 3️⃣ GPU - 3️⃣ Warp Shuffling and Distributed Shared Memory 3️⃣ SPIR-V & GLSL/HLSL
* 3️⃣ Branchless programming, branch prediction and pipelines
* 3️⃣ SIMD
* 3️⃣ Sparsity
* 3️⃣ Random Access and Monte Carlo (Gyro Dropout)
* 3️⃣ Sorting
* 3️⃣ Graph representations - pointers and indices
* 3️⃣ Trees using indices
* 3️⃣ Parallel work on graphs

# 4️⃣ Exercise
Describe the base architecture of the egui-winit-wgpu template.  
Expand on the template and program some things (needs suggestions)  
using some of the primitives introduced in the module

# 🧬4️⃣ Exercise
Pick items worth a total of 3 points or more, write am interpretation of each
item of at least 10 times the number of points lines.

* 1 - Data-oriented design - Entity component systems
* 1 - Array of Structs, Structs of Arrays, Auto-Vectorization
* 1 - Linearized octrees
* 2 - Sorting kernels in divergent workloads - Wavefront path tracing
* 4 - ORB-SLAM - design and a warning about trying to code it
* 4 - Nanite
* 1 - PyTorch - Data-Distributed-Parallelism
* 1 - PyTorch - Model-Distributed-Parallelism
* 1 - PyTorch - [Optimizing inference](https://pytorch.org/blog/optimizing-libtorch/?hss_channel=lcp-78618366)
* 2 - Shadertoy
* 1 - Gyro Dropout - MLSys 2022
* 1 - Hierarchical Frustum Culling
* 1 - [SIMD optimization](https://ipthomas.com/blog/2023/07/n-times-faster-than-c-where-n-128/)
* 2 - Flash Attention
* 2 - Custom memory allocators
* 2 - [JAX](https://jax.readthedocs.io/en/latest/notebooks/Common_Gotchas_in_JAX.html)
* 1 - [Branch Prediction](https://stackoverflow.com/questions/11227809/why-is-processing-a-sorted-array-faster-than-processing-an-unsorted-array)
