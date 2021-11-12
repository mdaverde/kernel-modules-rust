//! A kernel module
#![no_std]
#![feature(allocator_api, global_asm)]

use kernel::prelude::*;
use kernel::sync;
use kernel::task::{ProcessIterator, Task};

module! {
    type: ProcIterModule,
    name: b"proc_iter",
    author: b"milan@mdaverde.com",
    description: b"A kernel module",
    license: b"Dual MIT/GPL",
}

struct ProcIterModule;

fn print_thread(task: &Task) {
    use kernel::bindings;
    use kernel::c_str;
    use kernel::str::CStr;

    const BUF_SIZE: usize = (bindings::TASK_COMM_LEN as usize) + 2;
    let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];

    let comm = task.comm().as_bytes_with_nul();

    let name = if let Some(_) = task.mm() {
        comm
    } else {
        buf[0] = b'['; // bracket to denote kernel thread
        let comm_len = comm.len() - 1; // remove nul byte

        for (i, ch) in comm[0..comm_len].into_iter().enumerate() {
            buf[i + 1] = *ch;
        }

        buf[comm_len + 1] = b']'; // end bracket; should be nul terminated from init
        &buf[0..(comm_len + 3)]
    };

    let name = CStr::from_bytes_with_nul(name)
        .unwrap_or(c_str!("<nul byte err>"))
        .to_str()
        .unwrap_or("<non-UTF comm>");

    // lock task?
    pr_info!("| {:>5} | {:>5} | {:>17}", task.tgid(), task.pid(), name);
}

fn show_processes() {
    pr_info!("| {:>5} | {:>5} | {:>17}", "TGID", "PID", "name");

    let proc_iter = ProcessIterator::new();
    // let first_proc = proc_iter.next(); Can test PhantomData?
    // logs first N tasks; make N a module parameter
    for (_, proc) in (0..1000).zip(proc_iter) {
        for thread in proc.threads() {
            // if *proc != *thread {
            print_thread(&thread);
        }
        sync::cond_resched(); // Should live in mod sync?
    }
}

impl KernelModule for ProcIterModule {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        show_processes();
        Ok(ProcIterModule)
    }
}
