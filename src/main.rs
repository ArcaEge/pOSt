#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod vga_buffer;

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vga_buffer::WRITER.lock().colour_code = vga_buffer::ColourCode::new(vga_buffer::Colour::White, vga_buffer::Colour::Blue);
    println!();
    println!("KERNEL PANIC: {}", info);
    loop {}
}

// Entry point
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    panic!("Panic test");

    loop {}
}