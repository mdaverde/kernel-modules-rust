# Linux kernel modules written in Rust

A collection of in-progress experimental Linux kernel modules written for the [Rust for Linux](https://github.com/Rust-for-Linux/linux) project

To run the out-of-tree modules here you'll need to run a custom kernel with the changes developed in [mdaverde/linux](https://github.com/mdaverde/linux) which will continuously be rebasing off the upstream R4L fork.

This project uses [just](https://github.com/casey/just) and [zx](https://github.com/google/zx) for project-wide task management but installing a specific module should just require make.

## Current modules

The modules listed here have only been tested on an Ubuntu 21.04 x86_64 VM

- [current](./current/main.rs) - logs (`dmesg`) information about the task context in which the module is running in (e.g. the module insert process)
- [proc_iter](./proc_iter/main.rs) - logs attributes of every `task_struct` (except `swapper/0`) currently running
- [mem_layout](./mem_layout/main.rs) - summarizes memory layout of the running kernel
- [bsa](./bsa/main.rs) - custom wrapper around a few of the kernel page allocation APIs and logs physical continuity
- [kmalloc_box](./kmalloc_box/main.rs) - custom [alloc::Allocator](https://doc.rust-lang.org/beta/alloc/alloc/trait.Allocator.html#) (nightly) wrapped around `kmalloc()` and `kfree()` used with `Box::try_new_in`

`mod_template/` is meant to be a starting template for future modules

## Contributing

This repo is meant to be experimental and a showcase of potential LKM functionality with Rust. This project assumes you have all the same dependencies as R4L installed and can compile/install custom kernels.

### Using Just

```shell
$ just --list
Available recipes:
    build module=DEFAULT_MODULE
    clean module=DEFAULT_MODULE
    create module
    default
    fmt
    rust-analyzer
    vars
$ just fmt # runs rustfmt */*.rs
$ just build # builds all modules
$ just build kmalloc_box # builds specific module
$ just create new_module # start new module
```

### To install a specific module

#### With Make

```shell
$ cd ./current
$ make KERNELDIR=/to/rust/kernel LLVM=1 modules
$ sudo insmod ./current.ko # install module
```
### Future module ideas

- [Procfs](https://www.kernel.org/doc/html/latest/filesystems/proc.html) recreation
- Module stacking example

## References

- [Rust for Linux fork](https://github.com/Rust-for-Linux/linux)
- [Rust port of LDD3](https://github.com/d0u9/Linux-Device-Driver-Rust)
- [The Linux Kernel Module Programming Guide](https://sysprog21.github.io/lkmpg/)
- [Linux Kernel Programming modules](https://github.com/PacktPublishing/Linux-Kernel-Programming)
