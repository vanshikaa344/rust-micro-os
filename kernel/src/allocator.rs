
use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicUsize, Ordering};

const HEAP_SIZE: usize = 64 * 1024;

struct Heap(UnsafeCell<[u8; HEAP_SIZE]>);

unsafe impl Sync for Heap {}

static HEAP: Heap = Heap(UnsafeCell::new([0; HEAP_SIZE]));

pub struct BumpAllocator {
    next: AtomicUsize,
}

impl BumpAllocator {
    pub const fn new() -> Self {
        Self {
            next: AtomicUsize::new(0),
        }
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let heap_start = HEAP.0.get() as *mut u8 as usize;

        let mut current = self.next.load(Ordering::Relaxed);

        loop {
            let aligned = match (heap_start + current)
                .checked_add(layout.align() - 1)
            {
                Some(value) => value & !(layout.align() - 1),
                None => return core::ptr::null_mut(),
            };

            let offset = aligned - heap_start;

            let Some(end) = offset.checked_add(layout.size()) else {
                return core::ptr::null_mut();
            };

            if end > HEAP_SIZE {
                return core::ptr::null_mut();
            }

            match self.next.compare_exchange_weak(
                current,
                end,
                Ordering::SeqCst,
                Ordering::Relaxed,
            ) {
                Ok(_) => return aligned as *mut u8,
                Err(actual) => current = actual,
            }
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Bump allocator: individual deallocation is not supported.
    }
}
