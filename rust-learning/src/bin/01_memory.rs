fn say_name(name: String) {
    println!("{}", name);
}
fn main() {
    /*
        1. "hello world" 作为一个字符串常量(string literal),
            在编译时被存入可执行文件的 .RODATA段（GCC)或者.RDATA段(VC++),
            在程序加载时，获得一个固定的内存地址
        2.  当执行 "hello world".to_string() 时， 在堆上， 一块新的内存被分配出来，
            并把 "hello world" 逐个字节拷贝过去
        3.  当我们把堆上的数据赋值给s时， s作为分配在栈上的一个变量， 他需要知道堆上的
            内存地址， 另外由于堆上的数据大小不确定且可以增长，我们还需要知道它的长度以及
            它现在有多大
        4.  最终为了表述这个字符串， 我们使用了3个word：第一个表示指针， 第二个表示字符串的当前长度，
            第三个表示这片内存的总容量（11）。在64位的操作系统下，三个word是24个字节
    */
    let _s = "hello world".to_string();
    println!("{}", _s);

    /*
        栈
          栈是程序运行的基础。每当一个函数被调用时， 一块连续的内存就会在栈顶分配出来，
        这块内存被成为帧（frame)。
          我们知道，栈是自顶向下增长的，一个程序的调用栈最底部，除去入口帧（entry frame），
        就是 main() 函数对应的帧，而随着 main() 函数一层层调用，栈会一层层扩展；
        调用结束，栈又会一层层回溯，把内存释放回去。在调用的过程中，一个新的帧会分配足够的空间存储寄存器的上下文。
          在函数里使用到的通用寄存器会在栈保存一个副本，当这个函数调用结束，通过副本，
        可以恢复出原本的寄存器的上下文，就像什么都没有经历一样。此外，
        函数所需要使用到的局部变量，也都会在帧分配的时候被预留出来。
    */

    /*
            一个函数运行时， 怎么确定究竟需要多大的帧呢？
        这要归功于编译器。在编译并优化代码的时候， 一个函数就是一个最小的编译单元
        在这个函数里，编译器得知道需要用到哪些寄存器、栈上要放哪些局部变量， 而这些
        都要在编译时确定。所以编译器就需要明确每个局部变量的大小，以便于预留空间。

            这下我们就明白了： 在编译时，一切无法确定大小或者大小可以改变的数据，都无法安全地
        放在栈上，最好放在堆上。
        比如一个函数，参数是字符串：
    */

    say_name("Lindsey".to_string());
    say_name("Rosie".to_string());
    /*
        字符串的数据结构， 在编译时大小不确定， 运行时执行到具体的代码才知道大小。
        比如上面的 say_name()函数， 只有在运行时， 才能知道参数的具体长度。
        所以， 我们无法吧字符串本身放在栈上， 只能先将其放在堆上， 然后在栈上分配
        对应的指针，引用堆上的内存。
    */

    /*
        放栈上的问题
            栈上的内存分配是非常高效的。 只需要改动栈指针（stack pointer）， 就可以预留相应的空间；
            把栈指针改动回来， 预留的空间又会被释放掉。预留和释放只是动动寄存器，不涉及额外计算， 不涉及
            系统调用， 因而效率很高。
        那为什么实际工作中， 我们又要避免把大量数据分配在栈上呢？
                这主要是考虑到调用栈的大小，避免栈溢出（stack overflow）。 一般当前程序的调用栈超出了系统允许的
            最大栈空间， 无法创建新的帧， 来运行下一个要执行的函数， 就会发生栈溢出， 这时程序会被系统终止，
            产生奔溃信息。
                过大的栈内存分配是导致栈溢出的原因之一， 更广为人知的原因是地递归函数没有妥善终止。
            一个递归函数会不断调用自己，每次调用都会形成一个新的帧， 如果递归函数无法终止， 最终就会导致栈溢出。
    */

    /*
        堆
          栈虽然使用起来高效，但它的局限也显而易见。当我们需要动态大小的内存时，只能使用堆，比如可变长度的数组，列表，哈希表，字典，
        他们都分配在堆上。
          堆上分配内存时，一般都会预留一些空间， 这是最佳实践。
        比如你创建一个列表，并往里添加两个值：
    */
    let mut arr = Vec::new();
    arr.push(1);
    arr.push(2);
    /*
            这个列表实际预留的大小是4， 并不等于其长度2。 这是因为堆上内存分配会使用libc提供的malloc()函数， 其内部会请求操作系统的系统调用， 来分配内存。
        系统调用的代价是昂贵的， 所以我们要避免频繁地 malloc()。

            在堆内存分配时， 预留的空间4 会大于 实际大小2

            除了动态大小的内存需要被分配到堆上外，动态生命周期的内存也需要分配到堆上
        上面说到，栈上的内存在函数调用结束之后，所使用的帧Frame被回收，相关变量对应的内存
        也都被回收待用。 所以栈上内存的生命周期是不受开发者控制的，并且局限在当前调用栈。

            而堆上分配出来的每一块内存需要显示地释放，这就使堆上内存有更加灵活的生命周期，
        可以在不同的调用栈之间共享数据。
    */

    /*
            如果手工管理内存的话， 堆上内存分配后忘记释放， 就会造成内存泄漏，
        程序运行得越久， 就越吃内存， 最终会因为占满内存而被操作系统终止运行。
            如果堆上内存被多个现成的调用栈引用， 该内存的改动要特别小心，需要加锁
        以独占访问， 来避免潜在的问题。 比如说， 一个现成在编程列表， 而另一个线程在释放列表
        中的某一项， 就可以访问野指针， 导致堆越界（heap out of bounds)。 而堆越界是第一
        大内存安全问题
            如果堆上内存被释放， 但栈上指向内存你的相应指针没有被清空， 就有可能发生使用已释放
        内存（use after free）的情况， 程序轻则崩溃，重则隐含安全隐患。
    */
    //Programming Development Basic Concepts
    /*
        GC, ARC如何解决
            1. java为首的一系列编程语言， 采用了追踪式垃圾回收（Tracing GC）的方法， 来自动管理
        内存。这种方式通过定期标记（mark）找出不再被引用的对象， 然后将其清理（sweep）掉，来自动
        管理内存， 减轻开发者的负担。
            2. OjbC 和 Swift则走了另一条路： 自动引用计数（Automatic Reference Counting）。
        在编译时， 它为每个函数插入 retain/release 语句来自动维护堆上对象的引用计数， 当引用计数
        为0的时候， release语句就释放对象。
        对比： 从效率上来说， GC在内存分配和释放上无需额外操作，而ARC添加了大量的额外代码处理引用计数，
        所以GC的效率更高， 吞吐量（throughput）更大。
              但是， GC释放内存的时机是不确定的， 释放时引发的STW（Stop The World）， 也会导致代码
        执行的延迟（latency）不确定。 所以一般携带GC的编程语言，不适于做嵌入式系统或者实时系统。

    */

    /*
        总结：
            1. 对于存入栈上的值， 它的大小在编译器就需要确定。 栈上存储的变量生命周期在当前调用栈的作用域内，
        无法跨调用栈引用。
            2. 堆可以存入大小未知或者动态伸缩的数据类型。堆上存储的变量， 其生命周期从分配后开始， 一直到释放
        时才结束， 因此堆上的变量允许在多个调用栈之间引用。 但也导致堆变量的管理非常复杂， 手工管理会引发很多内存
        安全性问题，而自动管理， 无论是GC还是ARC， 都有性能损耗和其他问题。
        一句话对比总结就是： 栈上存放的数据是静态的， 静态大小，静态生命周期；堆上存放的数据是动态的， 动态大小，
        动态生命周期。
    */

    /*
        思考题：
            1. 如果有一个数据结构需要在多个线程中访问， 可以把它放在栈上吗？ 为什么？
                在多线程场景下，每个线程的生命周期是不固定的，无法在编译期知道谁先结束谁后结束，
              所以你不能把属于某个线程 A 调用栈上的内存共享给线程 B，因为 A 可能先于 B 结束。
              这时候，只能使用堆内存。这里有个例外，如果结束的顺序是确定的，那么可以共享，比如 scoped thread

            2. 可以使用指针引用栈上的某个变量吗？ 如果可以， 在什么情况下可以这么做？
                而同一个调用栈下，main() 调用 hello()，再调用 world()，编译器很清楚，world() 会先结束，之后是 hello()，
              最后是 main()。所以在 world() 下用指针引用 hello() 或者 main() 内部的变量没有问题，这个指针必然先于它指向的值结束。
              这个两个问题的实质是我们要搞明白哪些东西在编译期可以确定它们的关系或者因果，哪些只能在运行期确定。
    */
}

#[test]
fn go() {
    println!("Hello world");
}
