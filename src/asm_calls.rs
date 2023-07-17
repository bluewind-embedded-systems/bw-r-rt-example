// This module implements some assembler routines

#[cfg(not(target_arch = "tricore"))]
mod arch_impl {
    pub fn read_cpu_core_id() -> u32 {
        0
    }

    // global interrupts enable
    pub fn enable_interrupts() {}

    // global interrupts disable
    pub fn disable_interrupts() {}
}

#[cfg(target_arch = "tricore")]
mod arch_impl {
    use core::arch::asm;

    // global interrupts enable
    pub fn enable_interrupts() {
        unsafe {
            asm!("enable");
        }
    }

    // global interrupts disable
    pub fn disable_interrupts() {
        unsafe {
            asm!("disable");
        }
    }

    /** \brief FE1C, CPUx Core Identification Register */
    #[allow(dead_code)]
    const CPU_CORE_ID: u32 = 0xFE1C;

    /**
     * Read CPU core id.
     */
    pub fn read_cpu_core_id() -> u32 {
        #[allow(unused_assignments)]
        let value: u32;
        unsafe {
            asm!("mfcr {0}, 0xFE1C", out(reg32) value);
        }

        value
    }
}

pub use arch_impl::*;
