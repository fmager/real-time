# Intro
Why should you care about this.
Memory hierarchies are quite complex when you add in things like GPU's, the internet, multi-GPU-node setups.
There are a lot of tools like PyTorch, Tensorflow and JAX, that put certain restrictions
on the user. Why is that and why are restrictions a good thing?
A lot of it doesn't necessarily have much to do with how to calculate numbers faster, but how
to move the data there, and often, when not to move it back just yet.
In order to reason about these black box systems, it is quite helpful to have some idea
of what is going on.

## Code
In this module you will find the code in ´´´m1_memory_hierarchies/code/´´´.
Most of the sections deal with the project in ´´´computational_graphs´´´.
