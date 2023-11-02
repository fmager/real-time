# 👨🏼‍💻 Exercises
## Pen & Paper Exercises
### Vector Memory
Write out the stack and heap memory of THIS sequence of vector operations.
You can represent unitialized memory with a *.  
The vector must double its capacity and copy over each element once it gets a push operation while at capacity.
The vector must resize to half its size and copy over each active element once it gets a pop operation while at
quarter capacity.

* Create new vector of capacity 4
* .push(5)
* .push(2)
* .push(1)
* .pop()
* .push(42)
* .push(350)
* .push(1337)
* .push(1)
* .push(2)
* .push(5)
* .push(10)
* .pop()
* .pop()
* .pop()
* .pop()
* .pop()
* .pop()
* .pop()
* .pop()

Barring a few errors here and there this should be a simple exercise. Except.
Are you sure you got all of the undefined (*) values right?

Which common operation could have replaced the last sequence of .pop() operations
in the case where you wouldn't be using the popped values?

### Linearized indexing  
You have an array of dimensions ```MN``` in a linearized array ```data```, write pseudo code that iterates
through all data elements using two for-loops and doubles the value in place.

You have an array of dimensions ```MNK``` in a linearized array ```data```, write pseudo code that iterates
through all data elements using three for-loops and triples the value in place.

### Killing the garbage collector
<figure markdown>
![Image](../figures/harass_the_garbage_collector.png){ width="500" }
<figcaption>
Where would you add a pointer to hurt garbage collection the most?
</figcaption>
</figure>

Adding which pointer would result in the most nodes not being properly garbage collected?  
If the garbage collector implements cycle detection to depth 2 adding which pointer would break it?
The nodes can't point to themselves.  

<figure markdown>
![Image](../figures/general_graph.png){ width="500" }
<figcaption>
How could you make this sort of general graph, with very few restrictions, safe for garbage collection?
</figcaption>
</figure>

In the case of the multiply connected nodes, can you come up with a structural solution which allows
us to make arbitrary graphs in a garbage collected setting or safe in a C++/Rust setting?

## Programming
Extend the computational graph with an inplace operation for the ReLU operator (only for the non-fused ReLU)

The following list is sorted by expected complexity - do at least 1

* Add reusable buffers to the computational graph system (for the intermediate activations)
* Implement a shader cached version of the immediate mode GPU operators and add it to the benchmark
* Implement a version of the linear layer functions which uses shared memory and tiling
* Change the ```Tensor2DGPU``` to have switchable access details on its buffers. It should be able to
accomodate some tensors being exclusively read-only. Do you see any performance differences for whether
they are read-only or not?
* Implement the tree reduction version of the sum function and add it to the softmax function.
Also compare the single pass and the tree reduction performance graphs. [Reference](https://developer.download.nvidia.com/assets/cuda/files/reduction.pdf)
* Implement a max pooling operator, as well as fusing with ReLU, in all levels and implement tests
* Implement a convolution operator, as well as fusing with ReLU, in all levels and implement tests
