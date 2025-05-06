#![no_std]

use allocator::{BaseAllocator, ByteAllocator, PageAllocator, AllocResult, AllocError};
use core::alloc::Layout;
use core::ptr::NonNull;

/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///
pub struct EarlyAllocator<const PAGE_SIZE: usize> {
    start: usize,
    end: usize,
    b_pos: usize,
    p_pos: usize,
    b_count: usize,
}

impl<const PAGE_SIZE: usize> EarlyAllocator<PAGE_SIZE> {
    pub const fn new() -> Self {
        Self {
            start: 0,
            end: 0,
            b_pos: 0,
            p_pos: 0,
            b_count: 0,
        }
    }
}

impl<const PAGE_SIZE: usize> BaseAllocator for EarlyAllocator<PAGE_SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.end = start + size;
        self.b_pos = start;
        self.p_pos = start + size;
        self.b_count = 0;
    }

    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        if start + size > self.start && start < self.end {
            return Err(AllocError::MemoryOverlap);
        }
        if start + size != self.start {
            return Err(AllocError::InvalidParam);
        }
        self.start = start;
        Ok(())
    }

}

impl<const PAGE_SIZE: usize> ByteAllocator for EarlyAllocator<PAGE_SIZE> {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        let alloc_end = self.b_pos + layout.size();
        if alloc_end > self.p_pos {
            return Err(AllocError::NoMemory);
        }
        let ret = NonNull::new(self.b_pos as *mut u8).ok_or(AllocError::NoMemory)?;
        self.b_pos += layout.size();
        self.b_count += 1;
        Ok(ret)
    }

    fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        if self.b_count == 0 {
            return;
        }
        let pos = pos.as_ptr() as usize;
        if pos < self.start || pos >= self.end {
            return;
        }
        if pos + layout.size() == self.b_pos {
            self.b_pos = pos;
        }

        self.b_count -= 1;
        if self.b_count == 0 {
            self.b_pos = self.start;
        }
    }

    fn total_bytes(&self) -> usize {
        self.p_pos - self.start
    }

    fn used_bytes(&self) -> usize {
        self.b_pos - self.start
    }

    fn available_bytes(&self) -> usize {
        self.p_pos - self.b_pos
    }

}

impl<const PAGE_SIZE: usize> PageAllocator for EarlyAllocator<PAGE_SIZE> {
    const PAGE_SIZE: usize = PAGE_SIZE;
    fn alloc_pages(&mut self, num_pages: usize, align_pow2: usize) -> AllocResult<usize> {
        let alloc_end = self.p_pos - num_pages * Self::PAGE_SIZE;
        if alloc_end < self.b_pos {
            return Err(AllocError::NoMemory);
        }
        let ret = alloc_end;
        self.p_pos -= num_pages * Self::PAGE_SIZE;
        Ok(ret)
    }

    fn dealloc_pages(&mut self, pos: usize, num_pages: usize) {
        if pos < self.start || pos >= self.end {
            return;
        }
        if pos == self.p_pos && pos + num_pages * Self::PAGE_SIZE <= self.end {
            self.p_pos += num_pages * Self::PAGE_SIZE;
        }
    }

    fn total_pages(&self) -> usize {
        (self.end - self.b_pos) / Self::PAGE_SIZE
    }

    fn available_pages(&self) -> usize {
        (self.p_pos - self.b_pos) / Self::PAGE_SIZE
    }

    fn used_pages(&self) -> usize {
        (self.end - self.p_pos) / Self::PAGE_SIZE
    }
}
