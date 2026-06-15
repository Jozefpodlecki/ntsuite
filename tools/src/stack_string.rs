pub struct U16CStackString<const N: usize> {
    buf: [u16; N],
    len: usize,
}

impl<const N: usize> U16CStackString<N> {
    pub fn from_str(value: &str) -> Option<Self> {
        let mut buf = [0u16; N];
        let mut len = 0;
        
        for ch in value.encode_utf16() {
            if len + 1 >= N {
                return None;
            }
            buf[len] = ch;
            len += 1;
        }
        
        if len + 1 > N {
            return None;
        }
        buf[len] = 0;
        
        Some(Self { buf, len })
    }

    pub fn push_str(&mut self, value: &str) -> bool {
        for ch in value.encode_utf16() {
            if self.len + 1 >= N {
                return false;
            }
            self.buf[self.len] = ch;
            self.len += 1;
        }
        
        if self.len + 1 > N {
            return false;
        }
        self.buf[self.len] = 0;
        
        true
    }
    
    pub fn clear(&mut self) {
        self.len = 0;
        self.buf[0] = 0;
    }
    
    pub fn as_ptr(&self) -> *const u16 {
        self.buf.as_ptr()
    }
    
    pub fn as_mut_ptr(&mut self) -> *mut u16 {
        self.buf.as_mut_ptr()
    }
    
    pub fn len(&self) -> usize {
        self.len
    }
    
    pub fn as_slice(&self) -> &[u16] {
        &self.buf[..self.len]
    }

    pub fn leak(&mut self) -> *mut u16 {
        self.buf.as_mut_ptr()
    }
}