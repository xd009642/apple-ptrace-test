extern crate nix;
extern crate mach;

use nix::sys::ptrace;
use nix::sys::signal::{raise, Signal};
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::fork;
use nix::unistd::ForkResult::*;

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
            assert_eq!(waitpid(child, None), Ok(WaitStatus::Stopped(child, Signal::SIGTRAP)));
            ptrace::cont(child, None).unwrap();
            assert_eq!(waitpid(child, None), Ok(WaitStatus::Stopped(child, Signal::SIGTRAP)));
            ptrace::cont(child, Signal::SIGKILL).unwrap();
            // child dead
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
