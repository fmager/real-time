# 2️⃣ Events
Events
Event loops
Can become much more complicated
[Event driven architecture](https://en.wikipedia.org/wiki/Event-driven_architecture).
One way of doing this is building on channels to deliver events to event consumers/subscribers.
This requires the inverse of the ```mpsc::channel``` we saw earlier. A single producer, multiple consumer channel.
A lot of GUI systems are event driven and most interactions from the users side will result in an event.

## 3️⃣ An Event Loop in Practice
winit!
