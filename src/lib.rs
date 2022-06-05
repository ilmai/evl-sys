#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use bitflags::bitflags;

bitflags! {
    #[derive(Default)]
    pub struct CloneFlags: u32 {
        const PRIVATE = 0;	      // EVL_CLONE_PRIVATE
        const PUBLIC = (1 << 16);     // EVL_CLONE_PUBLIC
        const OBSERVABLE = (1 << 17); // EVL_CLONE_OBSERVABLE
        const NONBLOCK = (1 << 18);   // EVL_CLONE_NONBLOCK
        const UNICAST = (1 << 19);    // EVL_CLONE_UNICAST
        const INPUT = (1 << 20);      // EVL_CLONE_INPUT
        const OUTPUT = (1 << 21);     // EVL_CLONE_OUTPUT
    }
}

bitflags! {
    #[derive(Default)]
    pub struct MutexType: u32 {
        const NORMAL = 0;           	// EVL_MUTEX_NORMAL
        const RECURSIVE = (1 << 0); 	// EVL_MUTEX_RECURSIVE
    }
}

#[derive(Copy, Clone)]
pub enum BuiltinClock {
    MONOTONIC = -libc::CLOCK_MONOTONIC as isize,
    REALTIME = -libc::CLOCK_REALTIME as isize,
}

#[derive(Copy, Clone)]
pub enum SchedPolicy {
    FIFO = libc::SCHED_FIFO as isize,
    RR = libc::SCHED_RR as isize,
    // As defined in include/uapi/evl/sched.h:
    WEAK = 43,
    QUOTA = 44,
    TP = 45,
}
