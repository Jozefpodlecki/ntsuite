pub struct AStackString<const N: usize> {
    buf: [u8; N],
    len: usize,
}

impl<const N: usize> AStackString<N> {
    pub fn from_ptr(ptr: *const u8) -> Option<Self> {
        if ptr.is_null() {
            return None;
        }
        
        let mut buf = [0u8; N];
        let mut len = 0;
        
        unsafe {
            let mut i = 0;
            while len < N - 1 {
                let ch = *ptr.add(i);
                if ch == 0 {
                    break;
                }
                buf[len] = ch;
                len += 1;
                i += 1;
            }
        }
        
        buf[len] = 0;
        
        Some(Self { buf, len })
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.buf.as_ptr()
    }
    
    pub fn as_slice(&self) -> &[u8] {
        &self.buf[..self.len]
    }

    pub fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(self.as_slice()) }
    }
}