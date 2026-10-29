#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod app;
mod std;
mod syscalls;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn _start() -> ! {
    let res = app::main::main();
    syscalls::exit(res as i64);
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    syscalls::exit(12);
    loop {}
}
