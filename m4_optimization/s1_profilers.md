# 3️⃣ Profilers
Hard to install and something that depends on your use case.  

## Choosing a Profiler

* web
* Task manager
* perf
* [RenderDoc](https://renderdoc.org/)
* [AMD Radeon GPU Profiler](https://gpuopen.com/rgp/)
* [Nvidia Visual Profiler](https://developer.nvidia.com/nvidia-visual-profiler)
* [Nsight Systems](https://developer.nvidia.com/nsight-systems)
* [PyTorch profiler](https://pytorch.org/tutorials/recipes/recipes/profiler_recipe.html)
* [A list of Rust profilers from the Rust Performance Book](https://nnethercote.github.io/perf-book/profiling.html)
Profiling on Windows is a bad experience if you are outside Visual Studio. You can try using Linux tools like perf
through WSL2 on Windows.

## Bottlenecks

* Disk bound
* Transfer bound
* Memory bound
* Compute bound

## Different Points of View
Benchmark table  
[Trace viewer](https://github.com/abhirag/tracy_rust_demo)  
Flamegraph  
Hot Spots  

## How to work with Profilers
Make sure you measured enough
Are there any glaringly obvious things wrong? Did you remember to turn on the GPU?
If you have any ideas for low hanging fruit, see how much they shake up the profiling landscape, maybe it suddenly
becomes compute bound instead of memory bound?
If you are doing something fairly standard, there might be optimization checklists available to give you ideas
for optimizations.

## Hardware

* Cache hits and misses
* CPU/GPU interactions
