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
use alloc::vec;

use panic_halt as _;
use alloc_cortex_m::CortexMHeap;
// use embedded_alloc::Heap;
use cortex_m_rt::entry;
use cortex_m_semihosting::{hprintln, debug};

// this is the allocator the application will use
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
// static HEAP: Heap = Heap::empty();

#[entry]
fn main() -> ! {
    // Initialize the allocator BEFORE you use it
    unsafe {
        const HEAP_SIZE: usize = 1024;
        ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE);
    }

    // {
    //     use core::mem::MaybeUninit;
    //     const HEAP_SIZE: usize = 1024;
    //     static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    //     unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    // }

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
