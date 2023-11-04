# 3️⃣ Branchless Programming
For this next section, I am going to be cheating a little bit.
I am going to introduce to you a concept that is mostly about optimizing for branch prediction and cache lines.
These things aren't technically concurrency. But they do lead to an introduction of single instruction,
multiple data (SIMD), which is... I don't know... concurrency's cousin or something. It also wasn't immediately
obvious where else this would go. Anyways, branch prediction, branchless programming, data oriented programming and
SIMD, here we go!

## Branch Prediction
While the speed at which CPUs and memory can execute hasn't risen aggresively in these last few years, that doesn't
mean they can't get smarter. One such effort is the hardware supporting branch prediction. Loading a value
from memory can take quite a while, in the mean time the CPU can easily do other stuff or find our which piece
of memory to load next. Below you'll see a small example of a pipelined architecture -

<figure markdown>
![Image](../figures/pipeline_architecture.png){ width="600" }
<figcaption>
Pipelined Hardware Architecture
<a href="https://microchipdeveloper.com/32bit:mz-arch-pipeline">
Image credit </a>
</figcaption>
</figure>

This features several sections executing different commands at the same time and then storing the result in a
register, to be moved on to the next part of the pipeline. This is vastly more efficient, but requires that the
hardware is able to predict which execution path should be followed. In the case of picking the wrong path,
all instructions which were part of the wrong path have to be flushed from the pipeline, and the new execution
path has to be begun. If you have a hard time picturing the scenario, let me paint you a picture with code -

```rust
for index in 0..data.len() {
    if index % 1337 == 0 && index != 0 {
        data[index] -= 1; // Path A
    } else {
        data[index] += 1; // Path B
    }
}
```

Below you can see a more abstract representation of pipelined instructions -

<figure markdown>
![Image](../figures/pipelined_instructions.png){ width="600" }
<figcaption>
Pipelined Instructions
<a href="https://microchipdeveloper.com/32bit:mz-arch-pipeline">
Image credit </a>
</figcaption>
</figure>

