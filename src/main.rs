#![no_std]
#![no_main]

#![feature(link_llvm_intrinsics)]
#![feature(abi_unadjusted)]

use core::panic::PanicInfo;

extern "unadjusted" {
    #[link_name = "llvm.riscv.intadd.s"]
    fn intadd_s(a: f64, b: f64) -> f64;
}

pub unsafe fn __intadd_s(a: f64, b: f64) -> f64 {
    intadd_s(a, b)
}

#[no_mangle]
fn _start() -> ! {
    let a = core::hint::black_box(0.007812505573383532); // [-1.0, 1.0]
    let b = core::hint::black_box(2.0000014305114746); // [-2.0, 2.0]
    let c = unsafe { core::hint::black_box(__intadd_s(a, b)) };
    core::hint::black_box((a, b, c));
    unsafe { riscv::asm::ebreak() };
    loop { }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop { }
}

