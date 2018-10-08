extern crate nix;
extern crate mach;

use std::mem;

use nix::sys::ptrace;
use nix::sys::signal::{raise, Signal};
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::fork;
use nix::unistd::ForkResult::*;

use mach::traps::{mach_task_self, task_for_pid};
use mach::mach_port::*;
use mach::port::*;

pub fn try_it_out() {
    println!("Trying out an experiment");
    match fork().expect("Error, fork failed") {
        Child => {
            ptrace::traceme().unwrap();
            loop {
                raise(Signal::SIGTRAP).unwrap();
            }
        },
        Parent{child} => {
            unsafe {
            let mut port: mach_port_name_t =  mem::uninitialized() ;
            task_for_pid(mach_task_self(), child.into(), port as *mut mach_port_name_t);
            assert_eq!(waitpid(child, None), Ok(WaitStatus::Stopped(child, Signal::SIGTRAP)));
            ptrace::cont(child, None).unwrap();
            assert_eq!(waitpid(child, None), Ok(WaitStatus::Stopped(child, Signal::SIGTRAP)));
            ptrace::cont(child, Signal::SIGKILL).unwrap();
            // child dead
            //
            mach_port_deallocate(mach_task_self(), port);
            }
        }
    }
    println!("If you read this I've worked");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
