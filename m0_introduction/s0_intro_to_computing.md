# Introduction to the Computing Landscape
If you are new to programming, or perhaps have been able to get by using scripting languages only,
you might not have been introduced to the other options. Some of the concepts presented here
lay the foundations for the choices dictating the rest of the guide.
Though the guide has made some clearly defined choices about which tools to use, you should at all times
use the right tool for the job. Not only in which language or framework you might choose,
but in how you put together and design your systems using those tools. Part of the guide's
strategy is to introduce you to quite a lot of tools and concepts, also known as the learn
what to Google strategy, and then going into greater detail about core concepts and concepts
especially relevant to your specialization.
The guide will introduce concepts that aid some programs in producing faster and results than others.
An important factor is limitations. Usually, the word limitations carries a negative connotation,
very few people think less freedom sounds enticing, but in computing limitations can be a wonderful
thing to have and set. Especially, once you are past the first prototype. In some cases, even when prototyping.

## Scripting Languages
Chill out, take things one line of code at a time.  
Scripting languages aren't compiled, but run one line at a time.
This leaves the system unable to look beyond the current line of code,
unless you add a compiler to the mix, whic usually takes a look at all of your code.

[Python](https://en.wikipedia.org/wiki/Python_(programming_language)) is likely the
scripting language you are most familiar with.  
Very popular due to its apparent ease-of-use. Quite slow in its raw form.  
The main advantage of vanilla python is the ability to glue together a number of libraries  
written in other languages. In time, improvements have been made, such as putting type hints  
into your code SUCH AS, which helps catch errors and gives more informative function definitions.

In general, if other people reading your code must read the high-quality comments,
that you definitely remembered to write... right?, then you are laying the foundation of a
codebase that will frustrate people, probably including yourself.
Python is easy to write at first, but the lack of a compiler can leave your long running code to fail
just at the moment it is about to save the results because your forgot you were trying to save the  
wrong type.

Python does have additional tools you can use to compile it. This allows for additional verification
and performance improvements, but without additional limitations and indications of your intention,
it might not be possible to optimize your code as much as a language which leaves things less
open to interpretation.

## Compilers  
A compiler processes the given code in one or more steps.
In some steps it might verify that all of your code is correct, it might perform transform and optimize your code,
it might change it into different representations like byte code or machine code. Some compilers strictly function
ahead-of-time in an operation like ```some_compiler -compile my_file.code``` and output a runnable executable,
specifically for your type of machine.
Most compilers require additional constraints to transform and improve your code.

Imagine you ask someone to go get you some milk every Thursday at 12.
An unreasonably pedantic person (engineer) might be ready at 12 every Thursday and ask you what type of milk
you would like today.
It seems annoying and strange. You know what type of milk you like, the pedantic person should know what
type of milk you like. That bastard! If you instead asked for X brand skim milk delivered at 12 every Thursday,
the pedantic person might even try to optimize the process before the delivery day.
If it was milk with a long expiration day, they could buy it in bulk and just have it ready for you.
That unreasonably pedantic person is the compiler of whatever programming language you are using.
It will go far to help you, it just doesn't perform well in ambivalent circumstances.
Compilers are genereally not allowed to guess in a way that might functionally alter your code, such  
as reducing the level of precision.

The languages in the compiled languages are all designed with at least one compiler, usually  
compiling to byte code or machine code.  
However, it is possible to write a compiler after the fact. [Cython](https://cython.org/) is one such compiler.  
It benefits quite a bit from having the user perform additional annotations of their python code,  
allowing for a decent speedup.

Other compilers act Just-In-Time (JIT). Just as you want to run your code it will compile it.
While this seems a bit weird, why not just compile it once and for all, this can allow the compiler to
optimize the program specifically for your machine. The
[Java HotSpot VM](https://docs.oracle.com/en/java/javase/17/vm/java-virtual-machine-technology-overview.html#GUID-982B244A-9B01-479A-8651-CB6475019281)
even tries to optimize your code as it runs. If allowed to become a long-running process it can
swap byte code for compiled machine code. In general, JIT compilers increase the startup time
of your code, afterall, it has to compile it, just like the AOT compiler. Some JIT compilers
save the compilation artifacts (the outputs of the compilation process) for later to merely
reload it, but that won't help you much while you are developing your code. Some libraries and
frameworks such as [numba](https://en.wikipedia.org/wiki/Numba) perform JIT compilation of your
annotated code to optimize the performance.

## Compiled Languages
In some languages like [C](https://en.wikipedia.org/wiki/C_(programming_language)),
[C++](https://en.wikipedia.org/wiki/C%2B%2B) and
[Rust](https://en.wikipedia.org/wiki/Rust_(programming_language)), machine code is the outcome.
That machine code can be quite platform specific, both because of the operating system and the
hardware, and is binary. 1's and 0's!
These three languages are not garbage collected (more on that later).

Another quite popular language is [Go](https://en.wikipedia.org/wiki/Go_(programming_language)),
which also compiles to machine code, but is garbage collected.
[Julia](https://en.wikipedia.org/wiki/Julia_(programming_language)) has more of a scientific/numerical
focus, but features garbage collection, JIT compilation and can use either a runtime or compile to
a standalone binary.

Other languages like [Java](https://en.wikipedia.org/wiki/Java_%28programming_language%29) and
[C#](https://en.wikipedia.org/wiki/C_Sharp_(programming_language)) compile to something called bytecode,
which can then be interpreted by a process virtual machine. Thus all Java programs compile to the same
bytecode, regardless of whether it's supposed to run on a Mac or Windows platform.
The bytecode is then interpreted, sometimes optimized as well,
at runtime by a virtual machine written for that specific platform.

[Javascript](https://en.wikipedia.org/wiki/JavaScript) is a just-in-time compiled language running on most web pages.  
It can occassionally have a reasonable speed due to the optimizing runtime.  
Heavy development has tuned the widely used V8 runtime to improve Javascripts performance.  
Writing Javascript can seem easy, but the amount of things you are allowed to do,  
but shouldn't, can make it an arduous experience once the errors start piling up.  
The advantage is highly portable code, because everything is essentially just a string... including numbers.  

## The Guide and Languages
As you can probably see in the column on the left... the guide will be using Rust from here on out.
If you read the section on GPU programming, you will see there are no easy, one-size-fits-all,
solutions. Thankfully, the guide has clear goals and limitations.
To help you get familiar with new topics, we only need reasonable performance and for all
the code to be runnable on most laptops.  
After all, it's not much fun playing around with things on someone else's computer.  
Most importantly, the setup process should be easy and not make you want to stress-eat
the contents of your entire fridge when going through the installation process.
As such the guide will mainly use Rust and the GPU API wgpu. The guide will in all cases that do not require
graphics output only concern itself with pure computation through wgpu, which makes setup quite a bit simpler.
wgpu is an abstraction layer that runs whatever GPU API it finds best suitable on your system. Having exact
control and the absolute best performance isn't as important for the guide as allowing as many people to
participate and learn as possible. After all, if it doesn't work on your laptop/desktop, you can't really
play around and have fun with it!

## \*GPU Specific Languages and APIs
CUDA/OpenCL
Older APIs
Web languages
Modern APIs (D12, Metal, Vulkan, WebGPU(wgpu))

### CUDA/OpenCL
[CUDA](https://en.wikipedia.org/wiki/CUDA)
[OpenCL](https://en.wikipedia.org/wiki/OpenCL)
Haha! Surprise! CUDA is actually programmed in C++ with some additional functions and decorators available.

### WebGL/WebGPU/wgpu
[WebGL](https://en.wikipedia.org/wiki/WebGL)
[WebGPU](https://en.wikipedia.org/wiki/WebGPU)

### DirectX11/DirectX12/Metal
Platform specific stuff
[DirectX11](https://en.wikipedia.org/wiki/DirectX#DirectX_11)
[DirectX12](https://en.wikipedia.org/wiki/DirectX#DirectX_12)
[Metal](https://en.wikipedia.org/wiki/Metal_(API))

### OpenGL/Vulkan
[OpenGL](https://en.wikipedia.org/wiki/OpenGL)
[Vulkan](https://en.wikipedia.org/wiki/Vulkan)
[wgpu](https://wgpu.rs/)

### GLSL/HLSL/WGSL
[GLSL](https://en.wikipedia.org/wiki/OpenGL_Shading_Language)
[HLSL](https://en.wikipedia.org/wiki/High-Level_Shader_Language)
[WGSL](https://en.wikipedia.org/wiki/Shading_language#WebGPU_Shading_Language)
Graphics and compute code. Specifically for the
Can be compiled to SPIR-V, an intermediate representation, sort of like the byte code we discussed earlier.  
This allows the platform independent SPIR-V to be translated to the specific instructions the GPU
the code is actually run on.

## \*Domain Specific Languages and Frameworks
We can take this concept of setting limitations even further  
Including GPU programming

### Pytorch
[Pytorch](https://pytorch.org/)  
Has its own compiler from [2.0](https://pytorch.org/get-started/pytorch-2.0/).  

### Taichi
[taichi](https://www.taichi-lang.org/)

### Halide
[Halide](https://halide-lang.org/)

### Futhark
[Futhark](https://futhark-lang.org/)
