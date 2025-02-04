# oscmp midterm

# 目标描述

本项目使用 aster-frame 复刻 seL4

- 一个小型的微内核
- 兼容 seL4 的用户程序
- 满足 aster-frame 的要求(将黑暗的 "unsafe rust" 只留在其中)

# 比赛题目分析和相关资料调研

## 项目难点

客观

1. seL4 仓库数量多，构建系统复杂
2. safe rust 需要对 seL4 的核心数据结构进行重新包装，并且在 memory layout 上与原来的 C 结构保持一致（兼容 seL4 user libs）

主观

1. 对于 x86 结构不够熟悉，之前的背景都是基于 RISCV
2. 对于 seL4 没有基础得从头开始学习
3. aster-frame 中用了大量 rust unstable feature ，这些 feature 的文档比较少，只能通过 github tracking issue 逐个分析

## aster-frame

aster-frame 包含了内核启动，硬件初始化等相关代码 ，简化了内核的开发，并且将unsafe rust 代码进行封装提供了高级安全的抽象。

## seL4简介

seL4 设计目的是为各种应用程序领域的系统提供安全、可靠的基础。作为微内核，它为应用程序提供了少量服务，如创建和管理虚拟地址空间、线程和进程间通信(IPC)的抽象。在 ARM 架构下的seL4大约只有 8700 行。这使得ARMv6版本的内核可以在 Isabelle/HOL 定理证明器中提供形式化证明。内核较小的体积也有助于对最坏情况下的执行时间进行完整而可靠的分析。

seL4 将系统中的功能抽象为内核对象，而对这些对象进行操作则需要 Capability， 比如分页， 如果用户程序拥有自己 VSpace Cap ，则用户程序可以在其中映射分页中间结构，在 X86 中就是 pdpt 等。

### 内核对象 （ Kernel Objects ）

- CNode
    
    > 存储 capability，允许线程调用特定对象的方法。每个CNode都有固定数量的插槽，大小是2的 K 次幂，这是在CNode创建时确定的。插槽可以是空的，也可以包含一个能力。
    > 
- Thread Control Blocks
    
    > 线程是执行单元，根据应用程序与其他线程的交互情况，线程可以被调度、阻塞、未阻塞等。
    > 
- Endpoints
    
    > 用来线程之间的消息传递通信。IPC是同步的：一个线程试图在一个 endpoint 上发送或接收消息会发生阻塞。这意味着，只有在发送方和接收方同时使用 endpoint 时，消息传递才会进行，而内核可以只使用一个副本来传递消息(对于短消息，则不使用复制，只使用寄存器)。endpoint 的 capabiliy 可以被限制为仅发送或仅接收。此外，端点功能可以具有grant权限，这允许将发送 capability 作为消息的一部分。
    > 
- Notification Objects
    
    > 提供一个简单的信号机制。notification 是一个字大小的标志数组，每个标志的行为类似于二进制信号量。
    > 
- Virtual Address Space Objects
    
    > 用来构造一个虚拟的一个或多个线程的地址空间(或VSpace)。这些对象在很大程度上直接对应于硬件，因而是体系结构相关的。内核还包括ASID池和ASID控制对象，用于跟踪地址空间的状态。
    > 
- Interrupt Objects
    
    > 给予应用程序接收中断。最初，IRQControl只有一个 capability ，它允许创建IRQHandler 。IRQHandler capability 允许管理与特定设备关联的特定中断源。它委托给一个设备驱动程序来访问中断源。IRQHandler对象允许线程等待和确认各个中断。
    > 
- Untyped Memory
    
    > seL4内核内存分配的基础。内核只提供一种方法创建新的内核对象。如果方法成功，调用线程将获得对新创建对象的访问权限。此外，Untyped Memory 对象可以划分为一组较小的 Untyped Memory 对象，允许委托系统的部分(或全部)内存。
    > 

### Capability

capability 是唯一的、不可伪造的令牌，它赋予所有者访问系统中的实体或对象的权限。可以将其视为一种具有访问权限的指针。

在 seL4 中有三类 capability :

- 可以访问内核对象的 capabilities,
- 控制内核抽象资源的 capabilities 如 IRQControl
- 负责管理物理内存的 untyped capabilities

