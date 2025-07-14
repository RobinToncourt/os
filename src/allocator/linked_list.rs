use alloc::alloc::{GlobalAlloc, Layout};
use core::mem;
use core::ptr;

use super::Locked;
use crate::allocator::fast_align_up;

#[derive(Default)]
struct ListNode {
    size: usize,
    next: Option<&'static mut ListNode>,
}

impl ListNode {
    #[must_use]
    const fn new(size: usize) -> Self {
        Self { size, next: None }
    }

    fn get_start_addr(&self) -> usize {
        core::ptr::from_ref(self) as usize
    }

    fn get_end_addr(&self) -> usize {
        self.get_start_addr() + self.size
    }
}

#[derive(Default)]
pub struct LinkedListAllocator {
    head: ListNode,
}

impl LinkedListAllocator {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            head: ListNode::new(0),
        }
    }

    /// Initialize the allocator with the given heap bounds.
    ///
    /// # Safety
    ///
    /// This function is unsafe because the caller must guarantee that the given
    /// heap bounds are valid and that the heap is unused. This method must be
    /// called only once.
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        unsafe {
            self.add_free_region(heap_start, heap_size);
        }
    }

    /// Adds the given memory region to the front of the list.
    unsafe fn add_free_region(&mut self, addr: usize, size: usize) {
        // Ensure that the freed region is capable of holding a `ListNode`.
        assert_eq!(fast_align_up(addr, mem::align_of::<ListNode>()), addr);
        assert!(size >= mem::size_of::<ListNode>());

        // Create a new `ListNode` and append it at the start of the list.
        let mut node = ListNode::new(size);
        node.next = self.head.next.take();
        let node_ptr = addr as *mut ListNode;
        unsafe {
            // SAFETY: the memory is unsused and has enough space.
            node_ptr.write(node);
            self.head.next = Some(&mut *node_ptr);
        }
    }

    fn find_region(&mut self, size: usize, align: usize) -> Option<(&'static mut ListNode, usize)> {
        let mut current = &mut self.head;

        while let Some(ref mut region) = current.next {
            if let Ok(alloc_start) = Self::alloc_from_region(region, size, align) {
                let next = region.next.take();
                let ret = Some((current.next.take().unwrap(), alloc_start));
                current.next = next;
                return ret;
            }

            current = current.next.as_mut().unwrap();
        }

        None
    }

    fn alloc_from_region(region: &ListNode, size: usize, align: usize) -> Result<usize, ()> {
        let alloc_start = fast_align_up(region.get_start_addr(), align);

        let Some(alloc_end) = alloc_start.checked_add(size) else {
            return Err(());
        };

        if alloc_end > region.get_end_addr() {
            return Err(());
        }

        let excess_size = region.get_end_addr() - alloc_end;

        if excess_size > 0 && excess_size < mem::size_of::<ListNode>() {
            return Err(());
        }

        Ok(alloc_start)
    }

    /// Adjust the given layout so that the resulting allocated memory
    /// region is also capable of storing a `ListNode`.
    ///
    /// Returns the adjusted size and alignment as a (size, align) tuple.
    fn size_align(layout: Layout) -> (usize, usize) {
        let layout = layout
            .align_to(mem::align_of::<ListNode>())
            .expect("adjusting aligment failed")
            .pad_to_align();
        let size = layout.size().max(mem::size_of::<ListNode>());
        (size, layout.align())
    }
}

unsafe impl GlobalAlloc for Locked<LinkedListAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let (size, align) = LinkedListAllocator::size_align(layout);
        let mut allocator = self.lock();

        if let Some((region, alloc_start)) = allocator.find_region(size, align) {
            let alloc_end = alloc_start.checked_add(size).expect("overflow");
            let excess_size = region.get_end_addr() - alloc_end;
            if excess_size > 0 {
                unsafe {
                    // SAFETY: the memory is unsused and has enough space.
                    allocator.add_free_region(alloc_end, excess_size);
                }
            }
            alloc_start as *mut u8
        } else {
            ptr::null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let (size, _) = LinkedListAllocator::size_align(layout);

        unsafe {
            // SAFETY: the memory is unsused and has enough space.
            self.lock().add_free_region(ptr as usize, size);
        }
    }
}
