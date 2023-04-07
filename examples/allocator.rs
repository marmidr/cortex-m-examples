//! How to use the heap and a dynamic memory allocator
//!
//! This example depends on the alloc-cortex-m crate so you'll have to add it to your Cargo.toml:
//!
//! ``` text
//! # or edit the Cargo.toml file manually
//! $ cargo add alloc-cortex-m
//! ```
//!
//! ---

/* before 1.68
#![feature(alloc_error_handler)]
*/

#![no_main]
#![no_std]

extern crate alloc;
use panic_halt as _;

use alloc::vec;

use alloc_cortex_m::CortexMHeap;
use cortex_m_rt::entry;
use cortex_m_semihosting::{hprintln, debug};

// this is the allocator the application will use
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const HEAP_SIZE: usize = 1024; // in bytes

#[entry]
fn main() -> ! {
    // Initialize the allocator BEFORE you use it
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

    // Growable array allocated on the heap
    let xs = vec![0, 1, 2];

    hprintln!("{:?}", xs);

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}

// define what happens in an Out Of Memory (OOM) condition
/* before 1.68
#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    asm::bkpt();

    loop {}
}
*/
