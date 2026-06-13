
use core::ffi::{c_void, CStr};

use crate::ntdef::*;



#[repr(C)]
#[derive(Debug)]
pub struct OBJECT_ATTRIBUTES {
    pub Length: ULONG,
    pub RootDirectory: HANDLE,
    pub ObjectName: *mut UNICODE_STRING,
    pub Attributes: ULONG,
    pub SecurityDescriptor: *mut c_void,
    pub SecurityQualityOfService: *mut c_void,
}

pub const OBJECT_TYPE_CREATE: ACCESS_MASK = 0x0001;
pub const OBJECT_TYPE_ALL_ACCESS: ACCESS_MASK = STANDARD_RIGHTS_REQUIRED | OBJECT_TYPE_CREATE;

pub const DIRECTORY_QUERY: ACCESS_MASK = 0x0001;
pub const DIRECTORY_TRAVERSE: ACCESS_MASK = 0x0002;
pub const DIRECTORY_CREATE_OBJECT: ACCESS_MASK = 0x0004;
pub const DIRECTORY_CREATE_SUBDIRECTORY: ACCESS_MASK = 0x0008;
pub const DIRECTORY_ALL_ACCESS: ACCESS_MASK = STANDARD_RIGHTS_REQUIRED | DIRECTORY_QUERY | DIRECTORY_TRAVERSE | DIRECTORY_CREATE_OBJECT | DIRECTORY_CREATE_SUBDIRECTORY;

pub const SYMBOLIC_LINK_QUERY: ACCESS_MASK = 0x0001;
pub const SYMBOLIC_LINK_SET: ACCESS_MASK = 0x0002;
pub const SYMBOLIC_LINK_ALL_ACCESS: ACCESS_MASK = STANDARD_RIGHTS_REQUIRED | SYMBOLIC_LINK_QUERY;
pub const SYMBOLIC_LINK_ALL_ACCESS_EX: ACCESS_MASK = STANDARD_RIGHTS_REQUIRED | SPECIFIC_RIGHTS_ALL;

pub const STANDARD_RIGHTS_REQUIRED: ACCESS_MASK = 0x000F0000;
pub const STANDARD_RIGHTS_ALL: ACCESS_MASK = 0x001F0000;
pub const SPECIFIC_RIGHTS_ALL: ACCESS_MASK = 0x0000FFFF;

pub const OBJ_PROTECT_CLOSE: ULONG = 0x00000001;
pub const OBJ_INHERIT: ULONG = 0x00000002;
pub const OBJ_AUDIT_OBJECT_CLOSE: ULONG = 0x00000004;

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum OBJECT_INFORMATION_CLASS {
    ObjectBasicInformation = 0,
    ObjectNameInformation = 1,
    ObjectTypeInformation = 2,
    ObjectTypesInformation = 3,
    ObjectHandleFlagInformation = 4,
    ObjectSessionInformation = 5,
    ObjectSessionObjectInformation = 6,
    ObjectSetRefTraceInformation = 7,
    MaxObjectInfoClass = 8,
}

#[repr(C)]
pub struct OBJECT_BASIC_INFORMATION {
    pub Attributes: ULONG,
    pub GrantedAccess: ACCESS_MASK,
    pub HandleCount: ULONG,
    pub PointerCount: ULONG,
    pub PagedPoolCharge: ULONG,
    pub NonPagedPoolCharge: ULONG,
    pub Reserved: [ULONG; 3],
    pub NameInfoSize: ULONG,
    pub TypeInfoSize: ULONG,
    pub SecurityDescriptorSize: ULONG,
    pub CreationTime: LARGE_INTEGER,
}

#[repr(C)]
pub struct OBJECT_NAME_INFORMATION {
    pub Name: UNICODE_STRING,
}