在 seL4 内核初始化完成之后会将整个系统中的所有能力传给 root task。

### Addressing

用户可以通过 Invocation 和 Direct CSpace addressing 的方式来寻找 capability 。

- invocation 就是通过隐式使用 thread 的 cspace root 进行寻找
- direct 即直接指定 CNode

未完待续

# 系统框架设计

**注意：暂时不实现 MCS 机制， 并且 aster-frame 目前不支持 Thread Local Storage**

整体上项目有点像用 Rust 重写 seL4 ，开源社区中也有这样的经验，比如 [DragonOS](https://dragonos.org/) 使用 Rust 重写了 C 代码。一般来说可以使用 rust-ffi 先将一部分代码与原来的 C 代码进行链接来进行替换。但是考虑到要使用 aster-frame 我们没有采用这样的方式。

## v1

在项目开始的时候，我们还不熟悉 seL4 的原理，故开始简单粗暴的将 seL4 中的 C 代码直接翻译成 Rust 代码，这样的好处是不会浪费时间并且能够对 seL4 能有一个全局性的认识，缺点是之后必须进行重构。参考了 aster-nix （使用 aster-frame 实现的兼容 linux 的宏内核）学习 aster-frame api 的具体用法，并且采用了 aster-nix 中 process，thread，vmar，vmo 的抽象

## v2

重构计划，准备使用sel4-sys 这个库来提供 sel4 中最基本的一些数据结构，然后在其上构建出 seL4中一个个内核对象的 Rusty 版本。sel4-sys 是 seL4 基金会支持的一个 seL4 rust binding，主要是让 rust 能够写 seL4 的用户程序而不是内核。重写一些 aster-frame 中的抽象，比如 PageTable，aster-frame 中的 page table 是对虚拟内存很好的抽象但是在 seL4 中由于 capability 的存在，它的灵活性就稍显不足了。

未完待续

# 遇到的主要问题和解决方法

1. 在由于使用了aster-frame，所以所有的空闲内存都是被 aster-frame 中的内存分配器管理的内核是不能直接使用的，在给 root task 传所有的物理内存时出现了问题，seL4 中会将所有的空闲内存以 Untyped Memory 的方式给到 root task， v1 中只能将空闲内存以 vmo 的方式映射给 root task 的页表。与 seL4 的行为不一致
2. 在 seL4 中将 root task elf 放到一个特定的内存地址之后会开始为其构造页表和相应的能力，而在 v1 中是构造页表再从其中生成出 pml4，pdpt，pd，pt 等能力

# 系统测试情况

最终目标能够通过 seL4test

目前只能给运行 seL4 tutorial中简单的程序

# 开发计划

- [x]  2024.4.11 - 2024.4.18
分析 aster-frame 相关代码和文档，配置开发环境
- [x]  2024.04.18 - 2024.04.22
阅读 seL4 官网的文档和手册，搭建 seL4 的开发环境，并且构建运行
- [x]  2024.04.22 - 2024.4.29
将 seL4 主体代码翻译为 Rust
- [x]  2022.05.13 - 2022.5.23
使用 aster-frame 启动系统，调试 kernel 和 root task
- [x]  2022.05.24-2022.5.25
完善相关技术文档。
- [ ]  2022.05.26 -
重构 Rust 代码， 打磨系统细节

# 中期开发状态

[**点击此处查看演示视频**](https://pan.baidu.com/s/1WcrzGCPzr5pznlpo4_jIRQ?pwd=wp6t)

**测试分析：**

```c
测试代码
/*
 * Copyright 2018, Data61, CSIRO (ABN 41 687 119 230).
 *
 * SPDX-License-Identifier: BSD-2-Clause
 */

/*
 * seL4 tutorial part 2: create and run a new thread
 */

/* Include config variables. */
#include <autoconf.h>

#include <assert.h>
#include <stdio.h>

#include <sel4/sel4.h>

#include <simple-default/simple-default.h>
#include <simple/simple.h>

#include <vka/object.h>

#include <allocman/allocman.h>
#include <allocman/bootstrap.h>
#include <allocman/vka.h>

#include <sel4utils/sel4_zf_logif.h>
#include <sel4utils/thread.h>
#include <utils/arith.h>
#include <utils/zf_log.h>

#include <sel4platsupport/bootinfo.h>

/* global environment variables */

/* seL4_BootInfo defined in bootinfo.h */
seL4_BootInfo *info;

/* simple_t defined in simple.h */
simple_t simple;

/* vka_t defined in vka.h */
vka_t vka;

/* allocman_t defined in allocman.h */
allocman_t *allocman;

/* static memory for the allocator to bootstrap with */
#define ALLOCATOR_STATIC_POOL_SIZE (BIT(seL4_PageBits) * 10)
UNUSED static char allocator_mem_pool[ALLOCATOR_STATIC_POOL_SIZE];

int main(void) {
  UNUSED int error = 0;

  /* TASK 1: get boot info */
  /* hint: platsupport_get_bootinfo()
   * seL4_BootInfo* platsupport_get_bootinfo(void);
   * @return Pointer to the bootinfo, NULL on failure
   */

  info = platsupport_get_bootinfo();
  ZF_LOGF_IF(info == NULL, "Failed to get bootinfo.");

  /* TASK 2: initialise simple object */
  /* hint: simple_default_init_bootinfo()
   * void simple_default_init_bootinfo(simple_t *simple, seL4_BootInfo *bi);
   * @param simple Structure for the simple interface object. This gets
   * initialised.
   * @param bi Pointer to the bootinfo describing what resources are available
   */

  simple_default_init_bootinfo(&simple, info);
 
  /* TASK 3: print out bootinfo and other info about simple */
  /* hint: simple_print()
   * void simple_print(simple_t *simple);
   * @param simple Pointer to simple interface.
   */

  simple_print(&simple);
  return 0;
}
```

上述代码主要是打印内核传递出来的 BootInfo

seL4Runtime 会先做一些 C 语言运行时的初始化工作。

![Untitled](docs/oscmp-midterm/oscmp%20midterm%208a6e958a99b545d0a152681ca02447e7/Untitled.png)

与 seL4 结果对比：

![Untitled](docs/oscmp-midterm/oscmp%20midterm%208a6e958a99b545d0a152681ca02447e7/Untitled%201.png)

# 分工和协作

陈嘉鑫：方案制定，内核代码编写，文档编写

陈益兵：seL4 CMake 分析

顾鹏飞：相关测试代码编写

# 提交仓库目录和文件描述

```
├── antimono                                                               // anti monolithic kernel is micro kernel 🤔
│   ├── asm                                                                // assemberly
│   │   ├── kernel.asm
│   │   └── user.asm
│   ├── Cargo.toml
│   ├── crates                                                             // dep crates
│   │   ├── align-ext                                                      // aster-frame deps
│   │   ├── anti-frame                                                     // aster-frame fork
│   │   ├── anti-main                                                      // aster-frame deps
│   │   ├── anti-rights                                                    // aster-frame deps
│   │   ├── anti-rights-proc                                               // aster-frame deps
│   │   ├── anti-util                                                      // aster-frame deps
│   │   ├── id-alloc                                                       // aster-frame deps
│   │   ├── int-to-c-enum                                                  // aster-frame deps
│   │   ├── sel4                                                           // sel4 rust user-level libs
│   │   │   ├── bitfield-ops
│   │   │   ├── bitfield-parser
│   │   │   ├── build-env
│   │   │   ├── Cargo.nix
│   │   │   ├── Cargo.toml
│   │   │   ├── config
│   │   │   ├── src
│   │   │   └── sys                                                        // sel4 c bindings
│   │   ├── sel4-rustfmt-helper
│   │   ├── typeflags
│   │   └── typeflags-util
│   ├── ember                                                              // the sel4 fork kernel
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── boot                                                       // userless moved in sel4 mod
│   │       ├── c_api.rs                                                   // userless moved in sel4 mod
│   │       ├── common                                                     // userless moved in sel4 mod
│   │       ├── common.rs                                                  // userless moved in sel4 mod
│   │       ├── config.rs                                                  // userless moved in sel4 mod
│   │       ├── cspace                                                     // userless moved in sel4 mod
│   │       ├── cspace.rs                                                  // userless moved in sel4 mod
│   │       ├── debug.rs                                                   // userless moved in sel4 mod
│   │       ├── deps.rs                                                    // userless moved in sel4 mod
│   │       ├── error.rs                                                   // system errors
│   │       ├── exception.rs                                               // userless moved in sel4 mod
│   │       ├── interrupt                                                  // userless moved in sel4 mod
│   │       ├── interrupt.rs                                               // userless moved in sel4 mod
│   │       ├── kernel                                                     // userless moved in sel4 mod
│   │       ├── kernel.rs                                                  // userless moved in sel4 mod
│   │       ├── lib.rs
│   │       ├── object.rs                                                  // userless moved in sel4 mod
│   │       ├── process                                                    // process related
│   │       │    ├── mod.rs
│   │       │    ├── process_builder.rs
│   │       │    ├── process_vm
│   │       │    ├── program_loader                                        // elf loader and parser
│   │       │    └── sel4_thread
│   │       ├── sched.rs                                                   // userless moved in sel4 mod
│   │       ├── sel4                                                       // sel4 raw fork
│   │       │   ├── boot
│   │       │   ├── common
│   │       │   ├── cspace
│   │       │   ├── exception
│   │       │   ├── mod.rs
│   │       │   ├── utils
│   │       │   └── vspace
│   │       ├── syscall                                                    // syscalls
│   │       │   ├── invocation
│   │       │   ├── mod.rs
│   │       │   ├── sel4_syscalls
│   │       │   │   └── mod.rs
│   │       │   ├── syscall_reply.rs
│   │       │   └── utils.rs
│   │       ├── task_manager                                               // useless replaced by thread
│   │       │   ├── ipc
│   │       │   │   ├── endpoint.rs
│   │       │   │   ├── notification.rs
│   │       │   │   └── transfer.rs
│   │       │   ├── ipc.rs
│   │       │   ├── registers.rs
│   │       │   ├── scheduler.rs
│   │       │   ├── structures.rs
│   │       │   ├── tcb.rs
│   │       │   ├── tcb_queue.rs
│   │       │   └── thread_state.rs
│   │       ├── thread                                                     // thread related
│   │       │   ├── exception.rs
│   │       │   ├── kernel_thread.rs
│   │       │   ├── mod.rs
│   │       │   ├── status.rs
│   │       │   ├── task.rs
│   │       │   ├── thread_table.rs
│   │       │   └── user.rs
│   │       ├── utils.rs
│   │       ├── vm                                                         // virtual memory will be refactored
│   │       │   ├── mod.rs
│   │       │   ├── page_fault_handler.rs
│   │       │   ├── perms.rs
│   │       │   ├── vmar
│   │       │   └── vmo
│   │       ├── vspace                                                     // useless moved in sel4 mod
│   │       │   ├── asid.rs
│   │       │   ├── interface.rs
│   │       │   ├── structures.rs
│   │       │   ├── utils.rs
│   │       │   └── vm_rights.rs
│   │       └── vspace.rs
│   ├── images                                                             // root task programs
│   │   ├── hello
│   │   ├── hello-bin
│   │   ├── hello.S
│   │   └── root_task
│   ├── justfile
│   ├── OSDK.toml
│   ├── README.md
│   ├── rust-toolchain.toml
│   ├── src
│   │   └── lib.rs
├── asterinas // aster git module
├── OVMF // ed2k uefi related
└── README.md

```

# 导师沟通及指导情况

在2024年4月，团队与项目指导田教师（田洪亮）开始交流比赛信息，选择[proj306-基于框内核架构的 Rust OS实践和创新](https://github.com/oscomp/proj306-Rust-OS-for-Framekernel-Architecture)。

田老师和蒋老师一直在微信群里为我们答疑和指导，十分感谢两位老师。

最后感谢我的导师刘雅辉老师对我们的支持。

# 参考资料

[seL4 official site（manual)](https://docs.sel4.systems/)

[asterinas repo](https://github.com/asterinas/asterinas)

[intel 64 developer manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)

[rust-sel4 repo](https://github.com/seL4/rust-sel4)

# 比赛收获

- 深入理解了微内核的设计思想
- 学习了 x86 架构
- 提高了系统设计能力
- 对于 Rust 有了新的理解