# 2️⃣ Soft Memory Hierarchies
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

## The Dynamic Array
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
                                [[1, 2], [3, 4]],
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
calculated by

```rust
x_index * y_size * z_size + y_index * z_size + z_index
```

and for the two dimensions ```x``` and ```y```, we would access the 2-dimensional matrix with

```rust
x_index * y_size + y_index
```

I really hope this makes sense. Once it clicks it is a very simple formula, if a bit wordy.
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
Now that we have examined how we can deal with a more expensive type,
compared to the simpler integer or float, let's expand the scope a little bit.
How do we actually move around these vectors as data? In each language there are
some implicit rules, which can have wide reaching consequences, both in terms of
correctness and performance.

In Python, variables are all references to an underlying object, which is freed
when there are no longer any references to said object. Don't worry about it too
much, it is a level 3 concept I will introduce further down the page.
But, it does have consequences when this happens

```python
x = [5, 5, 3, 42]
y = x
```

There aren't actually two lists, but two references to a list which has
some data on the heap.
This can be a bit problematic, as you now have two variables, which can
both write to the same list without the other knowing.
Once both ```x``` and ```y``` go out of scope, the list on the heap will be
deallocated (eventually).

In C and C++, the following actually results in two different lists on the
heap, kept by two different variables.

```c++
vector<int> x{5, 5, 3, 42};
vector<int> y = x;
```

C++ is copy by default, and this is a deep copy. Which is what Rust would
call a clone. Rust however, is move by default.

```rust
let x: Vec<i32> = Vec::from([5, 5, 3, 42]);
let y: Vec<i32> = x;
```

Once the values in ```x```, the ```capacity```, ```size``` and the pointer
to the memory on the heap, have been moved from ```x``` into ```y```,
```x``` is no longer accessible. The Rust compiler will complain.
We can however, move it right back.

```rust
let mut x: Vec<i32> = Vec::from([5, 5, 3, 42]);
let y: Vec<i32> = x;
x = y;
```

Now, ```y``` is inaccessible at the end. We could also create a scope,
after which ```y``` is dropped, but the ownership is not moved back to ```x```.

```rust
let x: Vec<i32> = Vec::from([5, 5, 3, 42]);
{
    let y: Vec<i32> = x;
}
```

Unless we move the values back ourselves.

```rust
let mut x: Vec<i32> = Vec::from([5, 5, 3, 42]);
{
    let y: Vec<i32> = x;

    x = y
}
```

To actually create two lists, like we did in the C++ example, we have to
explicitly ask for a deep copy - a clone in Rust terminology.

```rust
let x: Vec<i32> = Vec::from([5, 5, 3, 42]);
let y: Vec<i32> = x.clone();
```

