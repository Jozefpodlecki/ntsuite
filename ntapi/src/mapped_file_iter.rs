use crate::{ntdef::*, ntmmapi::*};

const MAX_MODULE_NAME_LEN: usize = 260;

pub struct MappedFile {
    base_address: usize,
    name: [u16; MAX_MODULE_NAME_LEN],
}

impl MappedFile {
    pub fn base_address(&self) -> usize {
        self.base_address
    }
    
    pub fn name_as_utf16(&self) -> &[u16] {
        let len = (0..MAX_MODULE_NAME_LEN)
            .find(|&i| self.name[i] == 0)
            .unwrap_or(0);
        &self.name[..len]
    }

    fn to_utf8(&self) -> [u8; MAX_MODULE_NAME_LEN * 2] {
        let utf16 = self.name_as_utf16();
        let mut utf8_buf = [0u8; MAX_MODULE_NAME_LEN * 2];
        let mut pos = 0;
        
        for &ch in utf16 {
            if ch == 0 { break; }
            if ch <= 0x7F {
                if pos + 1 > utf8_buf.len() { break; }
                utf8_buf[pos] = ch as u8;
                pos += 1;
            } else if ch <= 0x7FF {
                if pos + 2 > utf8_buf.len() { break; }
                utf8_buf[pos] = (0xC0 | ((ch >> 6) & 0x1F)) as u8;
                utf8_buf[pos + 1] = (0x80 | (ch & 0x3F)) as u8;
                pos += 2;
            } else {
                if pos + 3 > utf8_buf.len() { break; }
                utf8_buf[pos] = (0xE0 | ((ch >> 12) & 0x0F)) as u8;
                utf8_buf[pos + 1] = (0x80 | ((ch >> 6) & 0x3F)) as u8;
                utf8_buf[pos + 2] = (0x80 | (ch & 0x3F)) as u8;
                pos += 3;
            }
        }
        utf8_buf
    }
    
    pub fn name_eq_utf8(&self, utf8_bytes: &[u8]) -> bool {
        let utf8_buf = self.to_utf8();
        let len = utf8_buf.iter().position(|&b| b == 0).unwrap_or(utf8_buf.len());
        &utf8_buf[..len] == utf8_bytes
    }
    
    pub fn name_contains_utf8(&self, utf8_bytes: &[u8]) -> bool {
        let utf8_buf = self.to_utf8();
        let len = utf8_buf.iter().position(|&b| b == 0).unwrap_or(utf8_buf.len());
        utf8_buf[..len].windows(utf8_bytes.len()).any(|window| window == utf8_bytes)
    }
}

pub struct MappedFileIterator {
    handle: HANDLE,
    address: usize,
}

impl MappedFileIterator {
    pub fn new(handle: HANDLE) -> Self {
        Self {
            handle,
            address: 0,
        }
    }
    
    fn get_mapped_file_name(&self, base_address: PVOID) -> Result<[u16; MAX_MODULE_NAME_LEN], NTSTATUS> {
        unsafe {
            let mut buffer = [0u8; 1024];
            let mut bytes_read: SIZE_T = 0;
            
            let status = NtQueryVirtualMemory(
                self.handle,
                base_address,
                MEMORY_INFORMATION_CLASS::MemoryMappedFilenameInformation,
                buffer.as_mut_ptr() as PVOID,
                buffer.len() as SIZE_T,
                &mut bytes_read,
            );
            
            if status < 0 {
                return Err(status);
            }
            
            if bytes_read >= core::mem::size_of::<UNICODE_STRING>() as SIZE_T {
                let us = &*(buffer.as_ptr() as *const UNICODE_STRING);
                let len = (us.Length as usize) / 2;
                
                if len > 0 && len < MAX_MODULE_NAME_LEN {
                    let slice = core::slice::from_raw_parts(us.Buffer, len);
                    let mut name_buf = [0u16; MAX_MODULE_NAME_LEN];
                    
                    for (i, &ch) in slice.iter().enumerate() {
                        name_buf[i] = ch;
                    }
                    
                    return Ok(name_buf);
                }
            }
            
            Err(-1)
        }
    }
}

impl Iterator for MappedFileIterator {
    type Item = MappedFile;
    
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let mut mbi: MEMORY_BASIC_INFORMATION = core::mem::zeroed();
            let mut return_length: SIZE_T = 0;
            
            let status = NtQueryVirtualMemory(
                self.handle,
                self.address as PVOID,
                MEMORY_INFORMATION_CLASS::MemoryBasicInformation,
                &mut mbi as *mut _ as PVOID,
                core::mem::size_of::<MEMORY_BASIC_INFORMATION>() as SIZE_T,
                &mut return_length,
            );
            
            if status < 0 {
                return None;
            }
            
            let base = mbi.BaseAddress as usize;
            let size = mbi.RegionSize;
            
            if size == 0 {
                return None;
            }
            
            self.address = base.saturating_add(size);
            
            if mbi.Type == MEM_IMAGE as u32 {
                let module_name = match self.get_mapped_file_name(mbi.BaseAddress) {
                    Ok(name) => name,
                    Err(_) => return None,
                };
                
                return Some(MappedFile {
                    base_address: base,
                    name: module_name,
                });
            }
            
            self.next()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator_finds_ntdll() {
        let handle = usize::MAX as _;
        let mut found_ntdll = false;
        
        for module in MappedFileIterator::new(handle) {
            if module.name_contains_utf8(b"ntdll.dll") {
                found_ntdll = true;
                assert!(module.base_address() > 0);
                break;
            }
        }
        
        assert!(found_ntdll, "ntdll.dll not found in process memory");
    }
}