# rustless
<h1>sandbox</h1><br>
<h2>5/4/2023:</h2><br>
Simple Rust Binary in progress<br> 
Current Status: <br>
Basic binary is buildable with <br>
cargo build --target thumbv7em-none-eabihf<br>
Alternatively can compile with: <br>
cargo rustc -- -C link-arg=-nostartfiles <br>
<h4>Some Notes on Kernel approach <h4><br>
On boot, the machine loads Basic IO System (BIOS) <br>
*** UEFI Unsupported *** <br>
BIOS finds a bootable disc, and control is transferred to bootloader.  <br>
Bootloader must determine kernel image on disk and load into memory. <br>
Passes memory map into OS Kernel <br>
Disc Image will print a statement when booted by extending binary <br>
Using nightly rust channel <br>
<h6> Target SpecificatioN <h6> <br>
Special configuration parameters are to be set via JSON File, <br>
Kernel target specifies Os as none to run on bare metal.<br>
@5/9/2023<br>
I can compile with target specifications, but only after configuring build-std to recompile core and compiler_builtins.  Also, I received many errors using stable rust channel and required nightly channel.<br>
To compile, must redirect target to the JSON Specification as such: <br>
cargo build --target x86_64-rustless.json

