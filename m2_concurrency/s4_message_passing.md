# 2️⃣ Message Passing
Let's use the [message passing memory model](https://en.wikipedia.org/wiki/Message_passing). 
Why is message passing easier than mutexes and atomics?
Synchronous and asynchronous message passing
The size of a buffer
What happens if it is dropped?
Is the receiver held by a thread that is blocking on the receiver or is it checking in once in a while
and executing?
The cost of this low cognitive cost is that executing is slower, especially for small workloads.
Good for having multiple threads doing different things, also known as
[task parallelism](https://en.wikipedia.org/wiki/Task_parallelism).

## 3️⃣ Rust MPSC Channels
Channels

## 3️⃣ Real-Time Message Passing
Maybe look at the try_rcv loop

## 5️⃣ Additional Reading
A longer, very friendly introduction to channels and message passing in Rust can be found
[here](https://doc.rust-lang.org/book/ch16-02-message-passing.html).

This is slightly larger scale, but when running code on multiple compute nodes in an HPC installation,
the most used method for sharing data between nodes is a standard called
[MPI](https://en.wikipedia.org/wiki/Message_Passing_Interface).
