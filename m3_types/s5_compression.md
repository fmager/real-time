# 3️⃣ Compression
Why should you care about compression? In most cases it will be handled for you, but if you are doing a real-time
system, in a lot of cases you will have time for preprocessing. In preprocessing you can compress your data which,
while requiring some encoding/decoding to then use your data, can relax the pressure on bandwidth,
reduce download or upload times and reduce the amount of memory used. It is already heavily used in everything
to do with video and audio.

In compression you have to distinguish between lossy and lossless compression. With lossless compression the
original data is recoverable down to the bit, where as with lossy compression we have some, usually variable,
amount of loss of quality. This quality might not even be perceptible, or you can calculate how much is
acceptable for your situation, allowing you reap a vast amount of memory savings. Choosing the
right compression scheme and transforming your data, with techniques like quantization, packing and sorting, to
maximize the compression scheme's effectiveness often requires domain knowledge, but the results can
be spectactular.

You will find lossy compression in formats like JPEG (2D images) and MPEG (2D videos), using codecs like H.264.
You will find lossless compression in stuff like PNG and ZIP. Some of these are ubiquitous enough that dedicated
hardware is available. For example, a lot of GPU's have dedicated hardware for video encoding and decoding, for
stuff like streaming. Compression times and characteristics vary wildly. Some are really good with text, some
are good with numerical data. Some take a really long time to compress or decompress. Some are even supported
in hardware by GPU's like video encoding/decoding. Knowing how external compression libraries work can help
you do transforms on your own data to maximize the performance of the compression libraries.  

Ok, so what is compression actually? In short, exploiting the structure and repetition of data to encode it in a
smaller representation. This is tightly connected to types, which is why it is in this module. For example, floats
are really hard to compress well, why? Because of repetition. As you might recall, it is generally bad form to
compare floats exactly. Instead you need to take the absolute difference and confirm that the two numbers are
within some threshold distance of each other, not to be considered the same, but similar enough. Compression
thrives on sameness. If we had a row of 1,000 pixels in an image, all of which were the exact same color, we could
instead represent the entire row by the amount of pixels in the row and the value itself. Thus replacing 1,000
values by 2! Think of the savings! This is what is known as run length encoding and is one of the easiest ways
of transforming your data before running it through a more general compression scheme (which might itself use
run length encoding). This also somewhat echoes the various ways we tried to do jagged arrays in ```m1::s0```.

If we wanted to use the run length transformation, we could then in turn begin
transforming our input data (the image) to maximize this similarity. After all, if we had a row of 1,000 pixels
and they all had different values, our simple run-length encoding would balloon up to double the size. We could
make ensure that the values became more similiar. If all of the colors were in 16-bit, we could lower the precision
to 8-bits by either dividing all of the values by 256, or we could introduce a defacto variable precision reduction
by nulling bits from the right in all of the values (I'm assuming integers here) to get the precision we could live
with. If we could at most live with a reduction from 16-bit to 11-bit precision in our colors, we could set the
5 least significant bits in all of the values to 0. We have then effectively increased the likelihood of any
given value being similar to its neighbor. We still have to keep it in memory as 16-bits per element, mind you, but
when compressed/encoded it will take up much less space.

We could take things even further and delta encode all of the values. We do this by having the starting value
be the initial value, and every subsequent value being the difference of that value relative to the preceeding
value. In that case we now have even more numbers that are similar. This does however make the decompression
take longer. You now have to de-delta encode all of the values to use them.

For a brief introduction to both the classic Lempel-Ziv schemes look under
week 10 on this [page](http://www2.compute.dtu.dk/courses/02282/2023/).
Another classic is
[Huffman Encoding](https://web.stanford.edu/class/archive/cs/cs106b/cs106b.1132/handouts/34-Huffman-Encoding.pdf).

## 🧬 Point Cloud Compression
This is mostly for people interested in graphics, I will showcase a few examples of transformations you can do to
compress point clouds, which is based on some real-world experience.

First up, let's set the scene. A user uploads a point cloud to your system. You have time to preprocess the point
cloud and at some point the user will see that the point cloud has been processed and is ready for viewing. The
point clouds can be massive. Much bigger than can fit in the VRAM of a GPU, or even the RAM of the users system.
Progressive rendering is chosen to render. In that case the frame buffer is cleared every time the user moves the
camera and the relevant parts of the scene is accumulated into the frame buffer, using the depth buffer to
discriminate between which points should be in view. Frustum culling is also implemented.

To prioritize which points to render first, and which points to not render at all, we need a data structure
which lets us make decisions about which packages to even download to the user system before we download them.
We choose a type of level-of-detail octree. The regular structure of the octree allows us to make some
nice decisions later on, which are encoded implcitly by our knowledge of the chosen structure. A level-of-detail
structure for point clouds means that the top node of the octree will cover the entire scene. We will choose
some maximum amount of points which can reside in this root node. Those points have to cover as much of
the scene as possible, preferably in as visually nice a way as possible. The fewer nodes at as low
levels-of-detail as we can get away with, will allow for fast navigation of the scene so the user can find their
angle and then set the camera to rest so we can accumulate the relevant points and converge on the "true" image.



Bit nulling!  

As found in [this paper](https://www.cg.tuwien.ac.at/research/publications/2021/SCHUETZ-2021-PCC/) by Schûtz,
et al., sorting the points, which we can do in preprocessing, can not only have an effect on the effectiveness
of compression, but also the effectiveness of rendering.

## 5️⃣ Additional Reading
[Information Theory](https://en.wikipedia.org/wiki/Information_theory)  
[Data Compression](https://en.wikipedia.org/wiki/Data_compression)  
[Lempel-Ziv 77 & 78](https://en.wikipedia.org/wiki/LZ77_and_LZ78)  
[Huffman Coding](https://en.wikipedia.org/wiki/Huffman_coding)  

[Zip](https://en.wikipedia.org/wiki/ZIP_(file_format))  
Gzip - [HTTP Compression](https://en.wikipedia.org/wiki/HTTP_compression)  

Lots of interesting things are happening in real-time decompression
[using neural networks](https://research.nvidia.com/labs/rtr/neural_texture_compression/s).
