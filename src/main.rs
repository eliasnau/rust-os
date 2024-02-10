#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

mod boot {
    use core::arch::global_asm;

    global_asm!(".section .text._start");
}

pub extern "C" fn _start() -> ! {
    unsafe {
        core::ptr::write_volatile(0x3F20_0008 as *mut u32, 1 << 3);

        loop {
            core::ptr::write_volatile(0x3F20_001C as *mut u32, 1 << 21);
            for _ in 1..50000 {
                asm!("nop");
            }
            core::ptr::write_volatile(0x3F20_0028 as *mut u32, 1 << 21);
            for _ in 1..50000 {
                asm!("nop");
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
