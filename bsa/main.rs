//! A kernel module showcasing BSA APIs alongside Rust's alloc apis
#![no_std]
#![feature(allocator_api, global_asm)]

use alloc::vec::Vec;
use kernel::{bindings, c_types};
use kernel::{prelude::*, PAGE_SIZE};

module! {
    type: BSA,
    name: b"bsa",
    author: b"milan@mdaverde.com",
    description: b"A kernel module showcasing BSA APIs",
    license: b"Dual MIT/GPL",
    params: {
        alloc_order: u32 {
            default: 3,
            permissions: 0,
            description: b"Order of the allocation (default = 3)",
        },
    },
}

// SAFETY: start_vaddr must live within kernel's lowmem region
unsafe fn log_physical_pages(
    start_vaddr: *mut c_types::c_void,
    page_count: usize,
    contiguity_check: bool,
) {
    // equivalent to BITS_PER_LONG?
    let header = if cfg!(target_pointer_width = "64") {
        "-pg#-  -------va---------   --------pa--------   -PFN-"
    } else {
        "-pg#-  ----va----   --------pa--------   -PFN-"
    };
    pr_info!("{}\n", header);

    let mut previous_page_frame_index = 0;

    for i in 0..page_count {
        let next_addr = unsafe { start_vaddr.add(i * PAGE_SIZE) };
        let physical_addr = unsafe { bindings::virt_to_phys(next_addr) };
        let page_frame_index = unsafe { bindings::PHYS_PFN(physical_addr) };
        pr_info!(
            "{:05}  {:p}   {:#018x}   {:05}\n",
            i,
            next_addr,
            physical_addr,
            page_frame_index
        );
        if i != 0 && contiguity_check && page_frame_index != previous_page_frame_index + 1 {
            pr_info!("----non-contiguity detected in physical addresses----\n");
        }
        previous_page_frame_index = page_frame_index;
    }
}

struct FreePages(*mut c_types::c_void, u32);

impl Drop for FreePages {
    fn drop(&mut self) {
        pr_info!(
            "Freeing BSA memory chunk @ {:p} of order: {}\n",
            self.0,
            self.1
        );
        unsafe { bindings::free_pages(self.0 as c_types::c_ulong, self.1) }
    }
}

// To be sent across init & drop threads of module
unsafe impl Sync for FreePages {}
unsafe impl Send for FreePages {}

struct PageAllocator;

impl PageAllocator {
    fn get_free_page() -> Result<FreePages> {
        Self::get_free_pages(0)
    }

    fn get_free_pages(order: u32) -> Result<FreePages> {
        Self::get_free_pages_masked(bindings::GFP_KERNEL, order)
    }

    fn get_free_pages_masked(mask: bindings::gfp_t, order: u32) -> Result<FreePages> {
        let allocation_ptr =
            unsafe { bindings::__get_free_pages(mask, order) as *mut c_types::c_void };
        if allocation_ptr.is_null() {
            Err(Error::ENOMEM)
        } else {
            Ok(FreePages(allocation_ptr, order))
        }
    }
}

struct BSA {
    // TODO: allocation not needed
    allocs: Vec<FreePages>,
}

impl BSA {
    fn try_new() -> Result<BSA> {
        Ok(BSA {
            allocs: Vec::try_with_capacity(2)?,
        })
    }

    fn run(&mut self, user_order: u32) -> Result<()> {
        let start_vaddr = unsafe { bindings::PAGE_OFFSET() as *mut c_types::c_void };
        unsafe { log_physical_pages(start_vaddr, 5, true) };

        let page_allocation = PageAllocator::get_free_page()?;
        pr_info!("alloc'd 1 page from the BSA @ {:p}\n", page_allocation.0);
        self.allocs.try_push(page_allocation)?;

        let amount_pages_to_alloc = 2_u32.pow(user_order);
        let page_allocation = PageAllocator::get_free_pages(user_order)?;
        pr_info!(
            "alloc'd {} pages from the BSA @ {:p}\n",
            amount_pages_to_alloc,
            page_allocation.0
        );
        pr_info!("PAGE_SIZE = {}", PAGE_SIZE);
        unsafe { log_physical_pages(page_allocation.0, amount_pages_to_alloc as usize, true) };
        self.allocs.try_push(page_allocation)?;

        Ok(())
    }
}

impl KernelModule for BSA {
    fn init() -> Result<Self> {
        let mut bsa = BSA::try_new()?;
        bsa.run(*alloc_order.read())?;
        Ok(bsa)
    }
}
