use core::alloc::{AllocError, Allocator, GlobalAlloc, Layout};
use core::ptr::{self, NonNull};
use core::ffi::c_void;

pub type PVOID = *mut c_void;
pub type ULONG = u32;
pub type SIZE_T = usize;
pub type PSIZE_T = *mut SIZE_T;
pub type BOOLEAN = u8;
pub type NTSTATUS = i32;

const MIN_ALIGN: usize = 16;
const HEAP_ZERO_MEMORY: ULONG = 0x00000008; 

#[unsafe(naked)]
pub unsafe fn get_process_heap() -> *mut c_void {
    core::arch::naked_asm!(
        "mov rax, gs:[0x60]",
        "mov rax, [rax + 0x30]",
        "ret"
    );
}

unsafe extern "system" {
    fn RtlAllocateHeap(HeapHandle: PVOID, Flags: ULONG, Size: SIZE_T) -> PVOID;
    fn RtlReAllocateHeap(HeapHandle: PVOID, Flags: ULONG, BaseAddress: PVOID, Size: SIZE_T) -> PVOID;
    fn RtlFreeHeap(HeapHandle: PVOID, Flags: ULONG, BaseAddress: PVOID) -> BOOLEAN;
}

#[repr(C)]
struct Header(*mut u8);

#[inline]
fn allocate(layout: Layout, zeroed: bool) -> *mut u8 {
    unsafe {
        if layout.size() == 0 {
            return ptr::null_mut();
        }
        
        let flags = if zeroed { HEAP_ZERO_MEMORY } else { 0 };
        let heap = get_process_heap();
        
        if heap.is_null() {
            return ptr::null_mut();
        }
        
        if layout.align() <= MIN_ALIGN {
            RtlAllocateHeap(heap, flags, layout.size()) as *mut u8
        } else {
            let total = layout.align() + layout.size();
            let ptr = RtlAllocateHeap(heap, flags, total) as *mut u8;
            
            if ptr.is_null() {
                return ptr::null_mut();
            }
            
            let offset = layout.align() - (ptr.addr() & (layout.align() - 1));
            let aligned = ptr.add(offset);
            (aligned as *mut Header).sub(1).write(Header(ptr));
            
            aligned
        }
    }
}

pub struct NtAlloc;

unsafe impl GlobalAlloc for NtAlloc {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { allocate(layout, false) }
    }
    
    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        unsafe { allocate(layout, true) }
    }
    
    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            if ptr.is_null() || layout.size() == 0 {
                return;
            }
            
            let heap = get_process_heap();
            if heap.is_null() {
                return;
            }
            
            let block = if layout.align() <= MIN_ALIGN {
                ptr
            } else {
                (ptr as *mut Header).sub(1).read().0
            };
            
            RtlFreeHeap(heap, 0, block as PVOID);
        }
    }
    
    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        unsafe {
            if new_size == 0 {
                self.dealloc(ptr, layout);
                return ptr::null_mut();
            }
            
            if ptr.is_null() {
                return self.alloc(Layout::from_size_align_unchecked(new_size, layout.align()));
            }
            
            let heap = get_process_heap();
            if heap.is_null() {
                return ptr::null_mut();
            }
            
            if layout.align() <= MIN_ALIGN {
                RtlReAllocateHeap(heap, 0, ptr as PVOID, new_size) as *mut u8
            } else {
                let new_layout = Layout::from_size_align_unchecked(new_size, layout.align());
                let new_ptr = self.alloc(new_layout);
                if !new_ptr.is_null() {
                    let copy_size = layout.size().min(new_size);
                    ptr::copy_nonoverlapping(ptr, new_ptr, copy_size);
                    self.dealloc(ptr, layout);
                }
                new_ptr
            }
        }
    }
}

unsafe impl Allocator for NtAlloc {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        unsafe {
            let ptr = self.alloc(layout);
            if ptr.is_null() {
                Err(AllocError)
            } else {
                Ok(NonNull::slice_from_raw_parts(NonNull::new_unchecked(ptr), layout.size()))
            }
        }
    }
    
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        unsafe { self.dealloc(ptr.as_ptr(), layout) }
    }
}
