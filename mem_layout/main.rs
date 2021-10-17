//! A kernel module that might only be useful on 64bit
#![no_std]
#![feature(allocator_api, global_asm)]

use kernel::bindings::{
    high_memory, FIXADDR_SIZE, FIXADDR_START, MODULES_END, MODULES_VADDR, PAGE_OFFSET, VMALLOC_END,
    VMALLOC_START,
};
use kernel::prelude::*;

#[cfg(CONFIG_KASAN)]
use kernel::bindings::{KASAN_SHADOW_END, KASAN_SHADOW_START};

// Why can't I use isize here?
const BYTES_MB: i64 = 2_i64.pow(20);
const BYTES_GB: i64 = 2_i64.pow(30);

// TODO: macros should also encapsulate the memory calculation
macro_rules! pr_mb {
    ($($pr_args: expr), +) => {
        pr_info!("| {:<20} {:<15x} - {:15x} | [{} MB]\n", $($pr_args), +);
    };
}

macro_rules! pr_mb_gb {
    ($($pr_args: expr), +) => {
        pr_info!("| {:<20} {:<15x} - {:15x} | [{} MB = {} GB]\n", $($pr_args), +);
    };
}

struct MemLayout;

impl MemLayout {
    fn kernel() {
        pr_info!(
            "
            Kernel layout (decreasing by address)
            --------------------------------------------------"
        );

        #[cfg(CONFIG_ARM)]
        pr_info!(
            "You're currently running on an ARM machine. Layout printed here may need changes"
        );

        pr_mb!(
            "fixmap region:",
            FIXADDR_START(),
            FIXADDR_START() + FIXADDR_SIZE(),
            FIXADDR_SIZE() / BYTES_MB
        );

        pr_mb!(
            "module_region:",
            MODULES_VADDR(),
            MODULES_END(),
            (MODULES_END() - MODULES_VADDR()) / BYTES_MB
        );

        #[cfg(CONFIG_KASAM)]
        pr_mb!(
            "KASAN shadow:",
            KASAN_SHADOW_START(),
            KASAN_SHADOW_END(),
            (KASAN_SHADOW_END() - KASAN_SHADOW_START()) / BYTES_MB
        );

        pr_mb_gb!(
            "vmalloc region:",
            VMALLOC_START(),
            VMALLOC_END(),
            (VMALLOC_END() - VMALLOC_START()) / BYTES_MB,
            (VMALLOC_END() - VMALLOC_START()) / BYTES_GB
        );

        let highmem = unsafe { high_memory as i64 };
        pr_mb_gb!(
            "lowmem region:",
            PAGE_OFFSET(),
            highmem,
            (highmem - PAGE_OFFSET()) / BYTES_MB,
            (highmem - PAGE_OFFSET()) / BYTES_GB
        );
    }
}

module! {
    type: MemLayout,
    name: b"mem_layout",
    author: b"milan@mdaverde.com",
    description: b"A kernel module",
    license: b"Dual MIT/GPL",
}

impl KernelModule for MemLayout {
    fn init() -> Result<Self> {
        MemLayout::kernel();
        Ok(MemLayout)
    }
}

impl Drop for MemLayout {
    fn drop(&mut self) {}
}
