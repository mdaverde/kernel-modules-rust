//! A kernel module to show a simple kernel memory layout visual (on 64bit)
#![no_std]
#![feature(allocator_api, global_asm)]

use kernel::bindings::{
    high_memory, FIXADDR_SIZE, FIXADDR_START, MODULES_END, MODULES_VADDR, PAGE_OFFSET, VMALLOC_END,
    VMALLOC_START,
};
use kernel::prelude::*;
use kernel::task::Task;

#[cfg(CONFIG_KASAN)]
use kernel::bindings::{KASAN_SHADOW_END, KASAN_SHADOW_START};

// Can't use usize here?
const BYTES_KB: u64 = 2_u64.pow(10);
const BYTES_MB: u64 = 2_u64.pow(20);
const BYTES_GB: u64 = 2_u64.pow(30);

// TODO: macros should also encapsulate the memory calculation
// Should use format_args here?
macro_rules! pr_b {
    ($($pr_args: expr), +) => {
        pr_info!("| {:<25} {:0>16x} - {:0>16x} | [{} bytes]\n", $($pr_args), +);
    };
}

macro_rules! pr_k {
    ($($pr_args: expr), +) => {
        pr_info!("| {:<25} {:0>16x} - {:0>16x} | [{} KB]\n", $($pr_args), +);
    };
}

macro_rules! pr_mb {
    ($($pr_args: expr), +) => {
        pr_info!("| {:<25} {:0>16x} - {:0>16x} | [{} MB]\n", $($pr_args), +);
    };
}

macro_rules! pr_mb_gb {
    ($($pr_args: expr), +) => {
        pr_info!("| {:<25} {:0>16x} - {:0>16x} | [{} MB = {} GB]\n", $($pr_args), +);
    };
}

struct MemLayout;

impl MemLayout {
    fn kernel() -> Result<()> {
        pr_info!("Kernel layout (decreasing by address)\n");

        #[cfg(CONFIG_ARM)]
        pr_info!(
            "You're currently running on an ARM machine. Layout printed here may need changes\n"
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

        let highmem = unsafe { high_memory as u64 };
        pr_mb_gb!(
            "lowmem region:",
            PAGE_OFFSET(),
            highmem,
            (highmem - PAGE_OFFSET()) / BYTES_MB,
            (highmem - PAGE_OFFSET()) / BYTES_GB
        );

        Ok(())
    }

    fn userspace() -> Result<()> {
        let current = Task::current();
        pr_info!(
            "Current task user VAS layout: {} (pid: {})\n",
            current.comm().to_str()?,
            current.pid()
        );
        if let Some(mem_desc) = current.mm() {
            let mm = unsafe { *mem_desc.as_ptr() };
            pr_b!(
                "Process environment:",
                mm.env_start,
                mm.env_end,
                mm.env_end - mm.env_start
            );
            pr_b!(
                "Arguments:",
                mm.arg_start,
                mm.arg_end,
                mm.arg_end - mm.arg_start
            );
            pr_info!(
                "| {:<25} {:0>16x} {:>18} |\n",
                "Stack start:",
                mm.start_stack,
                ""
            );
            pr_k!(
                "Heap segment:",
                mm.start_brk,
                mm.brk,
                (mm.brk - mm.start_brk) / BYTES_KB
            );
            pr_b!(
                "Static data segment:",
                mm.start_data,
                mm.end_data,
                mm.end_data - mm.start_data
            );
            pr_b!(
                "Text segment:",
                mm.start_code,
                mm.end_code,
                mm.end_code - mm.start_code
            );
        } else {
            pr_info!("No current->mm_struct found\n");
        }
        Ok(())
    }
}

module! {
    type: MemLayout,
    name: b"mem_layout",
    author: b"milan@mdaverde.com",
    description: b"A kernel module",
    license: b"Dual MIT/GPL",
    params: {
        show_userspace: bool {
            default: false,
            permissions: 0, // not available in sysfs
            description: b"Show few userspace VAS details (default = 0/false)",
        },
    },
}

impl KernelModule for MemLayout {
    fn init() -> Result<Self> {
        MemLayout::kernel()?;
        if *show_userspace.read() {
            MemLayout::userspace()?;
        }
        Ok(MemLayout)
    }
}
