//! A kernel module
#![no_std]
#![feature(allocator_api, global_asm)]

use alloc::boxed::Box;
use core::{ptr, slice};
use kernel::prelude::*;

struct KAllocator;

unsafe impl alloc::alloc::Allocator for KAllocator {
    fn allocate(
        &self,
        layout: alloc::alloc::Layout,
    ) -> core::result::Result<ptr::NonNull<[u8]>, alloc::alloc::AllocError> {
        use kernel::bindings;

        let size = layout.size();
        let kmalloc_ptr = unsafe { bindings::__kmalloc(size, bindings::GFP_KERNEL) as *mut u8 };
        pr_info!(
            "__kmalloc({}, GFP_KERNEL) returned {:?}\n",
            size,
            kmalloc_ptr
        );
        let slab = unsafe { slice::from_raw_parts_mut(kmalloc_ptr, size) };
        let alloc_ptr = slab as *mut [u8];
        ptr::NonNull::new(alloc_ptr).ok_or(alloc::alloc::AllocError)
    }

    unsafe fn deallocate(&self, ptr: ptr::NonNull<u8>, _layout: alloc::alloc::Layout) {
        use kernel::{bindings, c_types};

        pr_info!("kfree({:?})\n", ptr);
        unsafe { bindings::kfree(ptr.as_ptr() as *const c_types::c_void) };
    }
}

struct KmallocBox {
    buf: Option<Box<[u8; 1024], KAllocator>>,
}

impl KmallocBox {
    fn new() -> Self {
        KmallocBox { buf: None }
    }
    fn run(&mut self) -> Result<()> {
        let buf: [u8; 1024] = [0; 1024];
        let mut boxed_buf = Box::try_new_in(buf, KAllocator)?;

        for b in boxed_buf.iter_mut() {
            *b = 'm' as u8;
        }

        // Will be freed on KmallocBox drop
        self.buf = Some(boxed_buf);

        Ok(())
    }
}

module! {
    type: KmallocBox,
    name: b"kmalloc_box",
    author: b"milan@mdaverde.com",
    description: b"A kernel module",
    license: b"Dual MIT/GPL",
}

impl KernelModule for KmallocBox {
    fn init() -> Result<Self> {
        let mut kmalloc_box = KmallocBox::new();
        kmalloc_box.run()?;
        Ok(kmalloc_box)
    }
}
