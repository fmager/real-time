# 2️⃣ Intro to GPU's
GPU's are fairly ubiquitous at this point. They started off as purely for graphics, but
around 2008, enough researchers had tinkered with workarounds to use them for general
computing, that Nvidia put out CUDA, opening up GPU's for more general usage. GPU's still do lots
of graphis, but they are no longer opaque black boxes and even graphics API's such as OpenGL, Vulkan,
Metal and DirectX have opened up. With modern graphics API's you don't even necessarily need a graphics output
to use them. You can just use the pure compute capabilities. This guide won't get into graphics, except
for the graphics specialization.

Ok, so anyways, GPU's are pretty hot stuff right now as the software support becomes deeper and deeper,
the hardware increasingly has hardware support for neural network specific operations and ChatGPT has
increased the hype and demand for AI to exasperating levels.

You can think of the GPU as an expansion of the memory hierarchies we have been examining earlier.
It is not running in lockstep, and you have to program more things explicitly, while also changing
your mindset about how programming works. Memory transfers to and from the CPU and GPU will be
relatively explicit, you have explicit control of a part of the L1 cache, you have start programming
in a warp oriented fashion and if-statements become quite dangerous.

If the CPU, with its numerous cores is like a team of highly skilled specialists building a car, sure,
they can build an amazing car, they can adapt to changing circumstances quite well, they can act independently,
then the GPU is like a factory. Each path and process has to be carefully optimized, they might each only deal with
a very small piece each and people have to work in lockstep. But. Their throughput is unmatched.

