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

With these lines we manipulate the base pointer itself, by reassigning a value of the base address (42),
incremented by 3 (54), before doing a simple dereferencing and assigning a value of 3. This is not a recommended
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
While it is import that you increase your understanding of what it takes to get valid,
predictable, boring code. Which is the best kind. What the guide is most interested in
is for you to write more performant code. An absolutely
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

Now that we have learned a bit about cache lines, we are equipped to talk actually talk about access patterns.
I have made some Rust code for you which is located at ```m1_memory_hierarchies/code/access_patterns/``` or
[online](https://github.com/absorensen/the-real-timers-guide-to-the-computational-galaxy/tree/main/m1_memory_hierarchies/code/access_patterns).

First off is sequential access. It is the one we usually strive for. We start at one end and go through every
element until the end, from index 0 to the end.
If everything is cache aligned, great! If not, the cost of not being aligned will
probably be as low as it can be, when we aren't reusing any retrieved elements. If a value, say a
4-byte integer is spread across two cache lines, that specific value may have to be reconstructed
which can be expensive.

Next up is strided access. With strided access we only read every N elements. Based on the size of the stride
and the size of the elements, it might result in each cache line only being used for a single element.
In the implementations in the code there is both a non-wrapping and a wrapping stride implementation, 
meaning once we step over the end we wrap back around using a modulo operator.
This is to ensure that it accesses the same amount of elements as the sequential access.
With the non-wrapping stride we only access every N elements, but we also end up doing
much less work.

Finally, we have random access. This is basically the worst case scenario. We randomly select an element
to access the same amount of times as the number of elements in the array.

<figure markdown>
![Image](../figures/access_patterns.png){ width="400" }
<figcaption>
Timing access patterns in Rust.
</figcaption>
</figure>

Given that we just talked about cache lines, most of these numbers make good sense.
Random access is catastrophic, wrapping strided access is bad, but most interestingly
non-wrapping strided access, which actually accesses less elements than the others,
is slower than sequential access for strides 2 and 3. With stride 4, where we are
only accessing one fourth the elements of the sequential access pattern, we begin to
get faster. But what do you know, sometimes the nice and predictable path,
which might seem like we are doing more work actually runs faster. What a time to be alive!

## Stacking Heaps of Trouble
If you aren't familiar with the [stack and queue](https://en.wikibooks.org/wiki/Data_Structures/Stacks_and_Queues)
data structure types, this would be a good time to follow the link and familiarize yourself.

The stack is not just a data structure, but also a core part of how all of the variables
in your local scope are kept track of when the program enters into a function. The stack
is a designated part of the memory allocated to your program. It starts at size 0.
Once you enter a function, each local variable is pushed unto the stack. The stack
generally requires that sizes are known at compile time. Once you call a function
from within your function, the local variables are no longer accessible to the function
you just entered, but once you return from that function, they are.
When you enter that function, a pointer to where you called the function from is added
to the stack and that function has its own local variables.

<figure markdown>
![Image](../figures/call_stack.png){ width="500" }
<figcaption>
The call stack.
<a href="https://en.wikipedia.org/wiki/Stack-based_memory_allocation">
Image credit </a>
</figcaption>
</figure>

If you push enough of these frames unto the stack, you can get a stack overflow.
This can for example happen if you write a recursive program that doesn't terminate.
In general, using variables from the stack will be much faster than using variables
from the heap. But we also can't return pointers to a stack variable as it might disappear
or be overwritten at any moment.

The heap, in this context, is not the actual data structure known as a heap.
Instead it is a bunch of unstructured memory living in the
[same reserved space](https://courses.grainger.illinois.edu/cs225/fa2021/resources/stack-heap/)
as the stack. 

<figure markdown>
![Image](../figures/stack_and_heap.png){ width="500" }
<figcaption>
The stack and the heap sharing memory.
<a href="https://courses.grainger.illinois.edu/cs225/fa2021/resources/stack-heap/">
Image credit </a>
</figcaption>
</figure>

Thus if either one becomes too big they begin encroaching on the other.
Everytime you ask for dynamically sized memory, it is allocated on the heap.
This is a slow process and you have to remember to deallocate the memory to not
get a memory leak. But the memory survives across functions now.
If you remember the pointer examples from earlier - the memory segment we asked for
lived on the heap, whereas the pointer (address) itself lived on the stack.
We are allowed to keep the pointer on the stack because a pointer is a known size
at compile time. We can also have arrays on the stack, but they generally need to
have a size known at compile time. Moving a pointer from place to place, is also
a lot cheaper than copying every single element of a large array every time
ownership changes hands.

## Dynamic Array
The dynamic array is ubiquitous in C++ and Rust. It is quite often what we think
about, when we think of arrays in those languages. C++ has
[```std::vector<T>```](https://en.cppreference.com/w/cpp/container/vector)
and Rust has [```Vec<T>```](https://doc.rust-lang.org/std/vec/struct.Vec.html).
I highly recommend reading the first parts of the Rust Vec page.
They are basically the same though and I will refer to them as vector from here on out.
A dynamic array bundles up the behavior we saw earlier with the pointers,
allocations and deallocations, but adds the ability to automatically create
a new array that is larger (usually by a factor of 2) than the old array and move the
old values over to the new array. The vector has three values. How much memory is in
its allocation, ```capacity```, how much of the memory is currently in use, ```length```, and a pointer to
the data which lives on the heap.
The vector itself can live on the stack and make sure to free the memory it points to
once the vector is dropped from the stack.
The vector supports quite a few operations, but the core ones are ```push```,
```pop```, array access ```[]```, ```reserve``` and ```shrink_to_fit```.

Let's start off though with how we allocate a vector (in Rust).

```rust
let mut data: Vec<i32> = Vec::<i32>::new();
```

In this case we should get a completely empty vector. It will have a default ```capacity```, because
we didn't specify any capacity it should start with. Let's just say this ```capacity``` is 4.
However, if we want to print the current size

```rust
let mut data: Vec<i32> = Vec::<i32>::new();
println("{}", data.len());
```

we would get an output of 0! We have a ```capacity``` of 4, but a ```size``` of 0. Meaning,
we have 4 integers of 4 bytes each on the heap, but they are unitialized (containing garbage values),
and we have not used any of them. If we however use ```push``` to add some actual data and then print 

```rust
let mut data: Vec<i32> = Vec::<i32>::new();
data.push(0);
data.push(1);
println("{}", data.len());
```

we would print the number 2. Now we have live, initialized values on the heap at indices 0 and 1.
We can print them by accessing the values directly.

```rust
let mut data: Vec<i32> = Vec::<i32>::new();
data.push(0);
data.push(1);
println("{}", data.len());
println("{}", data[0]);
println("{}", data[1]);
```

In this case we print 2, 0 and 1. Push finds the first unused index, which is conveniently indicated
by the ```size``` value, increments ```size``` and puts the value into the designated index. If we pushed
5 values however, once we reached the 5th push, assuming the default capacity was 4, we would see the 
5th push taking a lot of time compared to the other 4 pushes. In this case the vector would allocate
a new memory segment on the heap with a size of 8, copy all of the values from elements 0-3 and then add
the 5th value to the vector. Conversely, we can also use the ```pop``` function.

```rust
let mut data: Vec<i32> = Vec::<i32>::new();
data.push(0);
data.push(1);
data.pop();
println("{}", data.len());
println("{}", data[0]);
```

Now we end up printing the values 1 and 0. In theory, a dynamic array should move to a smaller array at some point.
Such as, when at a quarter of the reserved capacity. But in practice, Rust doesn't move to a smaller array
unless explicitly asked to do so using the ´´´shrink_to_fit´´´ function. In that case it will allocate and move
to an array that is exactly the size of ```size```, thus also making ```capacity``` the same. In practice,
you should only do this for large arrays which are unlikely to see more elements added to it.

But, in the case of knowing how many elements we actually we want to put in our vector, or at least an expcected
minimum amount, we can just create the vector in a way where it has already reserved that amount of capcity.
If you can at all do this, it is one of the easiest ways to get better performance as you remove a whole
bunch of allocations, deallocations and copying.
There's a variety of ways to control how allocation happens. The simplest one, if you know how
many elements you want in your vector in advance, is to just create the vector with that capacity.

```rust
let mut data: Vec<i32> = Vec::with_capacity(5);
data.push(0);
data.push(1);
data.push(2);
data.push(3);
data.push(4);
```

In this case, we have been unambigously upfront about how many elements we will put in the vector.
It was created with a ```capacity``` of 5 and a ```size``` of 0. We can also tell the vector to make sure we
have a ```capacity``` of at least N. If it already has ```capacity``` to meet the minimum, nothing happens.
If it doesn't it will allocate, copy and deallocate.

```rust
let mut data: Vec<i32> = Vec::<i32>::new();
let element_count: usize = 42;
data.reserve(element_count);
for index in 0..element_count {
    data.push(index as i32);
}

```

There are more idiomatic ways to do this in Rust, which might also be faster, but you get the gist!

## The Vector
But, we aren't just interested in single lists of numbers, sometimes, we would even like a matrix.
In Rust we can have fixed size, arrays defined like so:

```rust
let data: [i32; 2] = [0, 1];
```

If the sizes given to the array definition are constants, known at compile time, the array will be
stack allocated. 
From what we have learned previously, the elements will be stored in memory in the order of 0 and 1.
But what if we create a two-dimensional array?

```rust
let data: [[i32; 2]; 2] = [[0, 1], [2, 3]];
```
In Rust the elements will be ordered in memory 0, 1, 2, 3. But that is not a universal truth.
This is called row-major ordering and is the standard layout in C, C++, Rust, Python and
most modern languages.
The alternative is column-major which is seen in Fortran and Matlab.
In column-major ordering the elements would be ordered in memory as 0, 2, 1, 3.
Basically, the memory will be most tightly packed in the innermost dimension.
To iterate through a 3 dimensional vector, this triple for-loop would access the memory
in order.

```rust
let data: [[[i32; 2]; 2]; 2] = 
                            [
                                [[1, 2], [3, 4],
                                [[5, 6], [7, 8]]
                            ];

let x_dimension: usize = 2;
let y_dimension: usize = 2;
let z_dimension: usize = 2;

for x_index in 0..x_dimension {
    for y_index in 0..y_dimension {
        for z_index in 0..z_dimension {
            println("{}", data[x_index][y_index][z_index]);
        }
    }
}
```

Where as if Rust was favored column-major ordering the in-memory-order traversal would be

```rust
let data: [[[i32; 2]; 2]; 2] = 
                            [
                                [[1, 2], [3, 4],
                                [[5, 6], [7, 8]]
                            ];

let x_dimension: usize = 2;
let y_dimension: usize = 2;
let z_dimension: usize = 2;

for z_index in 0..z_dimension {
    for y_index in 0..y_dimension {
        for x_index in 0..x_dimension {
            println("{}", data[x_index][y_index][z_index]);
        }
    }
}
```

If you think back to stride and cache lines, traversing our 3-dimensional array like the above
in the actual case, where Rust is row-major, would be like the stride access we looked at earlier.
We could also do this with nested vectors. 

```rust
let mut data: Vec<Vec<i32>> = Vec::<Vec<i32>>::new();
data.push(vec![0, 1]);
data.push(vec![2, 3]);


let x_dimension: usize = 2;
let y_dimension: usize = 2;

for x_index in 0..x_dimension {
    for y_index in 0..y_dimension {
        println!("{}", data[x_index][y_index]);
    }
}
```

This is even worse though. We now have a 2-dimensional array, which is highly flexible, but we
have to dereference two pointers for every access.

There is another way of doing this with a vector, which is the way I will be using
multi-dimensional arrays in this module. It involves using a single dimensional vector
as if it had more dimensions.

```rust
let mut data: Vec<i32> = Vec::<i32>::new();
data.push(vec![0, 1, 2, 3]);

let column_count: usize = 2;
let row_count: usize = 2;

for x_index in 0..row_count {
    for y_index in 0..column_count {
        println!("{}", data[x_index * column_count + y_index]);
    }
}
```

We just create a vector with as much room as we need and then access it with a bit of calculation.
We've flattened our matrix and can now both have it dynamic and with arbitrary dimensions. We
can even dynamically decide to see the matrix in a different way, for example by deciding
to swap the number of columns and rows. The formula to access each element is to multiply
the index by the dimensions that come after it and add it to the next index. 
For example with three dimensions ```x```, ```y``` and ```z```, the index would be
calculated by ```x_index * y_size * z_size + y_index * z_size + z_index``` and for
the two dimensions ```x``` and ```y```, we would access the 2-dimensional matrix with
```x_index * y_size + y_index```. I really hope this makes sense. Once it clicks
it is a very simple formula, if a bit wordy.
Usually libraries will work like this under the surface but wrap it in an interface
for you to simply access it like it was a multi-dimensional array.

To wrap it up I have made a performance test of these approaches. The code
doesn't match completely as we need bigger dimensions to get a good test.
The code is at ```m1_memory_hierarchies/code/the_vector/``` or
[online](https://github.com/absorensen/the-real-timers-guide-to-the-computational-galaxy/tree/main/m1_memory_hierarchies/code/the_vector).

Implementing all of the methods described above in both row-major and column-major form,
as well as an element-wise version, where we flatten the multidimensionality to save
the administration of two of the for-loops, so we just get one for-loop running across
a vector, we get the following numbers.

<figure markdown>
![Image](../figures/the_multidimensional_vector.png){ width="500" }
<figcaption>
Access times for multidimensional arrays.
</figcaption>
</figure>

The functions named Multi-Array are stack allocated instead of heap, which is why
they are that fast. I was however unable to run them for 64x64x64 and 128x128x128.
Rust refused citing a stack overflow. Interestingly as well, the element-wise function
can be quite fast as it saves two of the for-loops. So, if you can, use element-wise.
Otherwise, the row-major single vector function seemed to work the best. How much
is saved by not having the two extra for-loops depends on how much work you are
actually doing in each iteration. In this benchmark we do pretty much nothing.

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
