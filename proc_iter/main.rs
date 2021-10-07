#![no_std]
#![feature(allocator_api, global_asm)]

use kernel::prelude::*;
use kernel::task::ProcessIterator;

struct ProcessIteratorModule;

module! {
    type: ProcessIteratorModule,
    name: b"process_iterator",
    author: b"milan@mdaverde.com",
    description: b"A kernel module",
    license: b"Dual MIT/GPL",
}

fn show_processes() {
    let proc_iter = ProcessIterator::new();
    for proc in proc_iter {
        pr_info!("pid {:?}", proc.pid());
    }
}

impl KernelModule for ProcessIteratorModule {
    fn init() -> Result<Self> {
        show_processes();
        Ok(ProcessIteratorModule)
    }
}

impl Drop for ProcessIteratorModule {
    fn drop(&mut self) {}
}
