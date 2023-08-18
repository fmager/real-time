# 1️⃣ Parallelism
Ok, so in the past module we looked at parallelism in the form of GPU-parallelism. In many ways, I find it
to be an easier introduction to the concept of parallelism. We introduce parallelism in small pockets inside
a function which cannot do anything too complicated. The programs aren't long running and we choose a
specific subset of problems to use the GPU for. In this module, I'll mainly introduce you to CPU based
parallelism with different mechanisms. In creating longer running CPU-based parallel programs you will
likely need to combine a bunch of these mechanisms along with your accrued knowledge of data races,
as enforced by the borrow checker in Rust. Additionally, I will introduce a few more concepts in GPU
programming in 3️⃣.

Anyways, why do we need parallelism in CPU's? Eventually, the clock frequencies, as in how many times
per second a processor can do something, more or less flattened out. We get increased performance by
either doing things in a smarter way or by increasing the amount of processors, either through
a massive amount of parallelism in an accelerator, such as a GPU or through adding more processors.

But parallel programming and parallel-friendly algorithms put a much greater cognitive strain on
you, the programmer. The more you learn about parallel programming, the more you will see that
the basic components are actually quite simple. The strain lies in thinking about
parallelism and who owns what memory at which time. This is critical in not just getting
faster programs, but retaining the correctness of your program from before you started parallelizing it.

## Algorithms and Systems Design

<figure markdown>
![Image](../figures/AmdahlsLaw.svg){ width="500" }
<figcaption>
Amdahl's Law
<a href="https://en.wikipedia.org/wiki/Amdahl%27s_law">
Image credit</a>.
</figcaption>
</figure>

[Amdahl's Law](https://en.wikipedia.org/wiki/Amdahl%27s_law) is a fundamental concept in parallelism.
Skim the link, but the concept is very simple. If 90% of your program is infinitely parallelizable,
you will still be left with a runtime of 10% of the original runtime - if you take parallelization to
the absolute limit. But how do you actually gauge which parts of your system are parallelizable?
The answer is quite frustrating.

*It depends.*

It depends on what type of algorithms are in play in your system, what sort of hardware platform
you are running on, it depends on what amount of development time and skill you have available.
Sometimes when you think about optimizing your code you might visualize it as explosions and
speed, flamethrowers and excess!

<figure markdown>
![Image](../figures/doof-warrior-from-mad-max.png){ width="600" }
<figcaption>
Witness Parallelism!
<a href="https://www.classicfm.com/discover-music/musicians-battle/doof-warrior-mad-max/">
Image credit</a>.
</figcaption>
</figure>

But in actuality, working with parallelism takes restraint and consideration. Like
a watchmaker placing tiny gears with a pincette. If we look back at the way we
constructed computational graphs in ```m1```, we were able to parallelize internally
in each node/operator, but if we had very small matrices with a big depth, we would
more or less be unable to do any parallelization, as the launching of threads to
parallelize the matrices themselves, might cost more than simply having a single
thread just handle the whole thing.

Some elements in your system you might be able to parallelize lock free, wherein
you find a solution without needing synchronization primitives like scopes,
barriers, atomics, locks or mutexes. Some parts of your system might be amenable
to fine-grained parallelism, such as a matrix-matrix multiplication, whereas
other parts might only be amenable to coarse grained parallelism, such as
a SLAM system pipelined into 4 stages, thus only being able to utilize 4 threads.

All of these put one thing into the center of everything. Can you guess it?

*Memory!*

Some ways of accessing memory can seem completely fine when single threaded,
but break down under the scrutiny of parallization. Trees can be hard,
especially if you also have to modify them. As one researcher found it, a
[hierarchical hash map](https://www.researchgate.net/publication/354065094_Practical_Spatial_Hash_Map_Updates)
performed siginifcantly better for some types of algorithms on the GPU.

Once you have the correct CPU based implementation, you should start
asking yourself, where is this going to run and how is the memory
accessed in order to accomplish what I want to do?

## Here Be Dragons
Harder to debug
Hazards
Data races
Race conditions

## Platforms
CPUs, efficiency and performance cores
GPUs, integrated and separate, accelerators within accelerators
Data Center GPUs
Multiple GPUs
FPGAs
Edge
Cloud
Neuromorphic - Not necromorphic