#[repr(C)]
pub struct OBJECT_TYPE_INFORMATION {
    pub TypeName: UNICODE_STRING,
    pub TotalNumberOfObjects: ULONG,
    pub TotalNumberOfHandles: ULONG,
    pub TotalPagedPoolUsage: ULONG,
    pub TotalNonPagedPoolUsage: ULONG,
    pub TotalNamePoolUsage: ULONG,
    pub TotalHandleTableUsage: ULONG,
    pub HighWaterNumberOfObjects: ULONG,
    pub HighWaterNumberOfHandles: ULONG,
    pub HighWaterPagedPoolUsage: ULONG,
    pub HighWaterNonPagedPoolUsage: ULONG,
    pub HighWaterNamePoolUsage: ULONG,
    pub HighWaterHandleTableUsage: ULONG,
    pub InvalidAttributes: ULONG,
    pub GenericMapping: GENERIC_MAPPING,
    pub ValidAccessMask: ACCESS_MASK,
    pub SecurityRequired: BOOLEAN,
    pub MaintainHandleCount: BOOLEAN,
    pub TypeIndex: UCHAR,
    pub ReservedByte: CHAR,
    pub PoolType: ULONG,
    pub DefaultPagedPoolCharge: ULONG,
    pub DefaultNonPagedPoolCharge: ULONG,
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct GENERIC_MAPPING {
    pub GenericRead: ACCESS_MASK,
    pub GenericWrite: ACCESS_MASK,
    pub GenericExecute: ACCESS_MASK,
    pub GenericAll: ACCESS_MASK,
}

#[repr(C)]
#[derive(Debug)]
pub struct OBJECT_TYPES_INFORMATION {
    pub NumberOfTypes: ULONG,
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct OBJECT_HANDLE_FLAG_INFORMATION {
    pub Inherit: BOOLEAN,
    pub ProtectFromClose: BOOLEAN,
}

pub const DUPLICATE_CLOSE_SOURCE: ULONG = 0x00000001;
pub const DUPLICATE_SAME_ACCESS: ULONG = 0x00000002;
pub const DUPLICATE_SAME_ATTRIBUTES: ULONG = 0x00000004;

pub type WAIT_TYPE = u32;
pub const WaitAll: WAIT_TYPE = 0;
pub const WaitAny: WAIT_TYPE = 1;

#[repr(C)]
pub struct OBJECT_DIRECTORY_INFORMATION {
    pub Name: UNICODE_STRING,
    pub TypeName: UNICODE_STRING,
}

pub type BOUNDARY_ENTRY_TYPE = u32;
pub const BOUNDARY_ENTRY_TYPE_INVALID: BOUNDARY_ENTRY_TYPE = 0;
pub const BOUNDARY_ENTRY_TYPE_NAME: BOUNDARY_ENTRY_TYPE = 1;
pub const BOUNDARY_ENTRY_TYPE_SID: BOUNDARY_ENTRY_TYPE = 2;
pub const BOUNDARY_ENTRY_TYPE_IL: BOUNDARY_ENTRY_TYPE = 3;

#[repr(C)]
pub union OBJECT_BOUNDARY_VALUE {
    pub Name: [u16; 1],
    pub Sid: PSID,
    pub IntegrityLabel: PSID,
}

#[repr(C)]
#[derive(Debug)]
pub struct OBJECT_BOUNDARY_ENTRY {
    pub Type: BOUNDARY_ENTRY_TYPE,
    pub Size: ULONG,
    // Value: OBJECT_BOUNDARY_VALUE follows
}

pub const OBJECT_BOUNDARY_DESCRIPTOR_VERSION: ULONG = 1;

#[repr(C)]
#[derive(Debug)]
pub struct OBJECT_BOUNDARY_DESCRIPTOR {
    pub Version: ULONG,
    pub Items: ULONG,
    pub TotalSize: ULONG,
    pub Flags: ULONG,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SYMBOLIC_LINK_INFO_CLASS {
    SymbolicLinkGlobalInformation = 1,
    SymbolicLinkAccessMask = 2,
    MaxnSymbolicLinkInfoClass = 3,
}

#[inline]
pub fn initialize_object_attributes(
    attributes: &mut OBJECT_ATTRIBUTES,
    name: *mut UNICODE_STRING,
    root_directory: HANDLE,
) {
    attributes.Length = core::mem::size_of::<OBJECT_ATTRIBUTES>() as ULONG;
    attributes.RootDirectory = root_directory;
    attributes.ObjectName = name;
    attributes.Attributes = 0;
    attributes.SecurityDescriptor = core::ptr::null_mut();
    attributes.SecurityQualityOfService = core::ptr::null_mut();
}

unsafe extern "system" {
    pub fn NtQueryObject(
        Handle: HANDLE,
        ObjectInformationClass: OBJECT_INFORMATION_CLASS,
        ObjectInformation: *mut c_void,
        ObjectInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn NtSetInformationObject(
        Handle: HANDLE,
        ObjectInformationClass: OBJECT_INFORMATION_CLASS,
        ObjectInformation: *mut c_void,
        ObjectInformationLength: ULONG,
    ) -> NTSTATUS;

    pub fn NtDuplicateObject(
        SourceProcessHandle: HANDLE,
        SourceHandle: HANDLE,
        TargetProcessHandle: HANDLE,
        TargetHandle: *mut HANDLE,
        DesiredAccess: ACCESS_MASK,
        HandleAttributes: ULONG,
        Options: ULONG,
    ) -> NTSTATUS;

    pub fn NtMakeTemporaryObject(Handle: HANDLE) -> NTSTATUS;

    pub fn NtMakePermanentObject(Handle: HANDLE) -> NTSTATUS;

    pub fn NtSignalAndWaitForSingleObject(
        SignalHandle: HANDLE,
        WaitHandle: HANDLE,
        Alertable: BOOLEAN,
        Timeout: *mut LARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn NtWaitForSingleObject(
        Handle: HANDLE,
        Alertable: BOOLEAN,
        Timeout: *mut LARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn NtWaitForMultipleObjects(
        Count: ULONG,
        Handles: *const HANDLE,
        WaitType: WAIT_TYPE,
        Alertable: BOOLEAN,
        Timeout: *mut LARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn NtWaitForMultipleObjects32(
        Count: ULONG,
        Handles: *const i32,
        WaitType: WAIT_TYPE,
        Alertable: BOOLEAN,
        Timeout: *mut LARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn NtSetSecurityObject(
        Handle: HANDLE,
        SecurityInformation: SECURITY_INFORMATION,
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
    ) -> NTSTATUS;

    pub fn NtQuerySecurityObject(
        Handle: HANDLE,
        SecurityInformation: SECURITY_INFORMATION,
        SecurityDescriptor: PSECURITY_DESCRIPTOR,
        Length: ULONG,
        LengthNeeded: PULONG,
    ) -> NTSTATUS;

    pub fn NtClose(Handle: HANDLE) -> NTSTATUS;

    #[cfg(windows)]
    pub fn NtCompareObjects(
        FirstObjectHandle: HANDLE,
        SecondObjectHandle: HANDLE,
    ) -> NTSTATUS;

    pub fn NtCreateDirectoryObject(
        DirectoryHandle: *mut HANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    #[cfg(windows)]
    pub fn NtCreateDirectoryObjectEx(
        DirectoryHandle: *mut HANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        ShadowDirectoryHandle: HANDLE,
        Flags: ULONG,
    ) -> NTSTATUS;

    pub fn NtOpenDirectoryObject(
        DirectoryHandle: *mut HANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn NtQueryDirectoryObject(
        DirectoryHandle: HANDLE,
        Buffer: *mut c_void,
        Length: ULONG,
        ReturnSingleEntry: BOOLEAN,
        RestartScan: BOOLEAN,
        Context: *mut ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn NtCreatePrivateNamespace(
        NamespaceHandle: *mut HANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        BoundaryDescriptor: *mut OBJECT_BOUNDARY_DESCRIPTOR,
    ) -> NTSTATUS;

    pub fn NtOpenPrivateNamespace(
        NamespaceHandle: *mut HANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        BoundaryDescriptor: *mut OBJECT_BOUNDARY_DESCRIPTOR,
    ) -> NTSTATUS;

    pub fn NtDeletePrivateNamespace(NamespaceHandle: HANDLE) -> NTSTATUS;

    pub fn NtCreateSymbolicLinkObject(
        LinkHandle: *mut HANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        LinkTarget: *const UNICODE_STRING,
    ) -> NTSTATUS;

    pub fn NtOpenSymbolicLinkObject(
        LinkHandle: *mut HANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn NtQuerySymbolicLinkObject(
        LinkHandle: HANDLE,
        LinkTarget: *mut UNICODE_STRING,
        ReturnedLength: PULONG,
    ) -> NTSTATUS;

    #[cfg(windows)]
    pub fn NtSetInformationSymbolicLink(
        LinkHandle: HANDLE,
        SymbolicLinkInformationClass: SYMBOLIC_LINK_INFO_CLASS,
        SymbolicLinkInformation: *mut c_void,
        SymbolicLinkInformationLength: ULONG,
    ) -> NTSTATUS;
}