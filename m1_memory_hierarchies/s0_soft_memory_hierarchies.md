# Soft Memory Hierarchies
<figure markdown>
![Image](../figures/amd_athlon_hierarchy.png){ width="600" }
<figcaption>
Memory hierarchy of the AMD Athlon.
<a href="https://en.wikipedia.org/wiki/File:Hwloc.png">
Image credit </a>
</figcaption>
</figure>

As mentioned in the module intro, the CPU's memory hierarchy is represented by a series of
hardware components with different sizes and speeds.
But don't fret, memory hierarchies and their hardware design subtleties won't be the primary focus of this module.
This section will focus on the various programming constructs to better use the memory hierarchy.
First off we will start bridging hardware and software.

## Getting to the Point(er)
One of the core mechanisms in using memory is the pointer! All it does is point to pieces of memory.
Why? Because a pointer is basically just an address. Anti-climactic, I know, but as one of the core building
blocks of computing, we need to take a bit of time to look at what it is.
If you have ever tried programming in C, you will invariably have been introduced to the pointer.
The examples in this heading will be in C, but don't worry, we won't even define an entire function.
It is rife with opportunities for making trouble, to a degree where in Rust, which is made to be a
reasonably safe language, you can't directly interact with a pointer unless you have an unsafe region
around the pointer interaction. Yikes! On the other hand, you can get some of the most extreme performance
by using raw pointers. So let's take a look!

### Allocation
First of all, how do we get a pointer? Please note that checks for whether we have been given a valid pointer
have been omitted. In the example below we get a pointer to a piece of memory which can hold up to 42 elements.

```c
int element_count = 42;
int* integer_array;
integer_array = malloc(element_count * sizeof(int));
```

Let's break it down!

```c
int element_count = 42;
```

We assign the number of elements to a variable in order to not have magic numbers.

```c
int* integer_array;
```

This is actually bad practice. We have an uninitialized variable here. We could try and dereference the pointer,
more on that in just a second, and try to access memory which we either don't have the right to access
or which doesn't exist. The pointer at this point is likely to either be 0 or complete garbage.
```int*``` reads as "a pointer to integers" or "address of one or more integers".

```c
integer_array = malloc(element_count * sizeof(int));
```

