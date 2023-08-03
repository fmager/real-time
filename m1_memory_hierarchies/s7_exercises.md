# 4️⃣ Exercises
## Pen & Paper Exercises
Speak to a classmate about your solutions.
Which pointer kills the reference counting garbage collector?
If the garbage collector implements cycle detection to depth X adding which pointer would break it?
Weak pointers.
Write out the memory of THIS sequence of vector operations.
N dimensional indexing in 1D array

## Programming
Extend the computational graph with an inplace operation for the ReLU operator (only for the non-fused ReLU)

The following list is sorted by expected complexity - do at least 1

* Implement a version of the linear layer functions which uses shared memory and tiling
* Add reusable buffers to the computational graph system
* Change the ```Tensor2DGPU``` to have switchable access details on its buffers. It should be able to
accomodate some tensors being exclusively read-only. Do you see any performance differences for whether
they are read-only or not?
* Implement the tree reduction version of the sum function and add it to the softmax function.
Also compare the single pass and the tree reduction performance graphs. [Reference](https://developer.download.nvidia.com/assets/cuda/files/reduction.pdf)
* Implement a max pooling operator, as well as fusing with ReLU, in all levels and implement tests
* Implement a convolution operator, as well as fusing with ReLU, in all levels and implement tests
