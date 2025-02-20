#![no_std]
#![no_main]

use fasila::Interval32;

use core::panic::PanicInfo;

#[no_mangle]
fn _my_start() -> ! {
    let a = Interval32::new(-1.0, 1.0);
    let b = Interval32::new(-2.0, 2.0);
    let c = a.add(b);
    core::hint::black_box(c);
    unsafe { riscv::asm::ebreak() };
    loop { }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop { }
}

