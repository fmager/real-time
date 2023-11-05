# Tips and Tricks

* memcpy
* Check/validate everything before the hot loop
* Unitialized memory
* Hot loops, event loops
* Allocations in a hot loop
* Object Pools
* System calls - hoist out of the hot loop
* Logging and printing
* Walk, don't run, testing for correctness before optimization
* Don't use abbreviations
* Don't talk moon man language to me, ya Blargon!
* Don't use postfix incrementation++
* When to care about software engineering and when to care about performance
* Don't use a string key/identifier or integer, when a type safe enum will do the job
* Hard coding types
* Cognitive load, and delaying errors to after the first draft - deliberate development vs. debugging
* Prefer stateless programming, minimize stateful programming (functional inspiration)
* Implicit casting
* Templating
* Know your system - mobile, laptop, desktop, integrated memory, which GPU
* Use version control even for solo development
* Am I copying/cloning things that don't need to be copied?
* Anything that can be immutable, should be immutable - aliasing!
* Testing and Seeding RNG's
* [Faster RNG](https://youtu.be/5_RAHZQCPjE)
* Timing real-time systems and how to escape or offload compute
* Multi-resolution computing for making your real-time target, video streaming and image loading
* Pressure testing and worst cases
* Static/Dynamic Dispatch - dyn, enum, trait
* The Markov Chain
* If using recursion and risking stack overflow, use a loop and a queue
* If you can, always prefer an array over more complicated structures
* Sorting and random access - GyroDropout, Sorting Functions benchmark
* Is there contention around synchronization primtives such as mutexes?
