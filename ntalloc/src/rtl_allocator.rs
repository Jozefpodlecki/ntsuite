use core::alloc::{AllocError, Allocator, GlobalAlloc, Layout};
use core::ops::Add;
use core::ptr::{self, NonNull};
use core::ffi::c_void;

use ntapi::{ntdef::*};
use tools::MappedFileIterator;

pub type PVOID = *mut c_void;
pub type ULONG = u32;
pub type SIZE_T = usize;
pub type PSIZE_T = *mut SIZE_T;
pub type BOOLEAN = u8;
pub type NTSTATUS = i32;

const MIN_ALIGN: usize = 16;
const HEAP_ZERO_MEMORY: ULONG = 0x00000008;
const HEAP_GENERATE_EXCEPTIONS: ULONG = 0x00000004;
const HEAP_SIGNATURE: ULONG = 0xDDEEDDEE;

#[unsafe(naked)]
pub unsafe fn get_process_heap() -> *mut c_void {
    core::arch::naked_asm!(
        "mov rax, gs:[0x60]",
        "mov rax, [rax + 0x30]",
        "ret"
    );
}

#[unsafe(naked)]
pub unsafe fn get_thread_heap_context() -> *mut c_void {
    core::arch::naked_asm!(
        "mov rax, gs:[0x1858]",
        "ret"
    );
}

#[unsafe(naked)]
pub unsafe fn set_thread_heap_context(ctx: *mut c_void) {
    core::arch::naked_asm!(
        "mov gs:[0x1858], rcx",
        "ret"
    );
}

#[repr(C)]
struct Header(*mut u8);


type RtlpHpAllocateHeapSlowFn = unsafe extern "system" fn(PVOID, SIZE_T, ULONG) -> PVOID;
type RtlpHpVsContextAllocateFn = unsafe extern "system" fn(PVOID, SIZE_T, ULONG) -> PVOID;
type RtlpHpLargeAllocFn = unsafe extern "system" fn(PVOID, SIZE_T, ULONG) -> PVOID;
type RtlpHpFreeHeapFn = unsafe extern "system" fn(PVOID, PVOID) -> BOOLEAN;
type RtlpHpTagReAllocateHeapFn = unsafe extern "system" fn(PVOID, PVOID, SIZE_T, ULONG) -> PVOID;
type RtlpReAllocateHeapInternalFn = unsafe extern "system" fn(PVOID, PVOID, SIZE_T, ULONG) -> PVOID;
type RtlpAllocateHeapRaiseExceptionFn = unsafe extern "system" fn(SIZE_T);

static mut FN_RTL_HP_ALLOCATE_HEAP_SLOW: Option<RtlpHpAllocateHeapSlowFn> = None;
static mut FN_RTL_HP_VS_CONTEXT_ALLOCATE: Option<RtlpHpVsContextAllocateFn> = None;
static mut FN_RTL_HP_LARGE_ALLOC: Option<RtlpHpLargeAllocFn> = None;
static mut FN_RTL_HP_FREE_HEAP: Option<RtlpHpFreeHeapFn> = None;
static mut FN_RTL_HP_TAG_REALLOCATE_HEAP: Option<RtlpHpTagReAllocateHeapFn> = None;
static mut FN_RTLP_REALLOCATE_HEAP_INTERNAL: Option<RtlpReAllocateHeapInternalFn> = None;
static mut FN_RTLP_ALLOCATE_HEAP_RAISE_EXCEPTION: Option<RtlpAllocateHeapRaiseExceptionFn> = None;

#[repr(C)]
pub struct HEAP_SEGMENT {
    pub Entry: HEAP_ENTRY,
    pub SegmentSignature: ULONG,
    pub SegmentFlags: ULONG,
    pub SegmentListEntry: LIST_ENTRY,
    pub Heap: PVOID,
    pub BaseAddress: PVOID,
    pub NumberOfPages: ULONG,
    pub FirstEntry: *mut HEAP_ENTRY,
    pub LastValidEntry: *mut HEAP_ENTRY,
    pub NumberOfUnCommittedPages: ULONG,
    pub NumberOfUnCommittedRanges: ULONG,
    pub SegmentAllocatorBackTraceIndex: USHORT,
    pub Reserved: USHORT,
    pub UCRSegmentList: LIST_ENTRY,
}

#[repr(C)]
pub struct HEAP_ENTRY {
    pub PreviousSize: USHORT,
    pub Size: USHORT,
    pub Flags: UCHAR,
    pub SmallTagIndex: UCHAR,
    pub SegmentIndex: USHORT,
    pub UnusedBytes: USHORT,
    pub FunctionIndex: USHORT,
    pub Context: PVOID,
    pub Interceptor: ULONG,
}

pub type PHEAP_SEGMENT = *mut HEAP_SEGMENT;
pub type PHEAP_ENTRY = *mut HEAP_ENTRY;

