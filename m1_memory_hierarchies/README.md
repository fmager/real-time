# Memory Hierarchies, Computational Graphs and Compilers
Not all efficiency comes from optimizing the various computational details like multiplications, divisions
and such of a function.
Quite a large part of it, in fact, comes from optimizing how much you write to and read from memory.
'Which memory?' you might ask, rightfully. The answering the where, when and what of memory will be
the focus of this module. We can almost always get more cores to throw at a problem, we can also,
at least on the CPU, quite easily get more memory, but that does not change the amount of time it takes
to get a piece of memory, only how much data we can have in memory before we have to go to a lower level,
e.g. go from RAM to disk. This is even more important given the relatively slow improvement of memory over time.
<figure markdown>
![Image](../figures/compute_vs_memory.png){ width="500" }
<figcaption>
<a href="https://www.cs.umd.edu/~meesh/411/CA-online/chapter/memory-hierarchy-design-basics/index.html">
Image credit</a>
</figcaption>
</figure>

## Perspective
The further you move from simple, albeit heavy, problems such as a matrix-matrix problem to more heterogenous
problems, such as training a neural network, the harder it can be to get good performance. How do you know or
reason about what is where when in complex systems like
[Pytorch](https://pytorch.org/), [Tensorflow](https://www.tensorflow.org/),
[JAX](https://jax.readthedocs.io/en/latest/), [Numba](https://numba.pydata.org/) and
[Taichi](https://www.taichi-lang.org/). All of these frameworks, compilers and domain specific languages have to
nudge you in different directions to give them the restrictions and hints needed to let them run your code
as efficiently as possible. Nudges like defining your neural network as a computational graph. If you're unsure
about what a computational graph is, the basic version is that you define a bunch of operations and how they relate
to each other. Like input layer, followed by linear layer, followed by ReLU. But more on that later! Other advances
include Pytorch, after several attempts with various degrees of succes, finally introducing a
[compiler](https://pytorch.org/tutorials/intermediate/torch_compile_tutorial.html) for optimizing the
neural network you just defined.
Or the functional programming style used by
[JAX](https://jax.readthedocs.io/en/latest/notebooks/Common_Gotchas_in_JAX.html) in conjunction with
the [XLA compiler](https://www.tensorflow.org/xla).

## The Hierarchy of your Memory is about to change
So what is memory anyways? Memory in a compute is represented in several stages, all having their own capacity and speed.
In order from smallest capacity and highest speed to largest capacity and lowest speed we have the register, the
L1-L3 caches, the main memory (RAM) and the disk.
The register is the fastest and smallest of the bunch. It resides right next to the parts of the CPU that does the computations.
As a rule of thumb, all of the variables you declare in the scope of your function, unless there is A LOT of variables,
will be kept in registers. The caches and the main memory all work in conjunction with each other as an invisible
way of speeding up the accesses to the main memory.

Say you load in a file from the disk. If small enough, that entire file can be kept in RAM.
Which is great! We could keep all of the values in a great big array which we
could access, like ```current_value = data[index]```. But if you just wanted to read the first 5 values in the file
in a loop, it would be incredibly slow to load those 5 values over and over again all the way from memory.
What happens instead is that those 5 values might have separate copies in the L3, L2 and L1 caches, perhaps even in
the registers. That would speed up things greatly. Whenever we asked for the first value we would first ask the
L1 cache, do you have this value? If yes, that would be a cache hit, and we would pay a small amount of time to
retrieve the value. If the L1 cache did not have a valid copy of the value, we would ask the L2 cache, and so on
and so on, until we reach memory. If our file was too big to fit in memory, the operating system might even
virtualize (don't worry about it) the memory and go all the way to the disk or to the internet to retrieve our value.
This is just as slow as it sounds.

<figure markdown>
![Image](../figures/memory_hierarchy_PLACEHOLDER.png){ width="500" }
<figcaption>
<a href="https://www.cs.umd.edu/~meesh/411/CA-online/chapter/memory-hierarchy-design-basics/index.html">
Image credit REPLACE ME </a>
</figcaption>
</figure>

To further complicate things, multicore CPU's have each CPU sharing the disk, memory and L3 cache.
We are also at risk of each core not just reading from the same values, but what if some of them modified
one or more of the 5 values? At any point in time a value loaded from memory, to L3 cache, to L2 cache, to L1 cache,
to the registers of thread A, might become invalidated because thread B wrote to that value. This may have updated
the value in memory and in thread B's registers, L1 and L2 caches, hopefully, it also updated it in an L3 cache it
shared with thread A, but even then we would still need to move the value from L3, to L2, to L1 to registers.
Then the chaos could start once again by thread A modifying the value. My, oh, my. And that is just in the case
where the contention around this data, also know as a
[data race](https://www.brainkart.com/article/Data-Races_9445/),
is caught and we even end up with a correct result.
Most likely thread A will end up with a stale version of the data and will continue as if the value had never been modified.
Thread A will then write its own new version of the value, or just be working off an old version, resulting in
incorrect results.

Nudging the programmer (that's you!), to better define your program, not just line-by-line, but as a whole,
to constrain these sorts of contentions, is one of the myriad reasons why
frameworks like Pytorch can greatly speed up your code, but you need help it along.

For a more in-depth explanation on the memory hierarchy see this chapter on
[Memory Hierarchy Design](https://www.cs.umd.edu/~meesh/411/CA-online/chapter/memory-hierarchy-design-basics/index.html).

## Expanding the Memory Hierarchy
To top it off... START HERE
Memory hierarchies, within and without.
GPU's and virtualization.

## Complex Systems
We are increasingly using sophisticated systems, like JIT-compilers and computational graphs to accelerate.
Graphs for memory movement.
A lot of the same guarantees and analysis we put on to our compute graphs, mirror what we do on the
scale when verifying our programs for correctness. Especially, when parallelising code.
Graph compilers for further optimization. Multiple GPUs.
