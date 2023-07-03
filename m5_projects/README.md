# Projects
This whole module is for levels 3 and 4

## How to create real time systems, good frameworks for the different fields and project proposals

* Starting with a simple prototype
* Identify your components
* Single threaded correct implementation -> Testing to avoid regression
* Optimize

### Tips and tricks in real time systems

* memcpy
* Hot loops, event loops
* Allocations in a hot loop
* System calls - hoist out of the hot loop
* Logging and printing
* Bindings - PyO3 and cxx
* Walk, don't run, testing for correctness before optimization
* Don't use abbreviations
* Don't use postfix incrementation++
* When to care about software engineering and when to care about performance
* Don't use a string key/identifier or integer, when a type safe enum will do the job
* Hard coding types
* Cognitive load, and delaying errors to after the first draft - deliberate development vs. debugging
* Prefer stateless programming, minimize stateful programming (functional inspiration)
* Implicit casting
* Compression
* Know your system - mobile, laptop, desktop, integrated memory, which GPU
* Use version control even for solo development
* Am I copying/cloning things that don't need to be copied?
* Check/validate everything before the hot loop
* Anything that can be immutable, should be immutable - aliasing!
* Testing and Seeding RNG's
* [Faster RNG](https://youtu.be/5_RAHZQCPjE)
* Timing real-time systems and how to escape or offload compute
* Multi-resolution computing for making your real-time target
* Pressure testing and worst cases

### Components - libraries/frameworks

[blessed](https://blessed.rs/crates)  
[rayon](https://github.com/rayon-rs/rayon)  
[egui](https://github.com/emilk/egui)  
[wonnx](https://github.com/webonnx/wonnx)  
[tch](https://github.com/LaurentMazare/tch-rs)  
[winit](https://github.com/rust-windowing/winit)  
[cv](https://github.com/rust-cv/cv)  
[ultraviolet](https://github.com/fu5ha/ultraviolet)  
[arewelearningyet](https://www.arewelearningyet.com/neural-networks/)  
[burn](https://github.com/burn-rs/burn)  

### Specializations - Project proposals

* Virtual 3D scanner for a point cloud dataset
* EEG system
* Change the latent variables in a network using GUI, optimize the network
* Point cloud renderer
* Real-time style transfer on a web cam feed
* Rendering fractals influenced by a web cam feed
* Eye tracking -> Present to screen and read from web cam ->
feature extraction -> classifier -> intervention signal ->
reading app (Wolfgang Fuhl, PISTOL, fixation detection)
* Bird classification from sound / Real-time classification of sound (Xeno-canto database)
* Who is talking? Real-time classification of sound
* Are you dyslexic? Eye tracking classifier
* Cognitive load tracker - Eyes & pupil dilation and online estimation of
signal strength (pupils vs. sound for the hearing impaired)

### What makes for a good project?

* What is your concept/project?
* Which concepts from the previous material do you think
are relevant to your project and why?
* Preprocessing your data?
* How do you adapt to your chosen/available platform?
* Which libraries did you choose for this problem?
* How fast did you get to your minimum viable product?
* Which steps did you take from there and why?
* How did you determine which parts of your system to optimize?
* What else would you like to do with your system?
