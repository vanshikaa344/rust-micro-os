
#![no_std]
#![no_main]

mod allocator;
mod scheduler;
mod sync;

use core::fmt::Write;
use core::panic::PanicInfo;
use core::alloc::{GlobalAlloc, Layout};

use bootloader_api::BootInfo;
use scheduler::{ReadyQueue, Task};

#[global_allocator]
static ALLOCATOR: allocator::BumpAllocator =
    allocator::BumpAllocator::new();

bootloader_api::entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    let mut serial = unsafe {
        uart_16550::SerialPort::new(0x3F8)
    };

    unsafe {
        serial.init();
    }

    writeln!(serial, "\n==============================").unwrap();
    writeln!(serial, "       RUST MICRO-OS").unwrap();
    writeln!(serial, "==============================").unwrap();

    writeln!(serial, "[BOOT] Kernel started successfully").unwrap();

    // Memory allocator demonstration
    let layout = Layout::from_size_align(128, 8).unwrap();

    let memory = unsafe {
        ALLOCATOR.alloc(layout)
    };

    if !memory.is_null() {
        unsafe {
            memory.write(42);
        }
        writeln!(serial, "[MEMORY] Allocated 128 bytes").unwrap();
    } else {
        writeln!(serial, "[MEMORY] Allocation failed").unwrap();
    }

    // Synchronization demonstration
    let lock = sync::SpinLock::new();
    lock.lock();
    writeln!(serial, "[SYNC] Spinlock acquired").unwrap();
    lock.unlock();
    writeln!(serial, "[SYNC] Spinlock released").unwrap();

    // Create ready queue
    let mut queue = ReadyQueue::new();

    queue.push(Task { id: 1 });
    queue.push(Task { id: 2 });
    queue.push(Task { id: 3 });

    writeln!(serial, "[SCHEDULER] Ready queue initialized").unwrap();

    // Demonstrate Round Robin scheduling
    for tick in 1..=9 {
        if let Some(task) = scheduler::round_robin(&mut queue) {
            writeln!(
                serial,
                "[SCHEDULER] Tick {}: Running Task {}",
                tick,
                task.id
            ).unwrap();
        }
    }

    writeln!(serial, "[KERNEL] Execution complete").unwrap();

    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut serial = unsafe {
        uart_16550::SerialPort::new(0x3F8)
    };

    unsafe {
        serial.init();
    }

    let _ = writeln!(serial, "\n[KERNEL PANIC] {}", info);

    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}
