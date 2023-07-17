//! Some basic utilities
use core::arch::asm;

/// Inner result type (later add better errors)
pub type Result<T> = core::result::Result<T, ()>;

/// Checked wait under a condition that panics with message if the condition
/// is not true within some 100_000k cycles
///
/// This is super random and copied from TriCore examples -> at least gives
/// a small idea if something works
pub fn wait(closure: impl Fn() -> bool) -> Result<()> {
    wait_cycles(closure, 100_000)
}

pub fn wait_cycles(condition: impl Fn() -> bool, max_wait_cycles: usize) -> Result<()> {
    for _ in 0..max_wait_cycles {
        unsafe { asm!("nop") };
        if condition() {
            return Ok(());
        }
    }
    
    Err(())
}

/// Basic nop-loop to wait some cycles where needed
pub fn wait_nop(cycle: u32) {
    for _ in 0..cycle {
        unsafe { asm!("nop") };
    }
}

#[macro_export]
macro_rules! block_while_nops {
    ($condition:expr, $log_statement:tt) => {
        block_while_nops!($condition, nops=100_000, $log_statement)
    };
    ($condition:expr, nops = $wait_cycles:expr, $log_statement:tt) => {
        {
            let condition = || $condition;
            let mut exit_success = false;
            let wait_cycles = $wait_cycles;
            for _ in 0..wait_cycles {
                unsafe { ::core::arch::asm!("nop") };
                if condition() {
                    exit_success = true;
                    break;
                }
            }
            drop(wait_cycles);
            if !exit_success {
                drop(exit_success);
                panic!($log_statement);
            }
        }
    };
}