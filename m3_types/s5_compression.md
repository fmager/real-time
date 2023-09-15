# 3️⃣ Compression
Why should you care about compression? In most cases it will be handled for you, but if you are doing a real-time
system, in a lot of cases you will have time for preprocessing. In preprocessing you can compress your data which,
while requiring some decoding to then use your data, can relax the pressure on bandwidth, reduce download times
and reduce the amount of memory used. It is already heavily used in everything to do with video and audio.

In compression you have to distinguish between lossy and lossless compression. With lossless compression the
original data is recoverable down to the bit, where as with lossy compression we have some, usually variable,
amount of loss of quality. This quality might not even be perceptible, or you can calculate how much is
acceptable for your situation, allowing you reap a vastly increased amount of memory savings. Choosing the
right compression scheme and transforming your data, with techniques like quantization, packing and sorting, to
maximize the compression schemes effectiveness often requires domain knowledge, but the results can be spectactular.

You will find lossy compression in formats like jpeg (2D images) and mpeg (2D videos), using codes like H.264.
You will find lossless compression in stuff like png and zip. Most of these are ubiquitous enough that dedicated
hardware is available. For example, a lot of GPU's have dedicated hardware for video encoding and decoding, for
things like streaming.

Quantization and compression can help us make much smaller networks  
Compression times and characteristics vary wildly. Some are really good with text, some are good
with numerical data. Some take a really long time to compress or decompress.  
Some are even supported in hardware by GPU's like video encoding/decoding.  
Knowing how external compression libraries work can help you do transforms on your own
data to maximize the performance of the compression libraries.  

Ok, so what is compression actually? In short, exploiting the structure and repetition of data to encode it in a
smaller representation. This tightly connected to types, which is why it is in this module. For example, floats
are really hard to compress well, why? Because of repetition. As you might recall, it is generally bad form to
compare floats exactly. Instead you need to take the absolute difference and confirm that the two numbers are
within some threshold distance of each other, not to be considered the same, but similar enough. Compression
thrives on sameness. If we had a row of 1,000 pixels in an image, all of which were the exact same color, we could
instead represent the entire row by the amount of pixels in the row and the value itself. Thus replacing 1,000
values by 2! Think of the savings! This is what is known as run length encoding and is one of the easiest ways
of compressing. This also somewhat echoes the various ways we tried to do jagged arrays in ```m1::s0```.

If that is the compression scheme we have chosen (there are a lot to choose from), we could then in turn begin
transforming our input data (the image) to maximize this similarity. After all, if we had a row of 1,000 pixels
and they all had different values, our simple run-length encoding would balloon up to double the size. We could
make ensure that the values became more similiar. If all of the colors were in 16-bit, we could lower the precision
to 8-bits by either dividing all of the values by 256, or we could introduce a defacto variable precision reduction
by nulling bits from the right in all of the values (I'm assuming integers here) to get the precision we could live
with. If we could at most live with a reduction from 16-bit to 11-bit precision in our colors, we could set the
5 least significant bits in all of the values to 0. We have then effectively increased the likelyhood of any
given value being similar to its neighbor.

We could take things even further and delta encode all of the values. We do this by having the starting value
be the initial value, and every subsequent value being the difference of that value relative to the preceeding
value. In that case we now have even more numbers that are similar. This does however make the decompression
take longer. You now have to de-delta encode all of the values to use them.

For a brief introduction to both the classic Lempel-Ziv schemes look under
week 10 on this [page](http://www2.compute.dtu.dk/courses/02282/2023/) for the
Algorithms for Massive Data Sets course.
Another classic is
[Huffman Encoding](https://web.stanford.edu/class/archive/cs/cs106b/cs106b.1132/handouts/34-Huffman-Encoding.pdf).

## 🧬 Point Cloud Compression
This is mostly for people interested in graphics, I will showcase a few examples of transformations you can do to
compress point clouds, which is based on some real-world experience.  
Bit nulling!  
Sorting for better rendering  

## 5️⃣ Additional Reading
[Information Theory](https://en.wikipedia.org/wiki/Information_theory)  
[Data Compression](https://en.wikipedia.org/wiki/Data_compression)  
[Lempel-Ziv 77 & 78](https://en.wikipedia.org/wiki/LZ77_and_LZ78)  
[Huffman Coding](https://en.wikipedia.org/wiki/Huffman_coding)  

[Zip](https://en.wikipedia.org/wiki/ZIP_(file_format))  
Gzip - [HTTP Compression](https://en.wikipedia.org/wiki/HTTP_compression)  

Lots of interesting things are happening in real-time decompression
[using neural networks](https://research.nvidia.com/labs/rtr/neural_texture_compression/s).
