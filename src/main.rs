#![feature(riscv_ext_intrinsics)]

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::riscv64::intadd_s;

#[no_mangle]
fn _start() -> ! {
    let a = 0.007812505573383532; // [-1.0, 1.0]
    let b = 2.0000014305114746;   // [-2.0, 2.0]
    let c = unsafe { intadd_s(a, b) };
    core::hint::black_box(c);
    unsafe { riscv::asm::ebreak() };
    loop { }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop { }
}

