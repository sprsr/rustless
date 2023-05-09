// No std env
#![no_std]
// Diverty entry point to Rust runtime
#![no_main]

//main.rs
//No_std environment requires a panic_handler definition
use core::panic::PanicInfo;
//Panic Handler
// PanicInfo contains file and line where panic happened
// and the optional panic message.  the function won't return
// diverging function, returning never
// For now this routine does nothing
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
// Disable name mangling ensures compiler outputs function _start.
// Without, compiler would generate cryptic symbols
#[no_mangle]
// We must extern C to tell compiler to use C calling convention
// Returning ! ~ Diverting function.
// Instead of return, we exit :)
pub extern "C" fn _start() -> ! {
    loop {}
}

