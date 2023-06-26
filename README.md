# rustless
<h1>sandbox</h1><br>
<h2>5/4/2023:</h2><br>
Simple Rust Binary lays foundation<br> 
Basic binary is buildable with
cargo build --target thumbv7em-none-eabihf<br>
Alternatively can compile with: <br>
cargo rustc -- -C link-arg=-nostartfiles <br>
<h4>Some Notes on Kernel approach </h4><br>
On boot, the machine loads Basic IO System (BIOS) <br>
*** UEFI Unsupported *** <br>
BIOS finds a bootable disc, and control is transferred to bootloader.  <br>
Bootloader must determine kernel image on disk and load into memory. <br>
Passes memory map into OS Kernel <br>
Disc Image will print a statement when booted by extending binary <br>
Using nightly rust channel <br>
<h4> Target Specification </h4> <br>
Special configuration parameters are to be set via JSON File, <br>
Kernel target specifies Os as none to run on bare metal.<br>
@5/9/2023<br>
I can compile with target specifications, but only after configuring build-std to recompile core and compiler_builtins.  Also, I received many errors using stable rust channel and required nightly channel.<br>
To compile, must redirect target to the JSON Specification as such: <br>
cargo build --target x86_64-rustless.json<br>
Now configured target so target argument can be voided <br>
<h4> Printing from Kernel build </h4>
@5/10/2024<br>
Implemented basic driver for VGA Buffer.  You can now:
use bootimage to compile kernel to an ELF file, compile bootloader dependency as a standalone executable, and link bytes of the kernel ELF file to the bootloader. <br>
With host OS as Linux, I am able to boot the disk image with QEMU Virtual Machine
Use the following command: <br>
</t> > qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin 
<h4> Printing <h4>
Users can now print with start function simply

