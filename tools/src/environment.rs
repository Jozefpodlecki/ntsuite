use ntapi::{ntdef::PVOID, ntrtl::RtlCreateEnvironmentEx};

pub struct Environment(PVOID);

impl Environment {
    pub fn new(inherit_current: bool) -> Self {
        let flags = if inherit_current { 0x4 } else { 0x0 };
        let source = core::ptr::null_mut();
        let mut env_ptr: PVOID = core::ptr::null_mut();
        let status = unsafe { RtlCreateEnvironmentEx(source, &mut env_ptr, flags) };

        if status < 0 {
            panic!("RtlCreateEnvironmentEx failed: 0x{:08X}", status);
        }

        Self(env_ptr)
    }

    pub fn size(&self) -> usize {
        if self.0.is_null() {
            return 0;
        }
        
        let mut size = 0;
        let ptr = self.0 as *mut u16;
        
        unsafe {
            while *ptr.add(size / 2) != 0 || *ptr.add(size / 2 + 1) != 0 {
                size += 2;
            }
            size += 4;
        }
        
        size
    }

    pub fn as_ptr(&self) -> PVOID {
        self.0
    }

    pub fn as_mut_ptr(&mut self) -> PVOID {
        self.0
    }
}
