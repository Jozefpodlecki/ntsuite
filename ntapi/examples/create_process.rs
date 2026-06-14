use std::path::Path;
use core::mem;
use ntapi::{ntdef::*, ntpsapi::*, ntrtl::*};

pub const PS_ATTRIBUTE_CHPE: ULONG_PTR = 0x0006001A;

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
         println!("PS_ATTRIBUTE_LIST");
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

pub fn create_suspended<R: AsRef<Path>>(path: R, args: &[&str]) -> () {

    unsafe {
        let path = path.as_ref();
        let mut env_ptr: PVOID = core::ptr::null_mut();
        let flags = if false { 0x4 } else { 0x0 };
        let status = RtlCreateEnvironmentEx(core::ptr::null_mut(), &mut env_ptr, flags);

        if status < 0 {
            panic!("RtlCreateEnvironmentEx failed: 0x{:X}", status);
        }

        let mut process_parameters: RTL_USER_PROCESS_PARAMETERS = core::mem::zeroed();
        process_parameters.MaximumLength = 0x708;
        process_parameters.Length = 0x708;
        process_parameters.Flags = 0x1;
        process_parameters.DebugFlags = 0x0;
        process_parameters.ConsoleHandle = 0x4 as HANDLE;
        process_parameters.ConsoleFlags = 0x0;
        process_parameters.StandardInput = core::ptr::null_mut();
        process_parameters.StandardOutput = core::ptr::null_mut();
        process_parameters.StandardError = core::ptr::null_mut();

        let parent_dir = path.parent().unwrap().to_str().unwrap();
        let current_dir = U16CStackString::<260>::from_str(parent_dir).unwrap();
        process_parameters.CurrentDirectory.DosPath.Buffer = current_dir.as_ptr() as _;
        process_parameters.CurrentDirectory.DosPath.Length = (current_dir.len() * 2) as u16;
        process_parameters.CurrentDirectory.DosPath.MaximumLength = (current_dir.len() * 2 + 2) as u16;
        process_parameters.CurrentDirectory.Handle = core::ptr::null_mut();

        let exe_path = path.to_str().unwrap();
        let image_path = U16CStackString::<260>::from_str(exe_path).unwrap();
        process_parameters.ImagePathName.Buffer = image_path.as_ptr() as _;
        process_parameters.ImagePathName.Length = (image_path.len() * 2) as u16;
        process_parameters.ImagePathName.MaximumLength = (image_path.len() * 2 + 2) as u16;

        let cmd_line = format!("{} {}", exe_path, args.join(" "));
        let cmd_line_wide = U16CStackString::<260>::from_str(&cmd_line).unwrap();
        process_parameters.CommandLine.Buffer = cmd_line_wide.as_ptr() as _;
        process_parameters.CommandLine.Length = (cmd_line_wide.len() * 2) as u16;
        process_parameters.CommandLine.MaximumLength = (cmd_line_wide.len() * 2 + 2) as u16;

        process_parameters.Environment = env_ptr;
        process_parameters.EnvironmentSize = calculate_environment_size(env_ptr) as u64;
        process_parameters.EnvironmentVersion = 0x0;

        process_parameters.StartingX = 0;
        process_parameters.StartingY = 0;
        process_parameters.CountX = 0;
        process_parameters.CountY = 0;
        process_parameters.CountCharsX = 0;
        process_parameters.CountCharsY = 0;
        process_parameters.FillAttribute = 0;
        process_parameters.WindowFlags = 0;
        process_parameters.ShowWindowFlags = 0;

        let window_title = U16CStackString::<260>::from_str(exe_path).unwrap();
        process_parameters.WindowTitle.Buffer = window_title.as_ptr() as _;
        process_parameters.WindowTitle.Length = (window_title.len() * 2) as u16;
        process_parameters.WindowTitle.MaximumLength = (window_title.len() * 2 + 2) as u16;

        let desktop_info = U16CStackString::<260>::from_str("Winsta0\\Default").unwrap();
        process_parameters.DesktopInfo.Buffer = desktop_info.as_ptr() as _;
        process_parameters.DesktopInfo.Length = (desktop_info.len() * 2) as u16;
        process_parameters.DesktopInfo.MaximumLength = (desktop_info.len() * 2 + 2) as u16;

        process_parameters.ShellInfo.Buffer = core::ptr::null_mut();
        process_parameters.ShellInfo.Length = 0;
        process_parameters.ShellInfo.MaximumLength = 0;

        process_parameters.RuntimeData.Buffer = core::ptr::null_mut();
        process_parameters.RuntimeData.Length = 0;
        process_parameters.RuntimeData.MaximumLength = 0;

        process_parameters.PackageDependencyData = core::ptr::null_mut();
        process_parameters.ProcessGroupId = 0x378;
        process_parameters.LoaderThreads = 0;
 
        let mut attribute_list = AttributeListBuilder::new()
            .image_name(r#"\??\C:\Windows\notepad.exe"#)
            .image_info()
            .client_id()
            .std_handle_info(true, 0x3)
            .chpe(true)
            .build();

        let mut init_state: PS_CREATE_INFO_U_INIT_STATE = core::mem::zeroed();
        init_state.InitFlags = 0x20000003;
        init_state.AdditionalFileAccess = 0x81;

        let mut create_info = PS_CREATE_INFO {
            Size: core::mem::size_of::<PS_CREATE_INFO>() as u64,
            State: 0x0,
            u: PS_CREATE_INFO_U {
                InitState: core::mem::ManuallyDrop::new(init_state),
            },
        };
                    
        let mut process_handle: HANDLE = HANDLE::default();
        let mut thread_handle: HANDLE = HANDLE::default();

        let status = NtCreateUserProcess(
            &mut process_handle as *mut _ as *mut _,
            &mut thread_handle as *mut _ as *mut _,
            PROCESS_CREATE_FLAGS_SUSPENDED,
            PROCESS_CREATE_FLAGS_SUSPENDED,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            0,
            0x1,
            &mut process_parameters as *mut _ as *mut _,
            &mut create_info,
            attribute_list,
        );

        if status < 0 {
            panic!("NtCreateUserProcess failed: 0x{:X}", status);
        }
        
    }
}


fn calculate_environment_size(env_ptr: PVOID) -> usize {
    if env_ptr.is_null() {
        return 0;
    }
    
    let mut size = 0;
    let ptr = env_ptr as *mut u16;
    
    unsafe {
        while *ptr.add(size / 2) != 0 || *ptr.add(size / 2 + 1) != 0 {
            size += 2;
        }
        size += 4;
    }
    
    size
}


fn main() {
    create_suspended(r#"C:\Windows\notepad.exe"#, &[]);
    // NtCreateUserProcess failed: 0xC000003B
    // NtCreateUserProcess failed: 0xC0000039
}