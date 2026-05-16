## Experiment 1.2: Understanding how it works

I added a new print statement after `spawner.spawn(...)`:

```rust
println!("Amberley's Komputer: hey hey");
```
the output was:

![image1](images/image1.png) 

The reason hey hey appears first is because spawner.spawn(...) only puts the async task into the executor queue. It does not immediately execute the async block.

The async block starts running only when executor.run() is called. After that, the executor polls the task. The task prints howdy!, then waits for TimerFuture for 2 seconds. When the timer finishes, it wakes the task, and the executor polls it again, so it prints done!

## Experiment 1.3: Multiple Spawn

I added three async tasks using `spawner.spawn(...)`.

Each task prints `howdy`, waits for 2 seconds using `TimerFuture`, then prints `done`.

The output shows that all `howdy` messages appear first, then after around 2 seconds, all `done` messages appear.

This happens because each spawned async block becomes a separate task. The executor polls each task once. Each task prints its `howdy` message, then reaches `.await` and becomes pending. The timers run in separate threads. When the timers finish, they wake the tasks and put them back into the executor queue. Then the executor polls them again and they print the `done` messages.

This shows that the tasks are running concurrently, not one by one sequentially.

### Removing `drop(spawner)`

When I removed `drop(spawner)`, the program printed all the messages, but it did not terminate.

This happens because the executor waits on `ready_queue.recv()`. The `recv()` function keeps waiting as long as there is still a sender alive. The `spawner` owns the sender side of the channel.

When `drop(spawner)` is used, the sender is closed. This tells the executor that there will be no more incoming tasks. After all tasks are complete, the executor can stop.

When `drop(spawner)` is removed, the sender is still alive, so the executor keeps waiting for possible future tasks forever.

![image2](images/image2.png) 