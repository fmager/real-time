# Introduction to the Computing Landscape
What are programs.  
How do we use them.  
Choosing the right tools for the job.  
Why do some programs produce faster and safer code than others.  
Limitations, while usually carrying a negative connotation, is a wonderful thing to have and set in computing.  
Will be briefly introducing some languages and concepts.  
By no means an expert, but planting seeds for later.  

## Scripting Languages
Chill out, take things one line of code at a time.  
We are left without the ability to look beyond the current line of code.  
Easy to write things at first, but the lack of a compiler can leave your long running code to fail  
just at the moment it is about to save the results because your forgot you were trying to save the  
wrong type.  

### Python  
The one you are most likely to be familiar with.  
Very popular due to its apparent ease-of-use.  Quite slow in its raw form.  
The main advantage of vanilla python is the ability to glue together a number of libraries  
written in other languages. In time, improvements have been made, such as putting type hints  
into your code SUCH AS, which helps catch errors and gives more informative function definitions.  
In general, if other people reading your code must read the high-quality comments,
that you definitely remembered to write... right?, then you are laying the foundation of a
codebase that will frustrate people, probably including yourself.

### Javascript
The one running on most web pages.  
Can occassionally have a reasonable speed due to the optimizing runtime.  
Heavy development has tuned the V8 runtime.  
Writing Javascript can seem easy, but the amount of things you are allowed to do,  
but shouldn't can make it an arduous experience, once the errors start piling up.  
The advantage is highly portable code, because everything is essentially just a string... including numbers.
The guide won't concern itself too much with Javascript.  

## Compiled Languages
Java, Go, C#, C, C++, Rust

### Compilers  
Most of them require additional constraints to transform and improve your code.  
They are not allowed to guess in a way that might functionally alter your code, such  
as reducing the level of precision.

#### Ahead-of-Time (AOT)
The languages in the compiled languages are all designed with at least one compiler, usually  
compiling to byte code or machine code.  
However, it is possible to write a compiler after the fact. [Cython](https://cython.org/) is one such compiler.  
It benefits quite a bit by having the user perform additional annotations of their python code,  
allowing for a decent speedup.  

#### Just-in-Time (JIT)  
Start-up time, compilation artifacts may be saved
If allowed to become a long-running process it may optimize the code while running  
and substitute the function for a new optimized version.
Numba  
Java  

## \*GPU Specific Languages and APIs
CUDA/OpenCL
Older APIs
Web languages
Modern APIs (D12, Metal, Vulkan, WebGPU(wgpu))

### CUDA
Haha! Surprise! CUDA is actually programmed in C++ with some additional functions and decorators available.

### OpenGL

### WebGL

### DirectX11/DirectX12/Metal
Platform specific stuff

### GLSL/HLSL/WGSL
Graphics and compute code. Specifically for the
Can be compiled to SPIR-V, an intermediate representation, sort of like the byte code we discussed earlier.  
This allows the platform independent SPIR-V to be translated to the specific instructions the GPU
the code is actually run on.

## Domain Specific Languages and Frameworks
We can take this concept of setting limitations even further  
Including GPU programming

### Pytorch
[Pytorch](https://pytorch.org/)  
Has its own compiler from [2.0](https://pytorch.org/get-started/pytorch-2.0/).  

### \*Taichi
[taichi](https://www.taichi-lang.org/)

### \*Halide
[Halide](https://halide-lang.org/)

### \*Futhark
[Futhark](https://futhark-lang.org/)

## The guide and languages
As you can probably see in the column on the left... the guide will be using Rust from here on out.
If you read the section on GPU programming, you will see there are no easy, one-size-fits-all,
solutions. Thankfully, the guide has clear goals and limitations.
To help you get familiar with new topics, we only need reasonable performance and for all
the code to be runnable on most laptops.  
After all, it's not much fun playing around with things on someone else's computer.  
Most importantly, the setup process should be easy and not make you want to stress-eat
the contents of your entire fridge when going through the installation process. 
As such the guide will mainly use Rust and wgpu.
