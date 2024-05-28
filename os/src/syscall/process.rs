//! Process management syscalls
use crate::{
    syscall::{SYSCALL_EXIT, SYSCALL_YIELD, SYSCALL_GET_TIME, SYSCALL_TASK_INFO},
    task::{exit_current_and_run_next, suspend_current_and_run_next, TaskInfo, TASK_MANAGER},
    timer::{get_time_us, MSEC_PER_SEC},
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    TASK_MANAGER.update_task_info(SYSCALL_EXIT, -1);
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    TASK_MANAGER.update_task_info(SYSCALL_YIELD, -1);
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    TASK_MANAGER.update_task_info(SYSCALL_GET_TIME, (us / MSEC_PER_SEC) as isize);
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    TASK_MANAGER.update_task_info(SYSCALL_TASK_INFO, -1);
    trace!("kernel: sys_task_info");
    TASK_MANAGER.get_task_info(_ti);
    0
}

















