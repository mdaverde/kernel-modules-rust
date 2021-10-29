//! A kernel module
#![no_std]
#![feature(allocator_api, global_asm)]

use kernel::prelude::*;

module! {
    type: ModTemplate,
    name: b"mod_template",
    author: b"milan@mdaverde.com",
    description: b"A kernel module",
    license: b"Dual MIT/GPL",
}

struct ModTemplate;

impl KernelModule for ModTemplate {
    fn init() -> Result<Self> {
        pr_info!("hello from rust\n");
        Ok(ModTemplate)
    }
}

impl Drop for ModTemplate {
    fn drop(&mut self) {
        pr_info!("bye from rust!\n");
    }
}