Usually, in Rust at least, adding lots of clones everywhere is the way
to get around the borrow checker and have everything be correct. But
once your first prototype is finished, one of the easiest improvements
to your performance will be to search for all instances of .clone() and
see whether there is some other solution that might work better.
Rust isn't fighting you in this case, even if it can be strict,
it is trying to protect you from having multiple write-enabled
references to the same data, as in the Python example, which could make for incorrect code.
C++ does have these [move operations](https://en.cppreference.com/w/cpp/utility/move)
as well, it is even highly recommended a lot of the time. It is however,
not the default behavior of the language.

Rust does however have something called traits (don't worry about it).
One of these traits is the ```Copy``` trait. If a type implements
the ```Copy``` trait, it will be
[copied rather than moved](https://blog.logrocket.com/disambiguating-rust-traits-copy-clone-dynamic/)
when assigned to a new value or passed as an argument to a function.
It is sort of like an implicit version of ```.clone()```, except
in the case of deeper structures, such as ```Vec<T>```, in that case,
it would copy all of the stack values, ```capacity```, ```size```
and the pointer to the memory on the heap.

But hold on a minute! That is illegal! We would have two pointers with
full write rights. Which is illegal in Rust! Which is also why ```Vec<T>```
doesn't implement ```Copy``` and this has all been a ruse, for your edification.

_________________

## 3️⃣ Smart pointers
Ok, so I promised previously, that I would explain how Python, and most other
garbage collected languages, deal with assigning one variable to another.
If you recall the previous example

```python
x = [5, 5, 3, 42]
y = x
```

We start by making a list and assigning a reference to ```x```. In this case
```x``` is not the actual owner of the list. Instead, the system takes
ownership of the list, and ```x``` is a live reference to that list.
The system keeps track of how many live references there are to the list.
Once ```x``` goes out of scope, the live reference count for the list
decreases by one. Once the live reference count reaches 0, it is deallocated.

Until we hit the end of the scope, and ```x``` and ```y``` disappear, there
are two live references to the the list created at line 1. While a fine enough
solution at first glance, sometimes, answering the question "what is alive"
can be quite difficult. More on that in the
[garbage collectors section](https://absorensen.github.io/the-real-timers-guide-to-the-computational-galaxy/m1_memory_hierarchies/s0_soft_memory_hierarchies/#garbage-collectors).

When dealing with raw pointers, like we saw earlier, once a system grows
beyond absolute simplicity, sharing multiple pointers to the same object
becomes a bit complex. If you have 5 pointers to the same object floating about
how do you ensure it isn't used after freeing? Who deallocates the pointer
and who ensures that the pointers are no longer valid? This at the absolute
crux of safety and your program not blowing up in C and C++.

In C++11+ and Rust, we can elect to use something called smart pointers. Which
can handle some of the intricacies for us.
First off there is the [unique_ptr<T>](https://en.cppreference.com/w/cpp/memory/unique_ptr),
as in C++, or the [Box<T>](https://doc.rust-lang.org/std/boxed/index.html) in Rust.
I will just refer to ```Box``` from here on out, their behaviors seem to be more or less the same.
```Box<T>``` is like a ```T *``` in C (pointer to object of type T).
With two notable exceptions. It cannot be copied. As in, you cannot have multiple
instances of ```Box``` pointing to the same underlying object. Thus ```Box``` in Rust,
as well as in C++, requires that ownership is moved, and not copied.
The other notable difference from a raw pointer is that once the ´´´Box´´´ goes out of scope,
the object on the heap that it is pointing to is deallocated.  

```rust
let box_variable: Box<i32> = Box::new(42);
let mut other_box: Box<i32> = box_variable; // box_variable no longer accesible due to move
let copied_variable: i32 = *other_box; // Dereference and copy the underlying value, this is not a move
*other_box += 1;
println!("{}", copied_variable); // prints 42
println!("{}", *other_box); // prints 43
```

Next up are the shared pointers. They are essentially what Python is using in the example
from earlier. In C++ it is called [shared_ptr<T>](https://en.cppreference.com/w/cpp/memory/shared_ptr),
in Rust it actually comes in two versions;
[Rc<T>](https://doc.rust-lang.org/std/rc/index.html) and
[Arc<T>](https://doc.rust-lang.org/std/sync/struct.Arc.html).
```Rc``` stands for reference counted. It is only made for single threaded usage as the
reference count itself is susceptible to a data race, which you may recall, is several
reads and/or writes to the same value. This could result in the count of live references
being incorrect and the underlying value never being deallocated.

```rust
use std::rc::Rc;
fn main() {
    let shared_reference_a: Rc<i32> = Rc::new(42); // Live references = 1
    println!("{}", Rc::strong_count(&shared_reference_a)); // prints 1

    let shared_reference_b: Rc<i32> = shared_reference_a.clone(); // Live references = 2
    println!("{}", Rc::strong_count(&shared_reference_b)); // prints 2

    {
        let shared_reference_c: Rc<i32> = shared_reference_a.clone(); // Live references = 3
        let shared_reference_d: Rc<i32> = shared_reference_b.clone(); // Live references = 4
        
        println!("{}", *shared_reference_c); // prints 42
        println!("{}", Rc::strong_count(&shared_reference_a)); // prints 4

        println!("{}", *shared_reference_d); // prints 42
        println!("{}", Rc::strong_count(&shared_reference_d)); // prints 4

    }
        // shared_reference_c and shared_reference_d are now dropped
        println!("{}", Rc::strong_count(&shared_reference_b)); // prints 2

        // Live references = 2
        println!("{}", *shared_reference_a); // prints 42
        println!("{}", *shared_reference_b); // prints 42
}
```

```Arc<T>``` is here to solve exactly that issue.
It uses atomic reference counting. Atomics will be introduced in the
[Concepts in Parallelism](https://absorensen.github.io/the-real-timers-guide-to-the-computational-galaxy/m2_concepts_in_parallelism/)
module. But in this context, it means that the reference counting is thread-safe, but a bit slower.

```rust
use std::sync::Arc;

fn main() {
let shared_reference_a: Arc<i32> = Arc::new(42); // Live references = 1
let shared_reference_b: Arc<i32> = shared_reference_a.clone(); // Live references = 2

{
    let shared_reference_c: Arc<i32> = shared_reference_a.clone(); // Live references = 3
    let shared_reference_d: Arc<i32> = shared_reference_b.clone(); // Live references = 4
    
    println!("{}", *shared_reference_c); // prints 42
    println!("{}", *shared_reference_d); // prints 42
}
    // shared_reference_c and shared_reference_d are now dropped

    // Live references = 2
    println!("{}", *shared_reference_a); // prints 42
    println!("{}", *shared_reference_b); // prints 42

}
```

While ```shared_ptr``` from C++ allows you to mutate the value it refers to
```Rc``` and ```Arc``` do not. They require a synchronization primitive wrapped around your
underlying value, like ```Arc<RwLock<i32>>```, but that is more advanced usage,
and don't worry about it right now. Other than the atomicity, and being shareable between
threads, ```Rc``` and ```Arc``` work more or less the same.

Finally, we have the weak pointer. This basically exists to weaken cyclical references.
If object A refers to another object, object B, with an ```Rc```, while the object
B refers to object A, we have a problem. When either, or both go out of scope,
they will not be deallocated as there is live references to both.

Try to take a second and imagine this and the things that can go wrong
when there are multiple references interconnected.

Go on.

I'll wait.

To solve this issue, the weak pointer comes to the rescue. It is along for the party,
but doesn't actually keep things alive.
In Rust it is called [Weak<T>](https://doc.rust-lang.org/std/rc/struct.Weak.html).
It can reference the same underlying object as the shared pointer it comes from,
but does not contribute to the live reference count. As such, it can allow you
to have cyclical references, without causing a memory leak.
If object A points to object B with an ```Rc``` reference, but object B
holds a ```Weak``` reference to object A, once object A goes out of scope,
both object A and object B can safely be deallocated.

```rust
use std::rc::Rc;
use std::rc::Weak;

fn main() {
    let shared_reference: Rc<i32> = Rc::new(42); // Live references = 1
    let weak_reference: Weak<i32> = Weak::new(42); // Create a weak reference from nothing
    let weak_shared_reference: Weak<i32> = Rc::downgrade(&shared_reference);

    println!("{}", Rc::weak_count(&shared_reference)); // prints 1!
}
```

For more information on smart pointers in Rust, there is a nice example
[here](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
and another example about
[reference cycles](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html),
which is what we needed weak pointers for.

## 3️⃣ The Vector Reloaded
This isn't meant to be a one-to-one representation of how tensors work in ```numpy``` or
```PyTorch```, but combined with creating different views on the same underlying
1-dimensional memory as we learned about earlier, we can look at a few other fun
concepts in different ways to arrange tensors.

### Strided Access and Transposition
One of the most used operations is the matrix matrix multiplication.
If we assume 2 2D matrices as input and output into another 2D matrix,
one of those input matrices will be accessed with a stride access in
a column major form.

<figure markdown>
![Image](../figures/mat_mul_strided_access.png){ width="500" }
<figcaption>
Matrix-matrix multiplication. The numbers indicate access order.
</figcaption>
</figure>

There is a solution for this. We can just transpose the second input matrix.
Transposition, as you may remember, is flipping a matrix around the diagonal.
Another way to do this is to flip all coordinates. (0, 0) becomes (0, 0), but
(3, 1) becomes (1, 3).
Transposition is an expensive operation however, and we have to create additional code
for whether the second input matrix is transposed and the other multiplication
code for just that case. We also need to keep track of which matrices are transposed.
In a more general, flexible system, or one in which the system does a lot of optimization
without user input, we also need to evaluate when and where to tranpose matrices.
But, if the matrix is fairly static and is read from often, it can definitely be worth
the time and effort.

<figure markdown>
![Image](../figures/mat_mul_strided_access_transposed.png){ width="500" }
<figcaption>
Matrix-matrix multiplication with the second matrix transposed.
</figcaption>
</figure>

Now, lets try out a simple example! Checkout the code at
```m1_memory_hierarchies/code/strided_access_and_transposition``` or check it out
[online](https://github.com/absorensen/the-real-timers-guide-to-the-computational-galaxy/blob/main/m1_memory_hierarchies/code/strided_access_and_transposition/src/main.rs)
.

Interestingly, when running the code there doesn't seem to be much of a difference until
the matrix sizes become quite big. Why do you think that is?

<figure markdown>
![Image](../figures/strided_access_transposition_benchmark.png){ width="500" }
<figcaption>
Difference gets bigger as the matrices get bigger.
</figcaption>
</figure>

One guess would be a combination of the compiler aggresively optimizing the code, the branch prediction of the
pipeline (don't worry about it) being really good at guessing these very uniform workloads, but most importantly,
the caches doing a lot of the heavy lifting for us. Once the caches run out of space we begin to see a gap
between the two ways of doing it. This might be more pronounced on the GPU. In most cases you should probably
start with making the simplest and easy comprehendible code and try out (AND MEASURE!!!!) potential
optimizations before spending your time going down rabbit holes. This is will be a bit of a theme
in the next few sections. Not much of a difference in anything until the caches begin running out of space.
At least if you aren't coding something really terrible, like randomized access.

### Permuted Arrays
Sometimes we might want to change around elements in a matrix, without permanently executing
the change. Not permanently executing these changes may also allow for several different
views of the same data. So let's take a look at how permutations work.

In the example below, the permutation is kept track of with the data in one vector and
the index changes in another. The second of the two indices we need to map from one
index to another is implicit. Thus for our permutation vector, index 0, means that
at index 0 in our new permuted array resides at index 4 in our original data.

This is likely to be quite a bit slower compared to normal sequential access as we now
have to follow more than one pointer to get to our data.

<figure markdown>
![Image](../figures/permutations.png){ width="500" }
<figcaption>
Create permutations of an array by creating a list of indices and permuting that list.
</figcaption>
</figure>

If we only view the data through the lens of the permutation array anyway and we
read from this vector alot, we might as well execute the permutation. If we
wanted to be able to undo the permutation, we could just keep track of the
permutation we executed and undo it later. But we should now be able to get back
to sequential access performance.

<figure markdown>
![Image](../figures/permuted_array.png){ width="500" }
<figcaption>
If reading a lot from the same array with the same permutations, go ahead and execute the permutations.
</figcaption>
</figure>

There is a middle ground however, which is if we are just permuting rows. As long as the rows are long,
we should be able to get partially sequential access, at least if we are moving through the elements in
order.

<figure markdown>
![Image](../figures/permuted_rows_array.png){ width="500" }
<figcaption>
Offset some of the cost of permutations, by just permuting rows.
</figcaption>
</figure>

Now, lets try out a simple example! Checkout the code at
```m1_memory_hierarchies/code/permuted_arrays``` or check it out
[online](https://github.com/absorensen/the-real-timers-guide-to-the-computational-galaxy/blob/main/m1_memory_hierarchies/code/permuted_arrays/src/main.rs)

<figure markdown>
![Image](../figures/permuted_arrays_benchmark_0.png){ width="500" }
<figcaption>
Huh, that's weird. There doesn't seem to be much of a difference.
</figcaption>
</figure>

It seems pretty much the same.

<figure markdown>
![Image](../figures/permuted_arrays_benchmark_1.png){ width="500" }
<figcaption>
The differences appear as the cache runs out of space with bigger data sizes.
</figcaption>
</figure>

Once we run out of cache however, the executed permutation is quite a bit faster.
Permuting just the rows can also give quite a performance boost.

### Jagged Arrays
A weird form of array is the jagged array. A 2D matrix can't simply be
expressed as having dimensions NxM, but Nx? or ?xM dimensions. As in N rows, each with their
own individual lengths, or M columns, each with individual lengths. It's a highly
flexible scheme, but unless you are absolutely sure you need it, you should probably avoid it.

In the example below, we attain this complete flexibility by using a vector of vectors,
which as you may recall is really bad for performance.

<figure markdown>
![Image](../figures/naive_jagged_array.png){ width="500" }
<figcaption>
Create a jagged array by using a vector of vectors.
</figcaption>
</figure>

If the difference between the smallest row and the largest row isn't too big,
we can sacrifice a bit of additional memory for allocating all rows as if they had
the same length and keep track of the length of the active sections in each row
in a separate vector.

<figure markdown>
![Image](../figures/jagged_array_size_constrained_aux.png){ width="500" }
<figcaption>
Slightly better now with the data in a single vector.
</figcaption>
</figure>

If we really wanted to compact the jagged array above, we could remove all of the
non-active segments (denoted -1) and use the auxiliary array to indicate where each
new row starts. Just like the first permutation scheme, we are derefercing two pointers
for access.

Finally, we could do all of this, still under the constraint that we have a reasonable
ceiling on the max length of each row, by interleaving the auxiliary array with the data
array.

<figure markdown>
![Image](../figures/jagged_array_size_constrained.png){ width="500" }
<figcaption>
All of the data in a single vector, with the blue values being the amount of active data in the row.
</figcaption>
</figure>

We can do this either compacted or non-compacted.

<figure markdown>
![Image](../figures/jagged_array_size_constrained_compacted.png){ width="500" }
<figcaption>
All of the data in a single vector, with the blue values being the amount of active data in the row. Compacted data.
</figcaption>
</figure>

We've now removed what was a consistent implicit form. We no longer have random access to the row lengths.
We also now have to translate from whatever type is in the data array to valid integers for indexing.
If the data is integers, casting won't be much of a problem, but for floating point numbers we have to be
sure to get it right. If a number is not a whole number we are likely to have the number floored to the
nearest whole number.
Instead we have to go from row length to row length and find out how many indices we have to move forward to
get to the next indicator. As such, to get to the lower right corner element (42), we would first have to read
index 0, jump 4 spots forward to index 4, read the 4, jump 5 spots forward to index 9, and then jump forward
2 elements to get to what in a dense array would be inded [2, 1].

This sort of makes me miss the auxiliary array. We can sum up the jumps to denote where each row starts,
this would allow for compaction of the data while keeping us to just 2 jumps. Note that we now keep track of
the length of each row by taking the difference between the starting index of the row we are looking to find
and the beginning of the next row. Which is also why I have inserted an extra starting index, which points
to the end of the array. Otherwise, we can't get the length of the last row.

<figure markdown>
![Image](../figures/jagged_array_size_compacted_aux.png){ width="500" }
<figcaption>
As we compacted the data, we can keep track of the starting index of each row in an auxiliary array.
</figcaption>
</figure>

Now for a simple performance benchmark. Checkout the code at
```m1_memory_hierarchies/code/jagged_arrays``` or check it out
[online](https://github.com/absorensen/the-real-timers-guide-to-the-computational-galaxy/blob/main/m1_memory_hierarchies/code/jagged_arrays/src/main.rs)

<figure markdown>
![Image](../figures/jagged_arrays_benchmark_0.png){ width="500" }
<figcaption>
Huh, that's weird. There doesn't seem to be much of a difference.
</figcaption>
</figure>

<figure markdown>
![Image](../figures/jagged_arrays_benchmark_1.png){ width="500" }
<figcaption>
There we go, we ran out of cache!.
</figcaption>
</figure>

Note that the only version which is not extremely slow for inserting values is the naive one. But in most other
cases our final optimized version JaggedArraySizeCompactedAux seems to be the winner. It doesn't take a lot of
memory compared to the other solutions and it seems to be in some cases on-par with the fastest
(with a reasonable variance) or the fastest. In most other cases the NaiveJaggedArray seems just fine.
Again, don't overcomplicate things and measure the differences for your case. In any case, you should
avoid a jagged array if you can. And especially the CompactedJaggedArray, which costs the least memory, but
has a catastrophic access time due to needing to accumulate the indices needed to find the row index. Plus,
having the indices be interleaved with the values is problematic as we mix control flow and data, as well
as needing to accomodate casting a data value to an index value. Please don't do that!

### Sparse Arrays
Finally, we have the sparse array. In the case of huge matrices with lots of values we don't care about,
especially 0's, we can use the run-length encoding we just saw to encode values. This usually results
in having to reconstruct where the indices are on the fly. The method below is ok for singular values,
such as a very large matrix with just diagonal values. If we have a band around the diagonal we could modify
the strategy from the last example in the jagged arrays section.  

<figure markdown>
![Image](../figures/sparse_arrays.png){ width="500" }
<figcaption>
A sparse array created with run-length encoding.
</figcaption>
</figure>

For this to be more efficient than the dense version, you usually need at least 90% sparseness, or an array
so big that you are having issues with memory. Sparse matrices also require their own separate implementations
and can be hard to parallelize.

## 3️⃣ Graphs and Trees
Neural networks/computational graphs  
Binary trees  
Octrees  

### Graphs
Uni-directional  
Bi-directional  
DAGs  

### Trees
#### Binary Trees  
#### 🧬 Octrees  
Relevant for all of the specializations that aren't deep learning, especially computer graphics.
åMight be relevant there too.

### Graphs and Trees using Pointers
The version most closely resembling our sketches  
Recursion  
C example  
We now have to keep track of the entire graph in order to free everything correctly or follow all of the
pointers exactly correctly in order to deallocate without a leak.  

### Graphs and Trees using Smart Pointers
Rust examples  
Very basic and non-exhaustive talk about code as both graphs and trees
can be complex in Rust. Especially, if they are mutable after construction - are we doing mutable
references outside of the main struct?

#### Graph
Generally considered hard in Rust specifically, which makes sense, because of the many caveats and
potential issues in graphs. Dynamic graphs especially are problematic and you should consider very carefully
whether all of the logic is correct.
Uni-directional is fine
But how do we keep track of bi-directional?  
Graphs must at the very least be mutable during creation  
Wrap the whole thing in an arena, only use weak pointers  
Have to handle if there is an outside reference using a weak pointer  
Better explanation of [graphs in Rust](https://github.com/nrc/r4cppp/blob/master/graphs/README.md)  
[graphs in Rust using indices](http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/)  

#### Trees
Actually a bit simpler to make as we have fewer cyclical references. If there is a child-to-parent pointer...
use weak.

### Graphs and Trees using Indices
Rust examples  
Connectivity  
Read-only  

## 3️⃣ Garbage collectors
Garbage collection is a way of freeing the programmer of having to deal with which memory is and isn't
relevant. It is usually implemented by most variables (especially the heap allocated ones) being
reference counted or otherwise tracked, which we will see later in the tracing section.
Once a variable is found to no longer be referenced it is either immediately cleaned up or cleaned up
during a garbage collection pass. A full garbage collection pass can be quite expensive, and if the implementation
is not particularly optimized, lock the whole system while it is being performed as to not have memory that has
just been cleaned up referenced anew.

Garbage collectors aren't really that relevant to the rest of the guide,
but if you are coming from Python, C#, Go or Java this section will use some of the concepts
previously introduced on this page to give you a quick perspective to how garbage collectors work.
This post takes a look at 
[how python handles garbage collection](https://stackify.com/python-garbage-collection/)
although, a bit light on the details for the generational garbage collection. In the following
sections I will introduce three different types of garbage collectors, and finally
set you up with a few tricks for working with the garbage collector.

### Reference Counted Garbage Collectors
[Reference counting garbage collection](https://en.wikipedia.org/wiki/Reference_counting) is one of the
simplest forms of dealing with garbage collection. Imagine that there is an ```Rc<T>```, like we saw earlier, wrapped
around every heap-allocated variable. Once the amount of references reaches 0, the object is deallocated. Simple,
can be handled locally, scales well, doesn't burden the entire system with a lockdown to clean up, which makes it
good for real-time systems which need to be responsive at all times and not have noticeable freezes.
What makes it not quite usable is, that it is up to the programmer to not create cyclical references.
Node A and Node B cannot refer to each other without causing a memory leak, despite not being referenced by
anything else. They cannot be cleaned, unless one of the references is a weak reference. Just like the
```Weak<T>``` type we saw in the smart pointer section. But it is up to the programmer to make sure that
the weak references are used correctly throughout the system, which isn't necessarily non-trivial.

### Tracing Garbage Collectors
[Tracing garbage collection](https://en.wikipedia.org/wiki/Tracing_garbage_collection) on the other hand
follows every root, this could for example be the variable holding the pointer to the root node of your graph,
if there even is such a thing, and then following every pointer making sure to mark all the objects it finds along
the way as not being ready for clean-up. This does however require that all the memory is frozen. There can't all
of a sudden be new references to some of the objects or some of them be removed. Once the marking process has
completed, all of the objects are traversed and every object not marked is cleaned up.

Another more sophisticated method, promises better performance by using a white, gray and black marking. All
objects start marked as white, and are then moved to grey, and then finally to black. Objects marked in white
are possibly accessible from roots and are candidates for collection. Gray objects are definitely accessible
from roots and might have pointers to objects marked in white. Black marked objects are definitely accessible
from roots and definitely do not have pointers to the white set.

You can read more about tri-color marking
[here](https://bwoff.medium.com/understanding-gos-garbage-collection-415a19cc485c).

### Generational Garbage Collection
Generational garbage collection is a different technique which sequesters allocated objects into different memory regions.
These regions, usually 3, are based on the age of the object. If an object survives a garbage collection pass it
is promoted from one region to the next, older region. The youngest region will usually be significantly larger
than the two older regions and it is estimated that most garbage collection will happen in the youngest region.
This strategy might not find all unreachable objects, however, and can be supplemented by an occasional
expensive full mark-and-sweep to ensure that no memory leaks go undetected for too long.
For more on
[generational garbage collection](https://en.wikipedia.org/wiki/Tracing_garbage_collection#Generational_GC_(ephemeral_GC))
.

## 3️⃣ Virtualized Memory Hierarchy
A simplified definition of virtualized memory is a single address space that doesn't correspond 1-to-1 to physical
memory. As we have seen earlier in jagged arrays and permuted arrays, if we have all of our data in memory
the caches, compiler and branch prediction take care of hiding memory access latencies, quite a bit,
however, what if we don't have all of our data in main memory?

### Virtualized Memory and Operating Systems
The operating system itself can, and will, [virtualize your memory](https://en.wikipedia.org/wiki/Virtual_memory).
It may at some point decide to spare the main memory, probably because it doesn't have any more, and instead
allocate temporary space on the disk to swap in and out of main memory. This is painfully slow, but happens
seamlessly behind the scenes to be able to continue to allocate more memory for your program. The programmer
does not have to do anything as the virtualization is hidden. Usually, there will be hardware support for the
virtualization with components such as a dedicated memory management unit.

Each process, your program would be its own process, is given its own virtual memory space. Meaning that your
program might see its addresses start in very low numbers despite a number of other processes running concurrently
on your computer. In face, while the address space given to your process might look continuous it is probably
fragmented, scattered across diffent physical locations, but the virtualization makes it appear continuous.
In general, it is a major security risk for programs to read memory outside of the memory
allocated for it. This is also known as a *segmentation fault*. The operating dislikes this concept so much that it
is likely to just kill your program entirely. If you have every programmed C or C++, you have probably tried this.
The virtual memory space allocated for your process, for stuff like heap and stack will typically look like below.

<figure markdown>
![Image](../figures/virtual_heap_and_stack.png){ width="300" }
<figcaption>
The stack and the heap sharing memory in their own virtual address space.
<a href="https://www.cprogramming.com/tutorial/virtual_memory_and_heaps.html">
Image credit</a>.
</figcaption>
</figure>

### Virtualizing Your Own Application
As I just described in the presceding virtualized memory section, the operating system will store temporary data
on the disk if it runs out of space in main memory, keep track of what is not in memory and in disk instead and,
when needed, invisibly load requested data into memory while swapping some other piece of data unto the disk.
But we can make our own virtualized memory too! We could for example have a dataset for training a neural network
that is astronomically big. Terabytes even! We have for some reason decided it is of the utmost importance that we
always random sample the entire dataset. So we randomly pick 20 samples. 4 were already on the GPU, 2 were in main
memory, 4 were on disk and the remaining samples are on the internet. It will be slow as all hell, but that is
something we can optimize too. The easiest would of course be to limit how often we decide to sample the parts
that are on the internet. We could for example choose to download a random block from the internet portion of our
virtualized memory, random sample from that block for a while and then download a new block. We could hide this
by defining data object structs for each sample which have an optional payload, along with a bit of additional
bookkeeping. We could make a sort of address space by keeping the list of samples, which need to be A LOT smaller
than the total data set for this to work, and using heuristics on this list of samples and associated metadata to
optimize our virtualized data set. We could give a priority to each of the samples based on how long ago they were
sampled last and in which block they were located, on which physical memory they were located (the cloud is just
someone else's computer). Optimizing these types of systems can be quite a fun algorithms and systems optimization
process.

### 🧬 Virtualized Rendering
Another use of this is the rendering of data sets too large to fit in a users computer. You preprocess
all of the data you need to visualize into a tree structure and then just keep the tree in memory at all times.
If you then render with progressive rendering, which is where as soon as the camera stands still you render across
multiple frames into the same uncleared buffers, letting the user see progress while the system downloads, unpacks
and renders the entire scene. This also allows for rendering of scenes which are too big to fit in GPU memory or
even main memory.

## 5️⃣ Further Reading
An explanation of memory allocation, stack and heap
[in C](https://cs2461-2020.github.io/lectures/dynamic.pdf).

A more rigorous [explanation](http://eceweb.ucsd.edu/~gert/ece30/CN5.pdf)
of the register, cache, main memory and virtual memory parts of the memory hierarchy.
For even more [virtual memory](http://csapp.cs.cmu.edu/2e/ch9-preview.pdf).

Check out the memory and cache specs for Apple's [M1 series](https://en.wikipedia.org/wiki/Apple_M1).

For an example of coding a [tri-color marking garbage collector](https://sean.cm/a/tricolor-garbage-collector).

For more about
[garbage collection in Python](https://devguide.python.org/internals/garbage-collector/),
[more basic garbage collection in Pyton](https://stackify.com/python-garbage-collection/) or
[garbage collection in Java](https://blogs.oracle.com/javamagazine/post/understanding-garbage-collectors).
