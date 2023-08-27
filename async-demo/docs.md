# docs

async/.await是Rust内置语法，用于让异步函数编写得像同步代码。async将代码块转化成 
实现了Future trait 的状态机。使用同步方法调用阻塞函数会阻塞整个线程，但阻塞Future只会
让出（yield）线程控制权，让其他Future继续执行

总的来说，异步的思想就是，把所有的工作都抽象为异步任务 Future，有两种状态，就绪和未就绪，
通过 Spawner 不断产生任务 Task，经过调度到 Executor 执行这个 Future 或者说 Task
如果任务已就绪，则执行，如果发现任务没有就绪，就需要将任务注册到 Reactor，等 Reactor
发现任务已就绪，则通过 Waker 唤醒任务，重新将任务分发到任务队列中，等待调度执行。

构造 Future，可以通过 async/await 构造，也可通过 futures 库中各种组合子构造

Future：用户的程序要表达异步逻辑所构造的对象。要么是通过基础的 Future经过各种组合子构造，要么通过 async/await构造  
Executor：用户构造出的 Future最终需要提交到 Executor中执行  
Reactor：在 Future 执行过程中无法推进需要等待时，需要将 Executor 提供的 Waker 注册在 Reactor 上
Reactor 负责监听 Future 是否 Ready， 如果 Ready，则通过 Waker 通知 Executor 继续执行对应的 Future

Futures crate 介绍

Futures: 是由异步计算产生的单个最终值(一些语言称之为promise)，目前其一部分已合入标准库  
Streams: 标识异步生成的一系列值  
Sinks: 为异步写入数据提供支持  
Executors: 程序负责运行异步任务

其实可以看到，Future，Stream，Sink这些都是抽象的接口，属于最底层的概念  
Future代表一次性的异步值，Stream类似迭代器可以迭代多个异步值，Sink用于发送这些异步值  
future是任务，而真正执行任务需要executor来承载（比较常用的block_on方法，当然也可以使用await）

虽然上面提到了各种各样的概念，但是仔细捋一下，便会发现整个异步可以分为三层：

- Future/Stream/Sink，Reactor/Executor直接作用于前面的三种类型。此层是为底层，一般用户很少接触，库的开发者接触较多。
- 组合子层，为了提供更为复杂的操作，诞生了一系列的异步组合子，使得异步变得更利于使用，用户会使用这些组合子来完成各种各样的逻辑。
- async/await，准确的说，这层远没有上面两层来的重要，但是依然不可或缺，这层使得异步的开发变得轻而易举

注意的地方  
- 不要在任何异步函数中执行任何阻塞操作，不仅仅是thread::sleep, 还有标准库的Tcp/Udp, 以及sync中的channel, Mutex, RWLock 都不应该继续使用，除非你知道你在干什么！替换为async-std 与 futures中实现的版本。
- 如非必要，不要自己尝试去实现Future，自己实现的没有触发wake操作的话，将永远不会唤醒，取而代之，用已经实现好的Future进行组合。
- 使用async/await代替所有需要异步等待的点，这将会极大的简化你的代码

ansyc 代码块最终由编译器转换成一个生成器,生成器会包装在GenFuture中,GenFuture实现  
Future,在poll中调用生成器的resume方法,如果是状态时Yielded则返回Poll::Pending,  
如果是Complete(x),则返回Poll::Ready(x)

```shell
cargo add async_std
cargo add futures

#增加 dev-dependencies 依赖
cargo add --dev tokio-test
```
