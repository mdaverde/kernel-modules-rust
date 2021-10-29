//! A kernel module
#![no_std]
#![feature(allocator_api, global_asm)]
use kernel::preempt;
use kernel::prelude::*;
use kernel::task::{Task, TaskRef};

module! {
    type: CurrentModule,
    name: b"current",
    author: b"milan@mdaverde.com",
    description: b"Display a few members of the current process task structure",
    license: b"Dual MIT/GPL",
}

struct CurrentModule;

// To get around the orpan rule
struct DebugTaskWrapper<'taskref, 'a>(&'taskref TaskRef<'a>);

impl<'taskref, 'a> core::fmt::Debug for DebugTaskWrapper<'taskref, 'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = &self.0.comm().to_str().unwrap_or("<utf8 err>");
        f.debug_struct("Task")
            .field("name", name)
            .field("pid", &self.0.pid())
            .field("tgid", &self.0.tgid())
            .field("uid", &self.0.uid())
            .field("euid", &self.0.euid())
            .finish_non_exhaustive()
    }
}

fn show_context() -> Result<()> {
    if preempt::in_task() {
        let current_task = Task::current();
        pr_info!(
            "In process context: {:#?}\n",
            DebugTaskWrapper(&current_task)
        );
    } else {
        pr_alert!("In interrupt context\n");
    }
    Ok(())
}

impl KernelModule for CurrentModule {
    fn init() -> Result<Self> {
        show_context()?;
        Ok(CurrentModule)
    }
}
