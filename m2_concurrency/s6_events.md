# 2️⃣ Events
[Events](https://en.wikipedia.org/wiki/Event_(computing)) are, well, an event. Something happens, and the event
is processed somehow, by one or more actors, reacting to that event. Usually, events are asynchronous (and often
generated from user actions) in origin and are processed synchronously. Distributed architectures can also be based
on events. There is even a type of system architecture based around events, called
[event driven architecture](https://en.wikipedia.org/wiki/Event-driven_architecture), which I won't be going
into at all. I will be focusing on events in the context of real-time systems, usually on a desktop.

If we import some event library, we could generate events, for example every time a user clicks the left mouse
button, and then any actor subscribing to that event would receive the signal that that event had happened.
Then each actor might each have their own way of handling such an event. Multiple actors can be subscribed to the
same event, sort of like the reverse of the multiple producer, single consumer channel we looked at earlier.
A single producer for an event, with potentially multiple not consumers, but listeners.
This is a very reactive way of handling events. Essentially, every time you subscribed to an event,
such as the left mouse button click, you would supply a callback function to be called, whenever
that event was fired. This can be quite complicated, especially if you don't really need it and
you might find yourself swimming in a hard-to-debug sea of callbacks and asynchronicity.

In the 3️⃣ example below, the event loop will instead be used. Every iteration of the loop, any and all events
will be handled and for each type of event a branch will handle what to do given that event. In the case
of the example for later on is that given a event for a mouse button press, the event handler might find out
which of the two windows was the mouse hovering over and which event handler should receive a signal. If it is
the main graphics window it might receive a command through a channel, if it is the GUI window, it might show an
animation on a button. This is a lot easier to debug and follow with a clear separation of flow and state.

Events are usually integral to interactive systems, if you do not have interactivity, you might want to stick
to async.

## 3️⃣ An Event Loop in Practice
I made some code for showing how you can work with events in practice. You can find the code in
```m2_concurrency::code::egui-winit-wgpu-template``` or
[online](https://github.com/absorensen/the-guide/tree/main/m2_concurrency/code/egui-winit-wgpu-template).

Note that the events stem from Rust's more-or-less defacto window handling library,
[winit](https://github.com/rust-windowing/winit). Try and make sense of what is happening and run the code!
