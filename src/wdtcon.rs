// lock/unlock the EDNINIT bit in the cpu WDTCON and safety WDTCON
// to access ENDINIT protected SFRs such as BIV, BTY, CLC.

use pac::RegValue;
use tc37x_pac as pac;

#[cfg(target_arch = "tricore")]
#[inline]
fn dsync() {
    use core::arch::asm;
    unsafe {
        asm! {"dsync"}
    };
}

#[cfg(not(target_arch = "tricore"))]
#[inline]
fn dsync() {}

/// Clears endinit bit for  protection against unintentional modifications.
/// See section 11.4 of AURIXTM TC3xx Target specification
pub fn clear_safety_endinit() {
    let mut passwd: u32 = unsafe { pac::SCU.wdtscon0().read() }.data();
    passwd &= 0xffffff00;

    unsafe {
        pac::SCU.wdtscon0().init(|mut w| {
            *w.data_mut_ref() = passwd | 0xf1;
            w
        })
    };

    dsync();

    unsafe {
        pac::SCU.wdtscon0().init(|mut w| {
            *w.data_mut_ref() = passwd | 0xf2;
            w
        })
    };

    // read back new value >
    let _ = unsafe { pac::SCU.wdtscon0().read() }.data();
}

/// Sets endinit bit for  protection against unintentional modifications.
pub fn set_safety_endinit() {
    let mut passwd = unsafe { pac::SCU.wdtscon0().read() }.data();
    passwd &= 0xffffff00;

    unsafe {
        pac::SCU.wdtscon0().init(|mut w| {
            *w.data_mut_ref() = passwd | 0xf1;
            w
        })
    };

    dsync();

    passwd |= 3;
    unsafe {
        pac::SCU.wdtscon0().init(|mut w| {
            *w.data_mut_ref() = passwd | 0xf2;
            w
        })
    };

    // read back new value >
    let _ = unsafe { pac::SCU.wdtscon0().read() }.data();
}

/// Clears endinit bit for  protection against unintentional modifications for CPU0 core.
/// See section 11.4 of AURIXTM TC3xx Target specification
pub fn clear_cpu_endinit() {
    let mut passwd = unsafe { pac::SCU.wdtcpu0con0().read() }.data();
    passwd &= 0xffffff00;

    unsafe {
        pac::SCU.wdtcpu0con0().init(|mut w| {
            *w.data_mut_ref() = passwd | 0xf1;
            w
        })
    };

    dsync();

    unsafe {
        pac::SCU.wdtcpu0con0().init(|mut w| {
            *w.data_mut_ref() = passwd | 0xf2;
            w
        })
    };

    // read back new value >
    let _ = unsafe { pac::SCU.wdtcpu0con0().read() }.data();
}

/// Sets endinit bit for  protection against unintentional modifications for current core.
pub fn set_cpu_endinit() {
    let mut passwd = unsafe { pac::SCU.wdtcpu0con0().read() }.data();
    passwd &= 0xffffff00;

    unsafe {
        pac::SCU.wdtcpu0con0().init(|mut w| {
            *w.data_mut_ref() = passwd | 0xf1;
            w
        })
    };

    dsync();

    passwd |= 3;
    unsafe {
        pac::SCU.wdtcpu0con0().init(|mut w| {
            *w.data_mut_ref() = passwd | 0xf2;
            w
        })
    };

    // read back new value >
    let _ = unsafe { pac::SCU.wdtcpu0con0().read() }.data();
}

/// Disable safety watchdog. The Safety Watchdog Timer provides an overall system level watchdog which is independent from the CPU Watchdog Timers
/// See section 11.4 of AURIXTM TC3xx Target specification
#[no_mangle]
pub fn disable_safety_watchdog() {
    clear_safety_endinit();
    unsafe { pac::SCU.wdtscon1().modify(|p| p.dr().set(true)) };
    set_safety_endinit();
}

/// Disable safety watchdog for CPU0 core.
/// See section 11.4 of AURIXTM TC3xx Target specification
#[no_mangle]
pub fn disable_cpu_watchdog() {
    clear_cpu_endinit();
    unsafe { pac::SCU.wdtcpu0con1().modify(|p| p.dr().set(true)) };
    set_cpu_endinit();
}
