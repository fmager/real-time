# 3️⃣ Optimization
Much like the memory hierarchies there's levels to measuring performance.
The easiest, most often overlooked is just using your systems own task manager.
You will get a view of all of the programs currently running on your system, but
it requires very little work and is easy to do. But much like the black box approach
to understanding the performance of someone else's library, trying to deduce the
characteristics of your program through the task manager might not be enough.

Next up you can insert print statements and measure the timing of functions yourself.
This won't give you any information about how many threads are in play, how much
memory is used, or how many L2 cache misses you have, but it will allow you to
measure the functions in your own code directly. This might require you to radically
change your code radically in order to get adequate measurements.

The harder to install, the more info it can probably give you

## 5️⃣ Additional Reading
[Full-Stack, GPU-based Acceleration of Deep Learning](https://nvlabs.github.io/EfficientDL/)  
[Writing Performant Concurrent Data Structures](https://www.youtube.com/watch?v=XKODaZgKcnE)  
