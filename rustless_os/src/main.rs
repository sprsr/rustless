// No std env
#![no_std]
// Diverty entry point to Rust runtime
#![no_main]

//main.rs
//No_std environment requires a panic_handler definition
// VGA Buffer
mod vga_buffer;
use core::panic::PanicInfo;
//Panic Handler
// PanicInfo contains file and line where panic happened
// and the optional panic message.  the function won't return
// diverging function, returning never
// For now this routine does nothing
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}",info);
    loop {}
}

//static STRANGER: &[u8] = b"Stranger Danger!";
// Disable name mangling ensures compiler outputs function _start.
// Without, compiler would generate cryptic symbols
#[no_mangle]
// We must extern C to tell compiler to use C calling convention
// Returning ! ~ Diverting function.
// Instead of return, we exit :)
pub extern "C" fn _start() -> ! {
    // Casting integer 0xb8000 into raw pointer 
    //let vga_buffer = 0xb8000 as *mut u8;
    // Iterating over the bytes in STRANGER and enumerate to get running variable i.
   /* 
    for(i, &byte) in STRANGER.iter().enumerate() {
        // Compiler doesn't know if our pointers are valid.  
        // Unsafe block tells compiler we
        // are sure the operations are valid. 
        unsafe {
            // Use offset to write string byte and corresponding color byte.
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    */
    //vga_buffer::print_string();
    //use core::fmt::Write;
    //vga_buffer::WRITER.lock().write_str("Testing Hello").unwrap();
    //write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();
    println!("Welcome to the Rust Kernel!{}", "!");
    panic!("testing the panic print statement");
    loop {}
}

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]){
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
