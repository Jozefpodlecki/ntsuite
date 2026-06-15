use core::mem;

use ntapi::{ntdef::ULONG_PTR, ntmmapi::SECTION_IMAGE_INFORMATION, ntpsapi::*, ntrtl::CLIENT_ID};

use crate::U16CStackString;

pub const PS_ATTRIBUTE_CHPE: ULONG_PTR = 0x0006001A;

pub enum PsAttribute {
    ImageName(Box<str>),
    ImageInfo,
    ClientId,
    StdHandleInfo {
        inherit_all: bool,
        subsystem_type: u32
    },
    Chpe(bool),
}

impl PsAttribute {
    pub fn to_ps_attribute(self) -> PS_ATTRIBUTE {
        let mut attr: PS_ATTRIBUTE = unsafe { core::mem::zeroed() };
        
        match self {
            PsAttribute::ImageName(data) => {
                attr.Attribute = PS_ATTRIBUTE_IMAGE_NAME;
                attr.Size = data.len() as u64 * 2;
                let mut wide = U16CStackString::<260>::from_str(&data).unwrap();
                attr.u.ValuePtr = Box::leak(Box::new(wide)).as_mut_ptr() as *mut _;
            }
            PsAttribute::ImageInfo => {
                attr.Attribute = PS_ATTRIBUTE_IMAGE_INFO;
                attr.Size = 0x40;
                let value = unsafe { Box::new(mem::zeroed::<SECTION_IMAGE_INFORMATION>()) };
                attr.u.ValuePtr = Box::into_raw(value) as *mut _;
            }
            PsAttribute::ClientId => {
                attr.Attribute = PS_ATTRIBUTE_CLIENT_ID;
                attr.Size = core::mem::size_of::<CLIENT_ID>() as u64;
                let value = unsafe { Box::new(mem::zeroed::<CLIENT_ID>()) };
                attr.u.ValuePtr = Box::into_raw(value) as *mut _;
            }
            PsAttribute::StdHandleInfo { inherit_all, subsystem_type} => {
                attr.Attribute = PS_ATTRIBUTE_STD_HANDLE_INFO;
                attr.Size = core::mem::size_of::<PS_STD_HANDLE_INFO>() as u64;
                let mut value = unsafe { Box::new(mem::zeroed::<PS_STD_HANDLE_INFO>()) };
                if inherit_all {
                    value.Flags |= 0x1;
                }
                value.StdHandleSubsystemType= subsystem_type;
                attr.u.ValuePtr = Box::into_raw(value) as *mut _;
            }
            PsAttribute::Chpe(val) => {
                attr.Attribute = PS_ATTRIBUTE_CHPE;
                attr.Size = 1;
                attr.u.Value = val as _;
            }
        }
        
        attr
    }
}


pub struct AttributeListBuilder(Vec<PsAttribute>);

impl AttributeListBuilder {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn image_name(mut self, path: &str) -> Self {
        self.0.push(PsAttribute::ImageName(path.into()));
        self
    }

    pub fn client_id(mut self) -> Self {
        self.0.push(PsAttribute::ClientId);
        self
    }

    pub fn image_info(mut self) -> Self {
        self.0.push(PsAttribute::ImageInfo);
        self
    }

    pub fn std_handle_info(mut self, inherit_all: bool, subsystem_type: u32) -> Self {
        self.0.push(PsAttribute::StdHandleInfo{
            inherit_all,
            subsystem_type
        });
        self
    }

    pub fn chpe(mut self, value: bool) -> Self {
        self.0.push(PsAttribute::Chpe(value));
        self
    }

    pub fn build(self) -> *mut PS_ATTRIBUTE_LIST {
        let attr_count = self.0.len();
        let total_length = core::mem::size_of::<usize>() + (attr_count * core::mem::size_of::<PS_ATTRIBUTE>());
        
        let mut buffer = vec![0u8; total_length];
        let ptr = buffer.as_mut_ptr() as *mut PS_ATTRIBUTE_LIST;

        unsafe {
            (*ptr).TotalLength = total_length as u64;
            // let attrs_ptr = (*ptr).Attributes as *mut PS_ATTRIBUTE;
            let attrs_ptr = core::ptr::addr_of_mut!((*ptr).Attributes) as *mut PS_ATTRIBUTE;

            for (i, attr) in self.0.into_iter().enumerate() {
                let ps_attr = attr.to_ps_attribute();
                core::ptr::write(attrs_ptr.add(i), ps_attr);
            }
        }

        core::mem::forget(buffer);
        ptr
    }
}