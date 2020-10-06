# DailySchedule 

*九月*                

| Mon                  | Tues                   | Wed                  | Thur                 | Fri                  | Sat                   | Sun                  |
| -------------------- | ---------------------- | -------------------- | -------------------- | -------------------- | --------------------- | -------------------- |
| 1   | 2   | 3  | 4   | 5   | 6  | 7  |
| 8  | 9   | 10 | 11 | 12  | 13 | 14 |
| 15  | 16 | 17 | 18 | 19  | 20  | 21 |
| 22  | 23<br>([D0](#0)) | 24<br>([D1](#1)) | 25<br>([D2](#2)) | 26<br>([D3](#3)) | 27<br>([D4](#4)) | 28<br>([D5](#5)) |
| 29<br>([D6](#6)) | 30   |                      |                      |                      |                       |                      |

*十月*

| Mon                  | Tues                 | Wed                  | Thur                 | Fri                  | Sat                  | Sun                  |
|----------------------|----------------------|----------------------|----------------------|----------------------|----------------------|----------------------|
|                      |                      | 1   | 2                  | 3                    | 4                   | 5 <br> ([D6](#6))                  |
| 6 <br> ([D7](#7))                | 7 <br> ([D7](#7))                   | 8 <br> ([D8](#8))                    | 9 <br> ([D9](#9))| 10 <br> ([D10](#10))  | 11 <br> ([D11](#11)) | 12 <br> ([D12](#12)) |
| 13 <br> ([D13](#13)) | 14 <br> ([D14](#14)) | 15 <br> ([D15](#15)) | 16  <br> ([D16](#16))| 17 <br> ([D17](#17)) | 18 <br> ([D18](#18)) | 19 <br> ([D19](#19)) |
| 20 <br> ([D20](#20)) | 21 <br> ([D21](#21)) | 22 <br> ([D22](#22))  | 23 <br> ([D23](#23)) | 24 <br> ([D24](#24)) | 25 <br> ([D25](#25)) | 26 <br> ([D26](#26)) |
| 27 <br> ([D27](#27)) | 28 <br> ([D28](#28)) | 29 <br> ([D29](#29))  | 30 <br> ([D30](#30)) |                      |                      |                      |

<span id="0"></span>

## Day 0 (2020-09-23)

### 完成

+ 参考了暑期同学的工作成果，赞叹于同学的努力程度以及一丝不苟
+ 创建了git仓库，开始每天记录学习的过程

### 计划

+ 搭建环境（参考OS_Tutorial以及往期同学的记录）
+ 准备Rust学习资源

<span id="1"></span>

## Day 1 

### 完成

+ 看[Rust 程序设计语言 简体中文版](https://kaisery.github.io/trpl-zh-cn/)前4小节
+ 在Mac上安装了rustup, Rust, Cargo

### 收获

+ rust工具箱：
  + rustc - rust compiler，用于编译.rs
  + rustfmt - 代码格式化（多希望Python也有个这样的，还有js手动加分号强迫症难受死）
  + rustup - rust安装工具
  + cargo - 包管理，组织项目（包管理像pip，组织结构像npm，但是快太多了）
  
+ cargo命令：
  + cargo new - 创建新项目
  + cargo build - 生成可执行。加--release生产环境。
  + cargo run - 生成并运行
  + cargo check - 快
  + cargo update - 升级兼容版本（补丁），大版本要手动改Cargo.toml
  
+ 所有权：Rust的杀手级特性，保障安全而且也不需要GC。概念区别于复制（深拷贝），浅拷贝。我的一种理解是限定某一块内存只能被一个指针所指向。

+ 引用、解引用：感觉类似指针，但是不可以修改指针所指向的量（因为没有所有权）。也就说是一个只读指针。

+ 可变引用、不可变引用：从根本上破坏死锁的条件。作用域内，同一时间要不然有一个可变引用，要不然有多个不可变引用。

  

### 计划

* 明天看完书的前15章



<span id="2"></span>

## Day 2

### 完成

+ 看[Rust 程序设计语言 简体中文版](https://kaisery.github.io/trpl-zh-cn/)4-10小节

### 收获

+ 结构体：整个实例可变。字段初始化简写语法。元组结构体，只有类型，没有字段名称。
+ {:?}：Debug输出。{:#?}可以格式化。（不用像python里一样去实现__repr__和__str__了）
+ 方法：感觉有点像go的方式，但是好在可以放在一个impl块当中。也可以放在好几个块中。
+ 枚举：多使用枚举，少使用if。用枚举列举出所有的可能性，用match去处理。match要穷尽所有可能。
+ 无空值：没有None，null这些玩意。要用的话要使用Option类型。
+ 通配符_：配合match使用。为剩下的可能性的缺省处理。
+ use：除了test，不要轻易将公有项全都引入作用域。慎用*。
+ 模块：其实这部分没太看懂。模块树，公私有。需要连连手才能明白了。
+ Vector: 类似python里的list。不过还是有很多不一样。写Rust时刻要注意所有权，引用的问题。
+ HashMap: 赋值的时候，所有权也会属于HashMap(移动)
+ 太多的match也不好！大量的嵌套会导致可读性变差

### 计划

* 继续看书（这本书的很多特性还是挺深的，光是看教程不是很有感觉，还是大概了解结构，真实上手项目不会的时候再回来查才行）

* 开始敲rustling小练习

  

<span id="3"></span>

## Day 3

### 完成

+ 看[Rust 程序设计语言 简体中文版](https://kaisery.github.io/trpl-zh-cn/)10-12小节
+ 开始rustling小练习

### 收获

+ 泛型：很强大，用的好静态语言能用出来动态语言的风范。不过开销是编译时开销而非运行时。（不过我Java的泛型就没搞太懂）
+ trait：目前的理解就是很像Java的Interface。这一节内容好多，看得有点晕了。
+ 测试：Rust写测试的方法

### 计划

* 把小练习敲完
* 准备rCore Lab0

<span id="4"></span>

## Day 4

请一天病假。前一天通宵干活来着，累倒了。

### 完成

+ 无

### 收获

+ 无

<span id="5"></span>

## Day 5

### 完成

+ 参考[rCore-Turorial](https://rcore-os.github.io/rCore-Tutorial-deploy/docs/pre-lab/env.html)完成了实验环境的搭建
+ 在自己的电脑上跑通了目前的rCore系统

### 收获

+ 应该除了像我这么菜的都知道，git clone --recursive的作用。。
+ 成功运行之后，很兴奋地用vi打开了一个文件，结果卡死了
+ 最尴尬的就是面对一屏幕蓝字，按Ctrl + C还没反应的时候，以为是没有启动成功。。。

### 计划

+ 学习make工具，大概能看懂下make脚本
+ 把rust小练习完成
+ 开始浏览组成架构

<span id="6"></span>

## Day 6

### 记录

+ [no_std]后报错和教程里稍有不一样
+ rust-objdump target/riscv64imac-unknown-none-elf/debug/os -x --arch-name=riscv64 输出内容顺序不太一样
+ Makefile 那一步会报错：Makefile:16: *** missing separator.  Stop. 是因为makefile里的指令需要用tab而不是空格。四个空格不会被认为成是一个tab。文档里直接复制出来是空格。

### 完成

+ 创建内核基础项目，不依赖操作系统的项目构建
+ 安装binutils工具集
+ 使用链接脚本来指定内存布局
+ 最终完成lab0

### 收获

+ 在Rust中Panic需要handler来处理
+ 从不返回的函数是发散函数，发散函数的返回类型称作Never，记为！
+ 堆栈展开
+ 运行时系统。main函数并不是实际执行的第一个函数。C语言的crt0
+ rustc --version --verbose 查看宿主系统。我的是x86_64-apple-darwin
+ rust-objdump -x查看元程序，-d进行反汇编
+ RISC-V 共有 3 种特权级，分别是 U Mode（User / Application 模式）、S Mode（Supervisor 模式）和 M Mode（Machine 模式）。
+ QEMU 可以使用 ctrl+a （macOS 为 control+a） 再按下 x 键退出。
+ 内联汇编（Inline Assembly）


### 计划

+ 完成lab1

<span id="7"></span>

## Day 7

### 记录

+ [handler.rs](https://rcore-os.github.io/rCore-Tutorial-deploy/docs/lab-1/guide/part-5.html)没有导入scause，会导致编译错误

  ~~~bash
  zhengjiyuandeMacBook-Pro:os zhengjiyuan$ make run
     Compiling os v0.1.0 (/Users/zhengjiyuan/Desktop/fifth_grade/operating_system/OS_rCore_Learning/rCore_from_scrach/lab1/os)
  error[E0412]: cannot find type `Scause` in this scope
    --> src/interrupt/handler.rs:28:56
     |
  28 | pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize) {
     |                                                        ^^^^^^ not found in this scope
     |
  help: consider importing one of these items
     |
  1  | use crate::interrupt::handler::scause::Scause;
     |
  1  | use riscv::register::scause::Scause;
  ~~~

  解决方法：(handler.rs)

  ~~~rust
  use riscv::register::{
      scause::{Exception, Interrupt, Scause, Trap},
      sie, stvec,
  };
  ~~~

+ 不要忘了改sbi.rs，在sbi中实现set_timer

  ~~~rust
  pub fn set_timer(time: usize) {
      sbi_call(SBI_SET_TIMER, time, 0, 0);
  }
  ~~~

+ lab1指导和lab-1的repo中的entry.asm稍微不同，一个是call rust_main，一个是jal rust_main

  ~~~asm
  ## entry.asm
  
  # 目前 _start 的功能：将预留的栈空间写入 $sp，然后跳转至 rust_main
  _start:
      la sp, boot_stack_top
      jal rust_main
  ~~~

### 完成

+ lab1的复现

### 收获

+ 操作系统中断的实现方式
+ RISC-V中断以及Context保存恢复
+ 复制粘贴是不行的，确实需要理解工作的过程


### 计划

+ 辨析下rust里包的各种层级及引用方式（mod，crate，super，self...）
+ 学习RISC-V
+ 完成lab2




