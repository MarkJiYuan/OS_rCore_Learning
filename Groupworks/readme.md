## 2020.9.28

今天我花了一整天的时间就为了解决windows的版本问题。

我的windows版本OS内部版本并没有达到安装WSL2的标准，但是电脑不知道为什么并不能继续向上进行功能升级。根据MS的以前的错误处理方案的指导依然没有效果。于是我最后决定重装系统。

在利用MS的工具重装系统之后终于成功地安装了WSL2，但是很多配置文件都没有了，电脑还需要重新配置和适应。

下面介绍一下我遇到的情况以及解决方案：

主要问题：windows10系统在升级版本的时候出现0x8000ffff错误，并且选择`重置`后会显示初始化电脑失败。根据 Microsoft Support 的相关帮助介绍检查了注册表等模块，检测程序没有报错。

解决方案：这个问题的产生很可能是某一次升级windows的时候断电了，导致系统文件除了问题。目前可以解决它的方法是直接重新装一下windows10操作系统。通过下载微软官方的`MediaCreationTool2004.exe`工具选择为其他电脑安装系统并且将一个U盘作为载体，重新安装系统即可。

## 2020.9.29

今天遇到了让人头疼的问题，跟着说明执行命令，在`./configure --target-list=riscv32-softmmu,riscv64-softmmu`时报错`ERROR: "cc" cannot build an executable (is your linker broken?)`，目前不知道究竟是一个什么问题。

根据报错信息和安装过程中的问题，初步推测是linux版本的问题，换用ubuntu18.04后就没有问题了，之前使用的是ubuntu20.04。

可能会缺少一些依赖，这些依赖可以从 https://wiki.qemu.org/Hosts/Linux 中找到，执行`sudo apt-get install git libglib2.0-dev libfdt-dev libpixman-1-dev zlib1g-dev`指令之后就可以正确地按照环境配置的教程完成qemu环境的搭建了。

然而在安装完qemu和rust工具链后依然不能成功编译运行，原因非常简单，是因为rust在安装后还没有合理地配置环境变量。一个简单的方法是打开对应的bash然后在最后一行追加`export export PATH=$PATH:$HOME/.cargo/bin`其中`$HOME/.cargo/bin`要根据当时安装的.cargo目录的位置进行修改。之后运行`source .bashrc`或者重启bash都可以成功配置环境变量。

配置完成后虽然可以正常运行程序了，但是最终还是有错误导致操统不能运行。操统在编译spin的时候报错

```bash
make[1]: Entering directory '/home/simonkorl/rCore-Tutorial/user'
   Compiling proc-macro2 v1.0.23
   Compiling unicode-xid v0.2.1
   Compiling syn v1.0.42
   Compiling rustversion v1.0.3
   Compiling spin v0.5.2
error[E0463]: can't find crate for `core`
  |
  = note: the `riscv64imac-unknown-none-elf` target may not be installed

error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.
error: could not compile `spin`.

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
Makefile:28: recipe for target 'build' failed
make[1]: *** [build] Error 101
make[1]: Leaving directory '/home/simonkorl/rCore-Tutorial/user'
Makefile:2: recipe for target 'run' failed
make: *** [run] Error 2
```

这个问题也让我花了很多时间进行排查，最后决定先把这个问题放下，去做Lab0。结果没有想到在Lab0中发现了对应的解决方法。

在环境配置部分并没写上两个component的配置：

```shell
# 增加RISC-V三元组
rustup target add riscv64imac-unknown-none-elf
# 增加需要的 cargo-binutils
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

这样就可以让 rCore 成功运行起来了，运行的结果类似如下：

```sh
make[1]: Entering directory '/home/simonkorl/rCore-Tutorial/user'
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
Targets: notebook hello_world
Image resized.
make[1]: Leaving directory '/home/simonkorl/rCore-Tutorial/user'
make[1]: Entering directory '/home/simonkorl/rCore-Tutorial/os'
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s

OpenSBI v0.6
   ____                    _____ ____ _____
  / __ \                  / ____|  _ \_   _|
 | |  | |_ __   ___ _ __ | (___ | |_) || |
 | |  | | '_ \ / _ \ '_ \ \___ \|  _ < | |
 | |__| | |_) |  __/ | | |____) | |_) || |_
  \____/| .__/ \___|_| |_|_____/|____/_____|
        | |
        |_|

Platform Name          : QEMU Virt Machine
Platform HART Features : RV64ACDFIMSU
Platform Max HARTs     : 8
Current Hart           : 0
Firmware Base          : 0x80000000
Firmware Size          : 120 KB
Runtime SBI Version    : 0.2

MIDELEG : 0x0000000000000222
MEDELEG : 0x000000000000b109
PMP0    : 0x0000000080000000-0x000000008001ffff (A)
PMP1    : 0x0000000000000000-0xffffffffffffffff (A,R,W,X)
mod memory initialized
mod interrupt initialized
mod driver initialized
.
..
notebook
hello_world
mod fs initialized
hello from kernel thread 7
thread 7 exit
hello from kernel thread 6
thread 6 exit
hello from kernel thread 5
thread 5 exit
hello from kernel thread 4
thread 4 exit
hello from kernel thread 3
thread 3 exit
hello from kernel thread 2
thread 2 exit
hello from kernel thread 1
thread 1 exit
hello from kernel thread 8
thread 8 exit
src/process/processor.rs:87: 'all threads terminated, shutting down'
```

### 环境配置部分文档需要更新的地方

1. 增加Ubuntu版本的警告，20.04版本的依赖可能不满足一些条件，但是18.04可以非常好地契合这个教程。
2. qemu的下载地址下载速度确实非常的慢，目前还没有提供更好的下载地址。
3. 克隆仓库与输出部分仍然是TODO，但是应该并不难填写
4. 需要增加上面的component设置才能成功测试运行。如果一上来不能成功运行标准代码还是挺让人失望的。

### Lab0

从某个角度上来讲，Lab0才是这个教程的开端，它会引导你编写一个最简单的 rust 内核，并且可以保证其可以运行测试。指导书的环境配置部分只需要把 Linux 环境, qemu 与 rust 根据指导安装完毕即可，其余部分更加细节的配置在 Lab0 中会有展现。

**记住，不要太过于较真‘环境配置’一章中可能无法解决的错误，直接开始写Lab会有逐步的引导来解决他们！**

#### Lab0中的问题

执行`rust-objdump target/riscv64imac-unknown-none-elf/debug/os -d --arch-name=riscv64`指令后得到的结果与指导书中不同

```sh
rust-objdump target/riscv64imac-unknown-none-elf/debug/os -d --arch-name=riscv64

target/riscv64imac-unknown-none-elf/debug/os:   file format ELF64-riscv


Disassembly of section .text:

0000000080200000 text_start:
80200000: 09 a0                         j       2
80200002: 01 a0                         j       0
```

但是这并没有影响到最后的程序运行，最后的实验结果依然成功。通过观察汇编代码发现缺少的部分代码恰好是处理栈帧的部分。