If you would like to know more about the hardware side, I do recommend you check out these
[slides](https://ics.uci.edu/~swjun/courses/2023F-CS250P/materials/lec5.5%20-%20Fast%20and%20Correct%20Pipelining.pdf)
from University of California, Irvine for a cursory glance at the fairly complex topic of the hardware involved in
branch prediction.  

Going back to the code from earlier, you can see, any branch predictor worth its salt would predict that path
B would be executed. In a few cases path A will be executed instead, but will be vastly more expensive.

## Branchless Programming
Again, please checkout the
[slides](https://ics.uci.edu/~swjun/courses/2023F-CS250P/materials/lec5.5%20-%20Fast%20and%20Correct%20Pipelining.pdf)
for an overview of various hazards with code examples. But a few of the basic highlights are control flow, through
short circuiting, unrolling of for-loops (your compiler will often do this automatically), reformulation of
branching through arithmetic.

The circuiting boolean operators ```a && b``` and ```a || b``` are used everywhere. The short circuiting part
means that because both ```a``` and ```b``` need to be true in order for ```&&``` to evaluate as ```true```, if
```a``` evaluates as false, ```b``` need not be evaluated. Do you see the problem?

It's a branch! Supposing that ```a``` and ```b``` aren't too expensive to evaluate we can reduce the amount of
branching in our code by evaluating both. One way to do so could be -

```rust
if 0 < (a as u8 & b as u8) {
```

or even -

```rust
if 0 < (a as u8 * b as u8) {
```

For ```||``` options can include -

```rust
if 0 < (a as u8 | b as u8)
```

or -

```rust
if 0 < (a as u8 + b as u8)
```

Another way to remove a potential if-statement branch could be to multiply by 0's and 1's the data we might
like to use, by reformulating our code arithmetically -

```rust
fn main() {
    let data: Vec<f32> = vec![0.5; 5];

    let a: bool = true;
    let a_arithmetic: f32 = a as u8 as f32;
    let b: bool = false;
    let b_arithmetic: f32 = b as u8 as f32;
    
    let calculated: f32 = a_arithmetic * data[0] + b_arithmetic * data[1];
    println!("Calculated: {}", calculated);
}
```

As with everything else, this is something you should benchmark before deciding. In terms of readability it is
usually harder to read, so just do it when you need better performance, after asserting correctness first,
of course.

Loop unrolling is the process of doing more work per loop, which also reduces the relationship between actual
work done and administrative control flow work. This unroll will happen a certain amount at a time. Let's
take a quick look at a loop unroll transformation, with an unroll of 4.

```rust
let mut data: Vec<f32> = vec![0.2; 19];
for index in 0..data.len() {
    data[index] *= 2.0;
}
```

And now for the unrolled version -

```rust
    let unroll_size: usize = 4;
    let mut data: Vec<f32> = vec![0.2; 19];
    let full_iterations: usize = data.len() / unroll_size; // 4 = 19 / 4
    
    for index in 0..full_iterations {
        let index: usize = index * unroll_size;
        data[index + 0] *= 2.0;
        data[index + 1] *= 2.0;
        data[index + 2] *= 2.0;
        data[index + 3] *= 2.0;
    }
    
    for index in (full_iterations * unroll_size)..data.len() {
        data[index] *= 2.0;
    }
```

Of course, the tail iterations in the second for-loop won't be as fast the main loop, but again, this usually
something the compiler will do for you in release mode.

## Data-oriented Design
We can take the branchless thinking, and add in optimization for cache lines, from the micro to the macro and
make it part of the way we formulate our data structures and code. This we will use things like sorting and
structuring our data into bigger single objects, while at the same time pulling them apart field by field.
For this, we will take a look at data-oriented design.

[Array-of-Structures-of-Arrays](https://www.rustsim.org/blog/2020/03/23/simd-aosoa-in-nalgebra/).
AOS SOA AOSOA - cache lines  
A macro-ish perspective  

## SIMD
Sorting functions.
Find the code in ```m2_concurrency::code::sorting_functions``` or
[online](https://github.com/absorensen/the-guide/tree/main/m2_concurrency/code/sorting_functions).  

<figure markdown>
![Image](../figures/sorting_functions_benchmark.jpg){ width="500" }
<figcaption>
Benchmark for the program in ```m2_concurrency::code::sorting_functions```.
This benchmark was run on my laptop boasting an Intel i7-1185G7, 3.0 GHz with 32GB of RAM. The operating system was
Windows 10. The L1/L2/L3 caches were 320 KB, 5 MB and 12 MB respectively. The CPU supports
Intel® SSE4.1, Intel® SSE4.2, Intel® AVX2, Intel® AVX-512.
</figcaption>
</figure>

Find the code in ```m2_concurrency::code::sphere_intersection``` or
[online](https://github.com/absorensen/the-guide/tree/main/m2_concurrency/code/sphere_intersection).

<figure markdown>
![Image](../figures/sphere_intersection_benchmark.jpg){ width="500" }
<figcaption>
Benchmark for the program in ```m2_concurrency::code::sphere_intersection```.
This benchmark was run on my laptop boasting an Intel i7-1185G7, 3.0 GHz with 32GB of RAM. The operating system was
Windows 10. The L1/L2/L3 caches were 320 KB, 5 MB and 12 MB respectively. The CPU supports
Intel® SSE4.1, Intel® SSE4.2, Intel® AVX2, Intel® AVX-512.
</figcaption>
</figure>

Check your system for SIMD hardware.  
Autovectorization  
Explicit SIMD programming  
Doesn't work if you are memory bound, SIMD won't magically make your memory bandwidth increase.  

## Additional Reading
A nice introduction video to [branchless programming](https://www.youtube.com/watch?v=g-WPhYREFjk) by Fedor Pikus.  
A nice introduction video to [SIMD](https://www.youtube.com/watch?v=x5tK5ET6Q1I) by Guillaume Endignoux.  
Mike Acton on [Data-oriented Design](https://www.youtube.com/watch?v=rX0ItVEVjHc).    
Wiki on [branch prediction](https://en.wikipedia.org/wiki/Branch_predictor).  
Wiki on [instruction pipelining](https://en.wikipedia.org/wiki/Instruction_pipelining).  
Slides on [instruction pipelining](https://web.eecs.utk.edu/~mbeck/classes/cs160/lectures/09_intruc_pipelining.pdf)
from The University of Tennessee, Knoxville.  

### 🧬 Shader Execution Reordering
[Megakernels Considered Harmful](https://research.nvidia.com/sites/default/files/publications/laine2013hpg_paper.pdf)  
[Wavefront Path Tracing](https://jacco.ompf2.com/2019/07/18/wavefront-path-tracing/)  
[Shader Execution Reordering][1]  

[1]: https://developer.nvidia.com/blog/improve-shader-performance-and-in-game-frame-rates-with-shader-execution-reordering/  
