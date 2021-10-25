# Linux kernel modules written in Rust

A collection of in-progress experimental Linux kernel modules written for the [Rust for Linux](https://github.com/Rust-for-Linux/linux) project

To run the out-of-tree modules here you'll need to run a custom kernel with the changes developed in [mdaverde/linux](https://github.com/mdaverde/linux) which will continuously be rebasing off the upstream R4L fork.

## Current modules

The modules listed here have only been tested on an Ubuntu 21.04 x86_64 VM

- [current.ko](./current) - logs (`dmesg`) information about the task context in which the module is running in (e.g. the module insert process)
- [proc_iter.ko](./proc_iter) - logs attributes of every `task_struct` (except `swapper/0`) currently running
- [mem_layout.ko](./mem_layout) - summarizes memory layout of the running kernel

`mod_template/` is meant to be a starting template for future modules

## To run a specific module

This project uses [just](https://github.com/casey/just) and [zx](https://github.com/google/zx) for project-wide task management but the instructions below should not require extra deps to run a specific module.

### With make

```shell
$ cd ./current
$ make KERNELDIR=/to/mdaverde/kernel/src LLVM=1 modules
$ sudo insmod ./current.ko # Insert module into live kernel
```

## References

- [Rust for Linux fork](https://github.com/Rust-for-Linux/linux)
- [Rust port of LDD3](https://github.com/d0u9/Linux-Device-Driver-Rust)
- [The Linux Kernel Module Programming Guide](https://sysprog21.github.io/lkmpg/)
- [Linux Kernel Programming modules](https://github.com/PacktPublishing/Linux-Kernel-Programming)
