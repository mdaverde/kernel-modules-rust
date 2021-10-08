# Linux kernel modules written in Rust

A collection of experimental Linux kernel modules written for the [Rust for Linux](https://github.com/Rust-for-Linux/linux) project

Everything here is most likely broken but if you want to attempt to run the modules written here, you'll need to run a kernel with the changes developed in [mdaverde/linux](https://github.com/mdaverde/linux).

## To run a specific module

```shell
$ cd ./current_affairs
$ make KERNELDIR=to/mdaverde/kernel/src LLVM=1 modules
$ sudo insmod ./current_affairs.ko # Insert module into live kernel
```

`mod_template/` is meant to be a starting template for future modules

## References

- [Rust for Linux fork](https://github.com/Rust-for-Linux/linux)
- [Rust port of LDD3](https://github.com/d0u9/Linux-Device-Driver-Rust)
- [Linux Kernel Programming modules](https://github.com/PacktPublishing/Linux-Kernel-Programming)
