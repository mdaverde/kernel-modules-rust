#![no_std]
#![feature(allocator_api, global_asm)]

use kernel::prelude::*;
use kernel::task::Task;
use kernel::c_types;

// TODO: get rid of
use kernel::bindings;
struct CurrentAffairs;

fn show_context() -> Result<()> {
    if unsafe { bindings::in_task() } {
        pr_info!("current_affairs: in task!");

        let current_task = Task::current();
        let pid: bindings::pid_t = current_task.pid();
        let tgid: bindings::pid_t = current_task.tgid();
        let name = current_task.comm();

        // Move into Task?
        let init_user_ns: *mut bindings::user_namespace =
            unsafe { &mut bindings::init_user_ns as *mut bindings::user_namespace };
        let task_uid: bindings::kuid_t = unsafe { bindings::current_uid() };
        let task_euid: bindings::kuid_t = unsafe { bindings::current_euid() };
        let uid: bindings::uid_t = unsafe { bindings::from_kuid(init_user_ns, task_uid) };
        let euid: bindings::uid_t = unsafe { bindings::from_kuid(init_user_ns, task_euid) };

        // TODO: name prints out array in debug form. need to fix this
        pr_info!(
            "{}: in process context ::
            PID  : {}
            TGID : {}
            UID  : {}
            EUID : {} ({} root)
            name : {:?}
        ",
            "current_affairs",
            pid,
            tgid,
            uid,
            euid,
            if euid == 0 { "have" } else { "don't have" },
            name
        );
    } else {
        pr_alert!("current_affairs: in interrupt context!");
    }
    Ok(())
}

module! {
    type: CurrentAffairs,
    name: b"current_affairs",
    author: b"milan@mdaverde.com",
    description: b"Display a few members of the current process task structure",
    license: b"Dual MIT/GPL",
}

impl KernelModule for CurrentAffairs {
    fn init() -> Result<Self> {
        pr_info!("inserted\n");
        pr_info!("size_of(kernel::Task)={}\n", core::mem::size_of::<Task>());
        // let current = Task::current();
        show_context()?;

        Ok(CurrentAffairs)
    }
}

impl Drop for CurrentAffairs {
    fn drop(&mut self) {
        show_context().unwrap();
        pr_info!("Bye world from rust!\n");
    }
}
