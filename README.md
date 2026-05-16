## Experiment 1.2: Understanding how it works

I added a new print statement after `spawner.spawn(...)`:

```rust
println!("Amberley's Komputer: hey hey");
```
the output was:

![image1](images/image1.png) 

The reason hey hey appears first is because spawner.spawn(...) only puts the async task into the executor queue. It does not immediately execute the async block.

The async block starts running only when executor.run() is called. After that, the executor polls the task. The task prints howdy!, then waits for TimerFuture for 2 seconds. When the timer finishes, it wakes the task, and the executor polls it again, so it prints done!