#[inline]
pub fn is_valid_heap(heap: PVOID) -> bool {
    unsafe {
        let segment = heap as *const HEAP_SEGMENT;
        (*segment).SegmentSignature == HEAP_SIGNATURE
    }
}

pub fn rtl_allocate_heap(heap_handle: PVOID, flags: ULONG, size: SIZE_T) -> PVOID {
    unsafe {
        let heap = heap_handle;
        let mut result: PVOID = core::ptr::null_mut();
        let mut heap_flags_combined: ULONG = 0;
        
        if !is_valid_heap(heap) {
            return core::ptr::null_mut();
        }

        let thread_heap_context = (heap as *mut u8).add(0x14) as PVOID;
        set_thread_heap_context(thread_heap_context);

        let context_flags = *(thread_heap_context as *const ULONG);
        let combined_flags = context_flags | flags;
        let masked_flags = combined_flags & 0x2FFA;
        
          if masked_flags & !0xFFFFFFFD != 0 {

            return FN_RTL_HP_ALLOCATE_HEAP_SLOW.unwrap()(heap, size, masked_flags);
        }
        
        let aligned_size = if size == 0 { 1 } else { size };
        
        if aligned_size > 0x20000 {
            result = FN_RTL_HP_LARGE_ALLOC.unwrap()(heap, size, flags);
            return result;
        }
        panic!("test");
        result = FN_RTL_HP_VS_CONTEXT_ALLOCATE.unwrap()(heap, size, flags);
        if !result.is_null() {
            return result;
        }
        
        if result.is_null() && (flags & HEAP_GENERATE_EXCEPTIONS != 0) {
            FN_RTLP_ALLOCATE_HEAP_RAISE_EXCEPTION.unwrap()(size);
        }
        
        result
    }
}

pub fn rtl_free_heap(heap_handle: PVOID, _flags: ULONG, base_address: PVOID) -> bool {
    unsafe {
        let heap = heap_handle;
        let ptr = base_address;
        
        if heap.is_null() || ptr.is_null() {
            return false;
        }
        
        if is_valid_heap(heap) {
            FN_RTL_HP_FREE_HEAP.unwrap()(heap, ptr) != 0
        } else {
            false
        }
    }
}

pub fn rtl_re_allocate_heap(
    heap_handle: PVOID,
    _flags: ULONG,
    base_address: PVOID,
    new_size: SIZE_T,
) -> PVOID {
    unsafe {
        let heap = heap_handle;
        let old_ptr = base_address;
        
        if heap.is_null() || old_ptr.is_null() {
            return core::ptr::null_mut();
        }
        
        if is_valid_heap(heap) {
            FN_RTL_HP_TAG_REALLOCATE_HEAP.unwrap()(heap, old_ptr, new_size, 0)
        } else {
            core::ptr::null_mut()
        }
    }
}

#[inline]
unsafe fn allocate(layout: Layout, zeroed: bool) -> *mut u8 {
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
            rtl_allocate_heap(heap, flags, layout.size()) as *mut u8
        } else {
            let total = layout.align() + layout.size();
            let ptr = rtl_allocate_heap(heap, flags, total) as *mut u8;
            
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

pub struct RtlAlloc;

impl RtlAlloc {
    pub fn init() {
        unsafe {
            let ntdll_base = Self::get_ntdll_base();
                FN_RTL_HP_ALLOCATE_HEAP_SLOW = Some(core::mem::transmute(ntdll_base.add(0x59710)));
                FN_RTL_HP_VS_CONTEXT_ALLOCATE = Some(core::mem::transmute(ntdll_base.add(0x583FC)));
                FN_RTL_HP_LARGE_ALLOC = Some(core::mem::transmute(ntdll_base.add(0xD8104)));
                FN_RTL_HP_FREE_HEAP = Some(core::mem::transmute(ntdll_base.add(0x78BA0)));
                FN_RTL_HP_TAG_REALLOCATE_HEAP = Some(core::mem::transmute(ntdll_base.add(0x54590)));
                FN_RTLP_REALLOCATE_HEAP_INTERNAL = Some(core::mem::transmute(ntdll_base.add(0x546C0)));
                FN_RTLP_ALLOCATE_HEAP_RAISE_EXCEPTION = Some(core::mem::transmute(ntdll_base.add(0x115A58)));
        }
    }

    fn get_ntdll_base() -> usize {
        let handle = usize::MAX as _;
        
        for module in MappedFileIterator::new(handle) {
            if module.name_contains_utf8(b"ntdll.dll") {
                return module.base_address();
            }
        }
        
        panic!("Could not find ntdll base?")
    }
}

unsafe impl GlobalAlloc for RtlAlloc {
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
            
            rtl_free_heap(heap, 0, block as PVOID);
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
                rtl_re_allocate_heap(heap, 0, ptr as PVOID, new_size) as *mut u8
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

unsafe impl Allocator for RtlAlloc {
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