At level 3️⃣ I will go into more detail as to how to actually write GPU code, but the guide is set up using
Rust and a GPU API abstraction layer called [wgpu](https://wgpu.rs/). You don't need to understand how it works
right now, but it means that you should be able to run all code, including GPU code, on your platform, even if
it's made by Apple or AMD. 

## GPU Hardware
First off, when dealing with the GPU, you will have to manipulate the GPU from the CPU with commands
like "allocate this much memory", "transfer this memory from the CPU to GPU", "execute this shader/kernel" and
"synchronize". These are all done in whatever language you are writing in on the CPU side, except for the
actual program the GPU has to run. This is distinct from the GPU API, some GPU API's even accept shaders
written in multiple shading languages, as they can either be transpiled (translated) from one language to
another, or they can be compiled to an intermediate representation, such as SPIR-V, which they can then ingest.

But once we have built up all of these commands, at least if they are non-blocking, as in the CPU program won't
advance until the command has completed, we have to actually submit them to the GPU. We do this with a
synchronization. The commands may/may not have already been submitted, but if you call a synchronization
function, the CPU-side code will block and wait until any and all submitted commands have executed on the GPU
and the GPU sends the all-clear signal in return. Imagine you are at a horse track. You have to give instructions
to a jockey on a race horse. You stand on the periphery of the big oval race track. You tell the jockey to make
some adjustment and do a lap. The horse first has to accelerate and then once it nears you slow down and you can
talk again. What would be more efficient was if you could leave notes for the jockey to pick up whenever he was coming around and the horse could just continue at speed. In some API's the GPU can just be set in motion and then
whenever you have a change to the loop it is running, adjust or change. Or you can set work in motion and come back
at a later time, whether the work might be done.

### Transfer
When transferring memory, you should have the following model in mind, nothing gets transferred without a staging
area. When transferring from CPU to GPU, at least in the CUDA programming model, it will pin an area in memory.
That memory won't be movable until it is unpinned. You basically transfer some memory from say, a vector you
want transferred to the GPU, to this pinned memory staging area. That pinned memory area means the GPU
can work in peace without interruptions. In CUDA, if you don't explicitly do it, CUDA will create a pinned memory
area and do it for you. If you do it yourself and optimize this process you are likely to see around 2x improvement
in transfer speed. The same thing happens on the GPU, a staging area visible from the CPU is where the transferred
memory is stored, and then moved from the controlled area to the rest of GPU memory, where the GPU is free to do
what it wants with it, without interruptions and guarantees.

### Threads, Warps and Blocks
Threads are sort of like a CPU core, except a CPU core is a physical entity, whereas a thread is more like
a set of variables (think back to the stack and function calls) which is following its own set of instructions.
Thread 1 could be running program A with various states in registers and local variables X. It makes a call
to something expensive, like a cache-missing memory access. While waiting, thread 1 is swapped for thread 2.
Its state is of course saved, but thread 2's program B and state Y are swapped in for it to do some work.
This keeps the CPU core itself occupied with work.

Threads on a GPU, will usually be executing the SAME program, unless several calls are overlapped, but let's
just focus on you having called a single operation. In that case all of your threads will launch, running
the same program. They might however, go down different branches (think if-statements!), but this more expensive
on the GPU and CPU, and should in general be avoided as much as possible. Each thread will have its own local
variables. Threads on a GPU are launched in groups. Depending on the platform and the API they will be
called something different. In wgpu, which is what we will be using, it is called a workgroup, while
in CUDA terminology it is called a warp. On Nvidia GPU's it will be at most 32 threads per workgroup
and on AMD it will be at most 64 threads. The "at most" might seem a bit weird, but there is something called
register pressure. All of the execution units that can run those 32 or 64 threads at the same time, share
a lot of the same physical memory, so if your program uses lots of memory, you might have to decrease
the amount of threads to have enough memory to run your program.

Anyways. Once you decided to write a matrix-matrix multiplication shader, you need to figure out which threads
are gonna go where. In that case, I would begin by launching 1 thread for every output element.

When programming for a GPU you have some maximum amount of threads you can launch. This is usually
defined in three dimensions. Yes! You can define these threads in three dimensions. It doesn't actually
have much of an effect, but it makes sense to tailor how you launch threads to your problem area.
If you are performing image processing or matrix multiplication, by all means, launch a 2D grid.
If you are summing an abitrary list of numbers, a single dimension will probably suffice.

So, we should launch a 2D grid, matching the output elements of our problem. Next up,
how do know which thread does what work? Each thread will usually begin its program
by asking built-in variables, which thread it is. This can be which thread it is within its
own workgroup, or it could be globally. Once it knows that, it should usually check whether
it is within legal bounds of the problem. We almost always want n^2 threads in our workgroup,
and it wouldn't be very flexible if the problem size always had to match exactly.
So usually, you should launch too many threads and then have an if-statement following
the thread ID calculation. If within acceptable range, do work, otherwise, don't do work.

It cannot be assumed that all work groups are running concurrently. The GPU might need to launch
waves of work groups because there aren't enough physical execution units.
As such, we can only synchronize between threads inside the warp.

### GPU Memory Hierarchy
The memory hierarchy on a GPU, here as exemplified by the Nvidia H100, which is a very expensive data center GPU
and most definitely not the one residing in your laptop, looks a lot like the memory hierarchy on the CPU.
But the bandwidth (how much data per second) internally on the card is a lot higher than on the CPU. All of
the streaming multiprocessors share the L2 cache and each streaming multiprocessor shares an L1 cache. On
Nvidia GPU's the streaming multiprocessor is a number of, in this case 4, units which can each execute a
work group, or in Nvidia terminology, a warp.

<figure markdown>
![Image](../figures/Full-H100-GPU-with-144-SMs-1024x457.png){ width="700" }
<figcaption>
The layout of a H100 GPU. Note that connectivity to the memory (HBM3) is on the left and right sides.
<a href="https://developer.nvidia.com/blog/nvidia-hopper-architecture-in-depth/">
Image credit </a>
</figcaption>
</figure>

Take some time to study these two diagrams and think about how data moves first from the CPU,
to the GPU's main memory, then to the L2 cache, then to what streaming multiprocessor which needs its L1 cache
until it finally is loaded up into the registers of the 32x4 threads executing on different, but adjacent, segments
of the same data.

<figure markdown>
![Image](../figures/H100-Streaming-Multiprocessor-SM-625x869.png){ width="400" }
<figcaption>
The layout of a single Streaming Multiprocessor. It can execute 4 work groups or warps at a time.
<a href="https://developer.nvidia.com/blog/nvidia-hopper-architecture-in-depth/">
Image credit </a>
</figcaption>
</figure>

The threads accumulate their data into their own registers until they are done and write the
result to main memory. The CPU waits for the GPU to be finished, until the GPU is, transfers to the CPU and
signals that it is finished.

It's not always as clear cut, though. If you are using a laptop, you probably have an integrated graphics card.
The CPU and GPU coexist and share the same memory. There may be sections where there is higher bandwidth than
just normal CPU-based memory, but overall the integrated GPU has access to the same memory the CPU has.
This makes for faster transfers, but probably slower overall computation. This has become quite useful
recently with most consumer grade GPU's having around 8 GB of memory and locally run neural networks
like diffusion models easily being able to use more than that. A desktop GPU with more than 16GB of RAM would
probably still outperform an integrated graphics card with 16GB of RAM available, but it would be very expensive.

## 3️⃣ Introducing wgpu

## 3️⃣ Setting up wgpu
Imported how in rust
What is the initial setup of a compute pipeline
[Learn Wgpu](https://sotrh.github.io/learn-wgpu/).

## 3️⃣ Basic GPU Programming
API and Shader language are not the same
Host side
Device side
Remove the loop where?

## 3️⃣ Warp Divergence, Occupancy and Overlap

## 3️⃣ Coalesced Accessing and Strides

## 3️⃣ Synchronization
A small code sample from wgsl

## 3️⃣ Shared memory
A small code sample from wgsl

## 5️⃣ Further Reading
[The GPU Memory Hierarchy](https://www.cs.cmu.edu/afs/cs/academic/class/15869-f11/www/lectures/08_mem_hierarchy.pdf),
[GPU Memory Hierarchy](http://meseec.ce.rit.edu/551-projects/spring2015/3-2.pdf),
[GPU Programming](http://dlsys.cs.washington.edu/pdf/lecture5.pdf),
[Hopper Architecture In-Depth](https://developer.nvidia.com/blog/nvidia-hopper-architecture-in-depth/)
and [GPU architecture and CUDA Programming](https://gfxcourses.stanford.edu/cs149/fall22/lecture/gpuarch/).
The last entry is highly recommended.

A slightly more detailed explanation of
[asynchronous memory transfers](https://engineering.purdue.edu/~smidkiff/ece563/NVidiaGPUTeachingToolkit/Mod14DataXfer/Mod14DataXfer.pdf)
for GPUs.