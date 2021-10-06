# Linux kernel modules written in Rust

A collection of Linux kernel modules written in Rust based on the [Rust for Linux](https://github.com/Rust-for-Linux/linux) project.

The intention here is to create a contribution workspace for experimenting with Rust in the kernel

Everything here is most likely broken but if you want to attempt to run the modules written here, you'll need to run a kernel with the changes developed in [mdaverde/linux](https://github.com/mdaverde/linux).

## To run a specific module

```shell
$ cd ./current_affairs
$ make KERNELDIR=to/mdaverde/kernel/src LLVM=1 modules
$ sudo insmod ./current_affairs.ko # Insert module into live kernel
```

## Links

- [Rust for Linux fork](https://github.com/Rust-for-Linux/linux)
- [Rust port of LDD3](https://github.com/d0u9/Linux-Device-Driver-Rust)
- [Linux Kernel Programming modules in C](https://github.com/PacktPublishing/Linux-Kernel-Programming)
