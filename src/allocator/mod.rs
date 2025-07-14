pub mod bump;
pub mod fixed_size_block;
pub mod linked_list;

use bootloader::bootinfo::MemoryMap;
//use linked_list_allocator::LockedHeap;
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};

use crate::memory;

pub const HEAP_START: usize = 0x4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

// static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();
// use bump::BumpAllocator;
// use linked_list::LinkedListAllocator;
use fixed_size_block::FixedSizeBlockAllocator;
#[global_allocator]
static HEAP_ALLOCATOR: Locked<FixedSizeBlockAllocator> =
    Locked::new(FixedSizeBlockAllocator::new());

pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Self {
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}

/// Aligns the given `addr` upwards to alignment `align`.
fn align_up(addr: usize, align: usize) -> usize {
    let remainder = addr % align;
    if remainder == 0 {
        addr
    } else {
        addr - remainder + align
    }
}

/// A faster implementation of `align_up` method.
fn fast_align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

/// # Errors
///
/// Fails if no frame left, `HUGE_PAGE` are in use or the given page is already
/// mapped to a physical frame.
pub fn init_heap(
    physical_memory_offset: u64,
    memory_map: &'static MemoryMap,
) -> Result<(), MapToError<Size4KiB>> {
    let phys_mem_offset = VirtAddr::new(physical_memory_offset);
    let mut mapper = unsafe {
        // SAFETY: complete physical memory is mapped to virtual memory
        // at the provided offset, also it is called once here.
        memory::init(phys_mem_offset)
    };
    let mut frame_allocator = unsafe {
        // SAFETY: `memory_map` is provided by the boot info.
        memory::BootInfoFrameAllocator::new(memory_map)
    };

    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1_u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            mapper
                .map_to(page, frame, flags, &mut frame_allocator)?
                .flush();
        }
    }

    unsafe {
        // SAFETY: at this point only this method
        // has access to the static variable.
        HEAP_ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}
