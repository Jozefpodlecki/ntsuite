use std::path::Path;
use core::mem;
use ntapi::{ntdef::*, ntpsapi::*, ntrtl::*};
use tools::*;

pub fn create_suspended<R: AsRef<Path>>(path: R, args: &[&str]) -> () {

    unsafe {
        let path = path.as_ref();
        let mut env = Environment::new(false);

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
        // let current_dir = U16CStackString::<260>::from_str(parent_dir).unwrap();
        let current_dir = widestring::U16CString::from_str(parent_dir).unwrap();
        process_parameters.CurrentDirectory.DosPath.Buffer = current_dir.as_ptr() as _;
        process_parameters.CurrentDirectory.DosPath.Length = (current_dir.len() * 2) as u16;
        process_parameters.CurrentDirectory.DosPath.MaximumLength = (current_dir.len() * 2 + 2) as u16;
        process_parameters.CurrentDirectory.Handle = core::ptr::null_mut();

        let exe_path = path.to_str().unwrap();
        // let image_path = U16CStackString::<260>::from_str(exe_path).unwrap();
        let image_path = widestring::U16CString::from_str(exe_path).unwrap();
        process_parameters.ImagePathName.Buffer = image_path.as_ptr() as _;
        process_parameters.ImagePathName.Length = (image_path.len() * 2) as u16;
        process_parameters.ImagePathName.MaximumLength = (image_path.len() * 2 + 2) as u16;

        // let mut cmd_line_wide = U16CStackString::<260>::from_str(&exe_path).unwrap();
        // cmd_line_wide.push_str(" ");
        // cmd_line_wide.push_str(&args.join(" "));
        let cmd_line = format!("{} {}", exe_path, args.join(" "));
        let cmd_line_wide = widestring::U16CString::from_str(&cmd_line).unwrap();
        process_parameters.CommandLine.Buffer = cmd_line_wide.as_ptr() as _;
        process_parameters.CommandLine.Length = (cmd_line_wide.len() * 2) as u16;
        process_parameters.CommandLine.MaximumLength = (cmd_line_wide.len() * 2 + 2) as u16;

        process_parameters.Environment = env.as_mut_ptr();
        process_parameters.EnvironmentSize = env.size() as _;
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

        // let window_title = U16CStackString::<260>::from_str(exe_path).unwrap();
        let window_title = widestring::U16CString::from_str(exe_path).unwrap();
        process_parameters.WindowTitle.Buffer = window_title.as_ptr() as _;
        process_parameters.WindowTitle.Length = (window_title.len() * 2) as u16;
        process_parameters.WindowTitle.MaximumLength = (window_title.len() * 2 + 2) as u16;

        // let desktop_info = U16CStackString::<260>::from_str("Winsta0\\Default").unwrap();
        let desktop_info = widestring::U16CString::from_str("Winsta0\\Default").unwrap();
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
            .image_name(r#"\??\C:\repos\reverse_projects\rust_playground\empty_x64_exe_no_std\target\release\empty_x64_exe_no_std.exe"#)
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
                InitState: init_state,
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

fn main() {
    create_suspended(r#"C:\repos\reverse_projects\rust_playground\empty_x64_exe_no_std\target\release\empty_x64_exe_no_std.exe"#, &[]);
    // create_suspended(r#"C:\Windows\notepad.exe"#, &[]);
    // NtCreateUserProcess failed: 0xC000003B
    // NtCreateUserProcess failed: 0xC0000039
}