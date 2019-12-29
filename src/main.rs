#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

use core::panic::PanicInfo;

use core::fmt::Write;
use core::ptr::{read_volatile, write_volatile};
//use core::str;

//use heapless::consts::*;
//use heapless::String;

extern crate linked_list_allocator;
use linked_list_allocator::*;
#[global_allocator]
static mut HEAP: LockedHeap = LockedHeap::empty();

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use riscv::register::time;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    hprint("Hello World\n");

    const HEAP_SIZE: usize = 1024;
    static mut HEAP_AREA: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
    unsafe { HEAP = LockedHeap::new(&HEAP_AREA[0] as *const u8 as usize, 4096) };

    for i in 1..10 {
        writeln!(UART, "i: {} 1/i: {}", i, 1.0 / i as f32).ok();
        writeln!(UART, "{}", time::read64()).ok();
    }

    let mut xs = Vec::new();
    let mut sv = Vec::new();

    for i in 1..1000 {
        xs.push(i);
        let mut s = String::new();
        writeln!(s, "{}", i).unwrap();
        if i % 100 == 0 {
            hprint(&s);
        }
        if i % 50 == 0 {
            sv.push(500 - i);
        }
        if xs.pop().unwrap() != i {
            panic!("???");
        }
    }

    let start = time::read64(); 
    sv.sort();
    let done = time::read64(); 
    writeln!(UART, "{:?} in {}", sv, done - start).ok();

    panic!("End of main()");
}

/* uart */
struct UART;

impl core::fmt::Write for UART {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        for c in s.bytes() {
            uart_rxtx_write(c);
        }
        Ok(())
    }
}

const CSR_UART_BASE: *mut u8 = (0xf0001000) as *mut u8;
const CSR_UART_TXFULL_ADDR: *mut u8 = (0xf0001004) as *mut u8;

fn uart_rxtx_write(value: u8) {
    unsafe {
        while uart_txfull_read() {}
        write_volatile(CSR_UART_BASE, value);
    }
}

fn uart_txfull_read() -> bool {
    unsafe { read_volatile(CSR_UART_TXFULL_ADDR) != 0 }
}

fn hprint(s: &str) {
    for c in s.bytes() {
        uart_rxtx_write(c);
    }
}

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    hprint("\npanic!\n");
    writeln!(UART, "{}", panic_info).ok();
    loop {
        // atomic::compiler_fence(Ordering::SeqCst);
    }
}

#[alloc_error_handler]
fn alloc_error(_: core::alloc::Layout) -> ! {
    //fn alloc_error(_layout: Layout) -> ! {
    panic!("Heap");
}
