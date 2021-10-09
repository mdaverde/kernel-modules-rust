//! A kernel module
#![no_std]
#![feature(allocator_api, global_asm)]
use kernel::prelude::*;
use kernel::task::Task;

// TODO: get rid of
use kernel::bindings;
struct CurrentModule;

fn show_context() -> Result<()> {
    if unsafe { bindings::in_task() } {
        let current_task = Task::current();
        let pid = current_task.pid();
        let tgid = current_task.tgid();
        let name = current_task.comm();
        let uid = current_task.uid();
        let euid = current_task.euid();

        // TODO: should kernel::task::Task have a debug interface? Does the kernel already have this?
        pr_info!(
            "In process context:
            PID  : {}
            TGID : {}
            UID  : {}
            EUID : {} ({} root)
            name : {}
        ",
            pid,
            tgid,
            uid,
            euid,
            if euid == 0 { "as" } else { "not as" },
            name.to_str().unwrap(),
        );
    } else {
        pr_alert!("in interrupt context!");
    }
    Ok(())
}

module! {
    type: CurrentModule,
    name: b"current",
    author: b"milan@mdaverde.com",
    description: b"Display a few members of the current process task structure",
    license: b"Dual MIT/GPL",
}

impl KernelModule for CurrentModule {
    fn init() -> Result<Self> {
        show_context()?;
        Ok(CurrentModule)
    }
}