We ask for a memory allocation ([malloc](https://en.cppreference.com/w/c/memory/malloc)) from the
operating system. What we get back is just a runtime dependent address.
The address itself is what is known as a [word](https://en.wikipedia.org/wiki/Word_(computer_architecture)).
The size of the word dictates how much memory you can address in a system. If you have a 32-bit, 4 bytes, word
and you use byte addressing, meaning one byte for every address, we can at most address 2GB of memory with a
single word. If we have 64-bit words we can address more memory than we could possibly get. When you
see something is a 32-bit or 64-bit operating system, this is why! It is also why we all of a sudden started
using more than 2GB of RAM per computer in the 2000's.
The address given by ```malloc``` will be different every time you run your code.
Usually, any call to the operating system will be a very slow operation and should happen as little as possible.
This can be stuff like writing to a terminal, accessing a file on disk, and so on.
What we give malloc as an argument is the number of BYTES, as in 8-bits per element, we want.
We want ```element_count``` elements which should each have a size of 32-bits (4 bytes).
```sizeof(int)``` returns 4. In total we ask for 168 bytes. ```malloc``` itself returns ```void*```.
Since C allows for implicit casting, what happens is that C, without us asking,
changes the type to ```int*```. Underlying it is the exact same thing.
It is an address where 168 bytes allocated for us begins. What changes from ```void*``` to ```int*``` is
how we dereference the pointer and what happens when we do.

### Dereferencing
A pointer is a reference to another place in memory. Quite literally it is just a number.
Dereferencing is a term for following the address to what it points to.

```c
int element_count = 42;
int* integer_array;
integer_array = malloc(element_count * sizeof(int));

*integer_array = 0;
*(integer_array + 1) = 1;
integer_array[2] = 2;
integer_array = integer_array + 3;
*integer_array = 3;
```

In this example there's three different ways of dereferencing shown.

```c
*integer_array = 0;
```

In C, we use the ```*``` operator in front of the pointer to follow the address to the memory.
The base pointer we got from ```malloc``` is the address of the first of the 42 elements in our memory.
Another way of seeing it is that ```integer_array``` holds an address, let's say... 42. Our program
now asks the CPU to write to the address 42, the number 0. So far so good. But then this happens.

```c
*(integer_array + 1) = 1;
```

This is one of the myriad reasons why we needed to have an ```int*```. If the address in ```integer_array``` is
42, to get the next integer element, we don't go to the address 43, which would just be the second byte of the
first element. No, we want to go to the address 46, where the second element in the array begins. Since
```integer_array``` has the type ```int*```, we have defined that each element is 4 bytes and we now have a
STRIDE of 4 bytes.
We also need to keep track of the size of our allocation close to the pointer itself,
as trying to access an element outside of our allocation will be catastrophic, and likely result in a
[segmentation fault](https://en.wikipedia.org/wiki/Segmentation_fault). So, no ```integer_array[42]```.
Back to the line on hand. We put our ```integer_array``` in a parentheses to make sure the
dereferencing doesn't happen until after we have changed the address. So we increment the base pointer (42)
with a stride of 4 (46), and then dereference (*) to assign a value of 1 to the second element in our array.

```c
integer_array[2] = 2;
```

A short hand for the previous line, is this line. ```integer_array[2]``` is shorthand
for ```*(integer_array + 2)```.

```c
integer_array = integer_array + 3;
*integer_array = 3;
```

With these lines we manipulate the base pointer itself, by reassigning a value of the base address (42), incremented by 3 (54), before doing a simple dereferencing and assigning a value of 3. This is not a recommended
way of doing things. How do we ensure that we always have the pointer to the base address?
The least you can do is to copy the base pointer and increment that. Why?

```c
int element_count = 42;
int* base_integer_array = malloc(element_count * sizeof(int));

*base_integer_array = 0;
*(base_integer_array + 1) = 1;
base_integer_array[2] = 2;

int* integer_array = base_integer_array + 3;
*integer_array = 3;
integer_array[1] = 4;
```

Because we need the address to give the memory back to the operating system.

### Deallocation
Once we are done with the section of memory we have so graciously been granted by the operating system,
we should remember to return it to the operating system. If we don't we might get a memory leak,
which is when our program uses more and more memory until the program is stopped or crashes.
The operating system might keep track of the memory though and clean up once our less than stellar code terminates.

In C, we can return our memory like this, using the [free](https://en.cppreference.com/w/c/memory/free) function.

```c
int element_count = 42;
int* base_integer_array = malloc(element_count * sizeof(int));

*base_integer_array = 0;
*(base_integer_array + 1) = 1;
base_integer_array[2] = 2;

int* integer_array = base_integer_array + 3;
*integer_array = 3;
integer_array[1] = 4;

free(integer_array);
```

Spot the error?

We had two pointers and forgot to ```free``` using the base pointer, ```base_integer_array```.
This is [undefined behavior](https://en.wikipedia.org/wiki/Undefined_behavior),
which means that there are literally no definitions of what will happen.
It is really bad. What we should have done was this.

```c
int element_count = 42;
int* base_integer_array = malloc(element_count * sizeof(int));

*base_integer_array = 0;
*(base_integer_array + 1) = 1;
base_integer_array[2] = 2;

int* integer_array = base_integer_array + 3;
*integer_array = 3;
integer_array[1] = 4;

free(base_integer_array);
```

Note that ```free``` takes a ```void*```. Our ```int*``` is cast, without us asking explicitly, to a ```void*```.
The operating system just wants an address. This allows the operating system to mark the section,
denoted by the start of the section, and probably by its own record of the length.
Note also that the address (42) held by ```base_integer_array``` is still in play.
It is what is known as a 'dangling pointer'.
We could try to dereference it after giving it to ```free```, which is the notorious use after free.
This is also undefined behavior as we try to access memory that is no longer accessible by our program.
What we could do is to set ```base_integer_array``` and ```integer_array``` to new values to denote
that they were invalid.

```c
int element_count = 42;
int* base_integer_array = malloc(element_count * sizeof(int));

*base_integer_array = 0;
*(base_integer_array + 1) = 1;
base_integer_array[2] = 2;

int* integer_array = base_integer_array + 3;
*integer_array = 3;
integer_array[1] = 4;

free(base_integer_array);
base_integer_array = NULL;
integer_array = NULL;
```

This does not however, stop us from trying to dereference those pointers, but it does allow for a more
general check to see whether the pointers are still valid.

```c
if (base_integer_array != NULL){
    free(base_integer_array);
}
```

If this all seems a bit scary, that's because it is.
Anytime a system depends on humans just not making any errors and being
rockstars at everything, it's a dangerous system and you should be on guard.

## Access Patterns  
One important part of you understanding memory is increasing your understanding of what it takes to get valid,
predictable, boring code. Which is the best kind. The other is for you to get performant code. An absolutely
essential part of getting performant code is how we access the underlying memory. Yes, we can address
memory a single byte at a time with [byte addressing](https://en.wikipedia.org/wiki/Byte_addressing). 
But, whenever we ask for a byte, the memory is transported as a cache line through the memory hierarchy.
As in, the L3, L2 and L1 cache all receive an entire cache line. That cache line is usually 64 bytes.

What is in the cache line is dictated by
[cache line alignment](https://en.algorithmica.org/hpc/cpu-cache/alignment/).
If for example you had made a struct (it's like an object, but just the data) like the one below
and you elected to turn off the auto-alignment with ```__attribute__ ((packed))```

```c
struct __attribute__ ((packed)) my_struct
{ 
    short first; // 2 bytes 
    int second; // 4 bytes
}
```

and you made allocated an array of ```my_struct``` like so

```c
int element_count = 4;
my_struct* structs = malloc(element_count * sizeof(my_struct)); // 4 * 6
structs[1].first = 0;
structs[1].second = 0;
```

if you had an alignment of say, 8 bytes, the last two lines would result in 2 cache lines being retrieved.

<figure markdown>
![Image](../figures/cache_alignment.png){ width="600" }
<figcaption>
Bad cache alignment.
</figcaption>
</figure>

Which is not good. What we could do instead would be to pad our struct a little bit,
which is the default behavior in C.

```c
struct my_struct
{ 
    short first; // 2 bytes 
    short _pad; // 2 bytes
    // Usually in C it will fix this automatically, padding
    // every element to a multiple of a value. This could for example
    // be 4 bytes.
    int second; // 4 bytes
}

int element_count = 4;
my_struct* structs = malloc(element_count * sizeof(my_struct)); // 4 * 6
structs[1].first = 0;
structs[1].second = 0;
```

Then our alignment becomes this.

<figure markdown>
![Image](../figures/cache_alignment_fixed.png){ width="600" }
<figcaption>
Better cache alignment.
</figcaption>
</figure>
And we now only involve a single cache line. Which to remind you, is quite a bit smaller than the
more standard 64 byte cache line. Now that we have that in place, let's take a look at some of the ways we can
run through an array of values.

Sequential, Strided, Random
Some code located at ```m1_memory_hierarchies/code/access_patterns/``` or
[online](https://github.com/absorensen/the-real-timers-guide-to-the-computational-galaxy/tree/main/m1_memory_hierarchies/code/access_patterns).

<figure markdown>
![Image](../figures/access_patterns.png){ width="600" }
<figcaption>
Timing access patterns in Rust.
</figcaption>
</figure>

## A Heap of Trouble
### Stack

* Function Call -> Capture local variables
* Recursion -> Stack Overflow

### Heap

## Dynamic Array 

A bit like a stack
Maybe some of this should be moved to vector games  

* The struct
* Allocation
* Pop -> Rust doesn't shrink, but .shrink_to_fit(), does C++ shrink?
* Reallocate
* Free

Remember the more detailed description in the level 3 section Virtualized Memory Hierarchy.  
Confusingly, there is also a data structure called heap. This heap is not the same.  

## The Vector
Column major -> Fortran, Matlab  
Row Major -> C, C++, Rust, Python (when do they actually allocate memory like this?)  
Vec<Vec<T>> -> Double Pointer  
Vec<T> and linearized memory  

## Move, Copy, Clone, Soldier, Spy
Clone  
Copy  
Move  
Vector Games  

## \*The Vector Reloaded
Strided Access and the transposition  
Permutations  
Jagged Arrays  
Sparse Arrays  
Using indices instead of pointers allow for predictability and seralization  

## \*Smart pointers
Why smart pointers -> Safety  
Smart pointers:  

* unique/box
* shared/rc/arc
* weak/weak

## \*Graphs and Trees
Graphs  
Trees  
Graphs and Trees using  

* Pointers
* Smart pointers
* Indices (static, dynamic issue getting a mutable reference to the collection in Rust)

## \*Virtualized Memory Hierarchy
Find a more formalized definition of virtualized memory  
The process' own virtual memory space (the stack and heap share the same memory)  
Stack/Heap Visualization  
Disk -> Image addresses for training networks -> Fat nodes/payload options  
Internet -> Rendering on the internet, or pulling images from the internet  

## \*Garbage collectors
Reference counting
How to still do memory leaks -> cyclical references, but save some for the exercises
Generational Garbage Collection
Calling the GC yourself
Object Pools

## \*Further Reading
An explanation of memory allocation, stack and heap
[in C](https://cs2461-2020.github.io/lectures/dynamic.pdf)

A more rigorous [explanation](http://eceweb.ucsd.edu/~gert/ece30/CN5.pdf)
of the register, cache, main memory and virtual memory parts of the memory hierarchy.

Check out the memory and cache specs for Apple's [M1 series](https://en.wikipedia.org/wiki/Apple_M1).

For more about
[garbage collection in Python](https://devguide.python.org/internals/garbage-collector/),
[more basic garbage collection in Pyton](https://stackify.com/python-garbage-collection/) or
[garbage collection in Java](https://blogs.oracle.com/javamagazine/post/understanding-garbage-collectors).
