# 2️⃣ Computational Graphs
Conveying the importance of computational graphs to people who were probably using Python to
program neural networks was the motivation for doing this whole thing in the first place.
So now let's get to business.

Computational graphs are more or less a way to communicate the flow of your program.
It can allow a library or a framework to keep data at various levels of the memory hierarchy.
It can allow it to check that all of the dimensions fit for the data,
it can make assumptions about fusing nodes (combining them), remove redundancies and removed unused elements.  

Let's take a look at this defined network from PyTorch's own
[documentation](https://pytorch.org/tutorials/recipes/recipes/defining_a_neural_network.html).

```python
class Net(nn.Module):
    def __init__(self):
      super(Net, self).__init__()
      self.conv1 = nn.Conv2d(1, 32, 3, 1)
      self.conv2 = nn.Conv2d(32, 64, 3, 1)
      self.dropout1 = nn.Dropout2d(0.25)
      self.dropout2 = nn.Dropout2d(0.5)
      self.fc1 = nn.Linear(9216, 128)
      self.fc2 = nn.Linear(128, 10)

    # x represents our data
    def forward(self, x):
      # Pass data through conv1
      x = self.conv1(x)
      # Use the rectified-linear activation function over x
      x = F.relu(x)

      x = self.conv2(x)
      x = F.relu(x)

      # Run max pooling over x
      x = F.max_pool2d(x, 2)
      # Pass data through dropout1
      x = self.dropout1(x)
      # Flatten x with start_dim=1
      x = torch.flatten(x, 1)
      # Pass data through ``fc1``
      x = self.fc1(x)
      x = F.relu(x)
      x = self.dropout2(x)
      x = self.fc2(x)

      # Apply softmax to x
      output = F.log_softmax(x, dim=1)
      return output
```

In PyTorch the user does not fully create a graph, but if the user makes sure ```x``` is on the
GPU by calling ```x.to_device()``` all of the functions will be executed on the GPU until the
output is transferred back to the CPU. This reactive paradigm might be part of why the first
complete iteration of a PyTorch training loop will be signifcantly slower than the subsequent
loop. Not to mention all of the allocations behind the scenes for backpropagation.

If you use ```torch.compile()``` it will do something called tracing behind the scenes.
You don't need to worry about the specifics, but just know that it creates a computational graph
from the Python code above and then optimizes that code to run faster and/or use less memory.

So why does it need the graph? That is something the rest of this module will try to answer,
along with a really basic introduction something called fusion, where layers can be combined
to be more efficient.

## What is a graph?
A graph is [a type of structure](https://en.wikipedia.org/wiki/Graph_(discrete_mathematics)),
for our needs, a data structure. In s0 there is a more elaborate examination of the concept for level 3.
So for right now, just give the link to graph's wiki page a quick look. You should get the gist just by
looking at a few of the images.

Once you have done this, just know, that the rest of this module won't actually use a real graph.
The graph we will concern ourselves with will be defined as being one way, unidirectional, and each
node can point to at most one other node. This reduces the entire graph to just being a list of operations
which will be executed sequentially.

## The network we want to support
For illustrating how computational graphs can benefit your code we don't really need to support
a lot of operators. We need transfers to and from the GPU (eventually), a linear operator
(matrix-matrix multiplication followed by an addition), a ReLU operator (single call to a max function with 0)
and a softmax operator. The softmax operator is the most complex part, don't worry I will show you some
CPU code that is fairly easy to understand. The GPU version gets a bit complicated and is only constructed
on a fairly simplistic version.

<figure markdown>
![Image](../figures/basic_graph_operations.png){ width="300" }
<figcaption>
Our minimal computational graph example will only contain these 5 operations.
</figcaption>
</figure>

For the computation graphs we will use, note that due to the constraints of transfer first and last, and softmax
next to last, the only difference is the amount of linear and ReLU layers. Eventually, when we look
at fusion, new operators will be produced, linear-ReLU and linear-ReLU-softmax. We will only use
32-bit floating point, ```f32``` in Rust, as the data type.

<figure markdown>
![Image](../figures/example_computational_graph.png){ width="300" }
<figcaption>
An example computational graph.
</figcaption>
</figure>

The code for the rest of the module can be found at ```m1_memory_hierarchies/code/computational_graphs/``` or
[online](https://github.com/absorensen/the-real-timers-guide-to-the-computational-galaxy/tree/main/m1_memory_hierarchies/code/computational_graphs).

## What's in a tensor
First of all we are going to start on the CPU.
We are going to create a data type which will hold the data our operators consume on the CPU.
Let's call it ```Tensor2D```. Our 2D tensor will actually be a simple piece of 1 dimensional memory under the
hood and we will keep track of the number of rows and columns to find out how to access each piece of data.
If you are in the root directory for ```computational_graphs``` go to ```src/shared/tensor_2d.rs``` or
[online](https://github.com/absorensen/the-real-timers-guide-to-the-computational-galaxy/blob/main/m1_memory_hierarchies/code/computational_graphs/src/shared/tensor2d.rs).

Start by taking a look at the definition of the ```Tensor2D``` struct at the very top. The ```derive``` stuff
at the top is asking some macros to automatically implement (derive) some traits (interfaces and behavior)
automatically. ```Clone``` means we can call a Tensor2D element as below -

```rust
let some_tensor: Tensor2D = Tensor2D::new(0.1, 8, 8);
let copy_of_some_tensor: Tensor2D = some_tensor.clone();

```

This creates a complete and total copy of ```some_tensor```. If we manipulate or move ```some_tensor```,
```copy_of_some_tensor``` will not be affected as they no longer have anything to do with each other.

Next, take a look at the ```new``` function. In it we create a new ```Tensor2D``` by creating a new
```Vec<f32>``` with size ```row_count*column_count```. Each element is given a value of ```scale*index```.
This is just for testing purposes so I found it useful for this to not be all zeros and not all random numbers.
This allows us to verify that the GPU implementations are functionally equivalent to the CPU implementations.

We don't need to otherwise relate to the values of ```row_count``` and ```column_count```. Even if we implement
a two dimensional structure on top of a piece of one dimensional memory, when we are iterating through all
elements, such as we do when setting all of the elements to some value or accumulating the sum of all elements
we can do away with the two dimensional stuff. Keeping up that illusion unneccesarily induces extra cost
in the form of more time and code spent on control flow statements like ```for-loops``` and ```if-statements```.

## Implementing operators
In this section I will be going through various implementations of the three operators and their fused variants
and show benchmarks to show you how big of a performance impact these sort of first guess optimizations can have
even without profiling or microoptimizations.

### Linear
There's some dimension checking functions, you can just ignore those. They use ```debug_assert``` statements
to raise an error if the dimensions of the tensors given to a linear layer function don't match. ```debug_assert```
is the same as an ```assert``` statement, except it is only run in debug mode. I did it this way incur only a small
hit to performance. You probably passed the ```linear_layer``` function on the way down, it just acts as a wrapper
around the ```linear_layer_preallocated``` function. If you haven't already allocated a tensor to use as output,
it will make one for you. If you do this a lot however, such as in a loop, you should be using the preallocated
version to not have memory allocations in your loops.

Finally, let's go down to the ```linear_layer_preallocated``` function. There are three main sections. One is
the call to the ```debug_assert``` function from earlier, to check for valid input and output dimensions, the
second is the matrix-matrix multiplication which needs three whole for-loops and finally the bias section. Note
the use of linearized accesses, if you need a reminder what that is all about, go back to m1::s0::The Vector.

It's not too bad, but we could do better, although we won't do more efficient implementations of matrix-matrix
multiplication, note that the read accesses of the weights tensor is strided. We could have implemented that some
tensors could be transposed, but you get the point. So we have a triple for-loop and a double for-loop in our
linear operator. Try to think, based on the contents of the last couple of sections, what would be good first
optimizations for this function?

While you think about that in the back of your mind, I'll take a quick detour to introduce a very simple but
finnicky concept - inlining!

Inlining in Rust, and most other languages, is done via an annotation to the compiler. It is usually more of a hint
or request than an actual instruction. In Rust it looks like the derivation of traits we saw earlier -
```#[inline(always)]```. In that case it actually is more of a command. There's other variants you can put inside
the parantheses like ```#[inline]```, which is more of a suggestion, or ```#[inline(never)]```. Inlining
is basically taking all calls to that function and substituting it with the code from the function. This is
largely good for very small functions, such as if we made a function for making our linearized array accesses
prettier to look at, but for large functions it either does nothing or makes the performance worse. So, in general,
unless you were trying to examine the concept of inlining like we are now, you should stick with ```#[inline]```
and suggest inlining to the compiler, without demanding it. The compiler is pretty smart and will usually figure
out what is best. As you will see in the function ```linear_layer_preallocated_inline```, the function itself
is not different in any way.

Next up is the function ```linear_layer_local_accumulation```. Now it's memory hierarchy time! While I didn't
get rid of the strided memory access of the weights tensor, I elected to not accumulate the result of each
output element directly in the output tensor. Instead I accumulate in a local variable, which will hopefully
be kept in a register.

Think back! Where is the register located? And why can it be problematic to accumulate in the output tensor?

The bias is the same. But that does mean we are writing to the same output element twice. Let's move the bias
calculation into the matrix-matrix multiplication loop. ```linear_layer_optimized``` moves the bias to just
before the writing of the accumulated result to the output tensor. If the bias calculation was a lot larger, this
might not be faster. In some cases for highly complex loops, it can instead make sense to separate them into
more loops, which is a process called ```loop fission```. I have only used a small subset of loop optimizations,
but you can read about more ways of optimzing a loop [here](https://en.wikipedia.org/wiki/Loop_optimization).

Ok, so try and run the code locally! To begin with go to the file ```src/lib.rs```. Comment out all
of the lines within and including the ```if configuration.compatible_gpu_found {``` line. Then in your terminal
navigate to the root folder, the one containing the ```src``` folder, and write ```cargo run --release```. Your
computer will now run a bunch of benchmarks relevant to the rest of this section. You can find the output
in ```computational_graphs/outputs/benchmarks/stack```. The one that should have been generated on your computer
that we want to look at now is called ```linear_layer_cpu_benchmark_stack.png```. If you weren't able to run
it locally, don't worry, I got you covered!

<figure markdown>
![Image](../figures/linear_layer_cpu_benchmark_stack.png){ width="800" }
<figcaption>
This benchmark was run on my laptop boasting an Intel i7-1185G7, 3.0 GHz with 32GB of RAM. The operating system was
Windows 10. The L1/L2/L3 caches were 320 KB, 5 MB and 12 MB respectively.
</figcaption>
</figure>

The graph is quite high resolution to allow you to zoom in. The X-axis is the size of the tensors. Only NxN matrices
are used. The Y-axis is time in nanoseconds averaged over 10 runs, which is a bit on the low side. Note the how the
lines are piece-wise linear. There are two points where all of the lines get quite a bit slower and scale worse
with the size of the tensors. Why do you think that is?

You guessed it! You are seeing the size of the tensor becoming too big for the different caches! It looks like the
last bend happens at 4096 elements. This corresponds to a 64x64 matrix. 4096 elements of 32-bits, or 4 bytes, each
corresponds to 16384 bytes, or 16 KB. We have 4 of these tensor structs, so we should have total allocations of
64 KB just for that data and then add in all of the other memory used by the application and everything else
running on the laptop at the time.

This might be a good time to experiment with changing the values in ```lib.rs``` for

```rust
let loop_count: usize = 10;
let loop_range: Vec<usize> = (2u32..8u32).map(|x| 2usize.pow(x)).collect();
```

```loop_count``` is how many measurements are made per data point. ```loop_range``` is a vector of matrix sizes.
In this case, the first element is 4, so the first measurement will be done with matrices sized 4x4.
Currently, it takes a range between 2 and 8 (not including 8) and yields a vector of sizes of 2^n.
So 4, 8, 16, 32, 64, 128. If you wanted all values in the range of 0 to 100 you could write

```rust
let loop_range: Vec<usize> = (0..101).collect();
```

You can zoom in on these parts of the graph by modifying ```lib.rs``` to just test values in these
interesting ranges. Like right around the size of the last bend.
Another thing to note is that only the versions of the linear operator that uses local accumulation
significantly outperform the naive version. One surprise is that keeping the bias outside of the
matrix-matrix loop, is better performing than moving the bias in. Sometimes it really is better to
keep things simple. So from now on, the
```linear_layer_local_accumulation``` version will be the preferred one.

### ReLU

### Softmax

### Fused

## Data dependencies and control dependencies
Working on a graph
Contatenation (multiple writes to the same node)
Do the dimensions fit

## Testing the correctness of the nodes
Testing in Rust

_________________

## 3️⃣ Compiler verifications and the restrict keyword
Perspective back to aliasing and graphs

## 3️⃣ Intermediate representations

## 3️⃣ Graph representations

## 🧬3️⃣ Graphs in Graphics/GPU Programming
Computational graphs are even making their way into the way you can program the GPU!
Ways to define computational graphs have been added to
[DirectX12](https://devblogs.microsoft.com/directx/d3d12-work-graphs-preview/)
and [Vulkan](https://gpuopen.com/gpu-work-graphs-in-vulkan/).
This development seems to be lead by game and graphics workloads being increasingly
compute shader driven.
