use crate::ntdef::*;

pub const REG_MAX_KEY_VALUE_NAME_LENGTH: u32 = 32767;
pub const REG_MAX_KEY_NAME_LENGTH: u32 = 512;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KEY_INFORMATION_CLASS {
    KeyBasicInformation = 0,
    KeyNodeInformation = 1,
    KeyFullInformation = 2,
    KeyNameInformation = 3,
    KeyCachedInformation = 4,
    KeyFlagsInformation = 5,
    KeyVirtualizationInformation = 6,
    KeyHandleTagsInformation = 7,
    KeyTrustInformation = 8,
    KeyLayerInformation = 9,
    MaxKeyInfoClass = 10,
}

#[repr(C)]
pub struct KEY_BASIC_INFORMATION {
    pub LastWriteTime: LARGE_INTEGER,
    pub TitleIndex: ULONG,
    pub NameLength: ULONG,
    pub Name: [u16; 1],
}
pub type PKEY_BASIC_INFORMATION = *mut KEY_BASIC_INFORMATION;

#[repr(C)]
pub struct KEY_NODE_INFORMATION {
    pub LastWriteTime: LARGE_INTEGER,
    pub TitleIndex: ULONG,
    pub ClassOffset: ULONG,
    pub ClassLength: ULONG,
    pub NameLength: ULONG,
    pub Name: [u16; 1],
}
pub type PKEY_NODE_INFORMATION = *mut KEY_NODE_INFORMATION;

#[repr(C)]
pub struct KEY_FULL_INFORMATION {
    pub LastWriteTime: LARGE_INTEGER,
    pub TitleIndex: ULONG,
    pub ClassOffset: ULONG,
    pub ClassLength: ULONG,
    pub SubKeys: ULONG,
    pub MaxNameLength: ULONG,
    pub MaxClassLength: ULONG,
    pub Values: ULONG,
    pub MaxValueNameLength: ULONG,
    pub MaxValueDataLength: ULONG,
    pub Class: [u16; 1],
}
pub type PKEY_FULL_INFORMATION = *mut KEY_FULL_INFORMATION;

#[repr(C)]
pub struct KEY_NAME_INFORMATION {
    pub NameLength: ULONG,
    pub Name: [u16; 1],
}
pub type PKEY_NAME_INFORMATION = *mut KEY_NAME_INFORMATION;

#[repr(C)]
pub struct KEY_CACHED_INFORMATION {
    pub LastWriteTime: LARGE_INTEGER,
    pub TitleIndex: ULONG,
    pub SubKeys: ULONG,
    pub MaxNameLength: ULONG,
    pub Values: ULONG,
    pub MaxValueNameLength: ULONG,
    pub MaxValueDataLength: ULONG,
    pub NameLength: ULONG,
    pub Name: [u16; 1],
}
pub type PKEY_CACHED_INFORMATION = *mut KEY_CACHED_INFORMATION;

pub const REG_FLAG_VOLATILE: u32 = 0x0001;
pub const REG_FLAG_LINK: u32 = 0x0002;

pub const REG_KEY_RESERVED_FLAG: u32 = 0x0001;
pub const REG_KEY_DONT_VIRTUALIZE: u32 = 0x0002;
pub const REG_KEY_DONT_SILENT_FAIL: u32 = 0x0004;
pub const REG_KEY_RECURSE_FLAG: u32 = 0x0008;

#[repr(C)]
pub struct KEY_FLAGS_INFORMATION {
    pub Wow64Flags: ULONG,
    pub KeyFlags: ULONG,
    pub ControlFlags: ULONG,
}
pub type PKEY_FLAGS_INFORMATION = *mut KEY_FLAGS_INFORMATION;

#[repr(C)]
pub struct KEY_VIRTUALIZATION_INFORMATION {
    pub VirtualizationCandidate: u32,
    pub VirtualizationEnabled: u32,
    pub VirtualTarget: u32,
    pub VirtualStore: u32,
    pub VirtualSource: u32,
    pub Reserved: u32,
}
pub type PKEY_VIRTUALIZATION_INFORMATION = *mut KEY_VIRTUALIZATION_INFORMATION;

#[repr(C)]
pub struct KEY_TRUST_INFORMATION {
    pub TrustedKey: u32,
    pub Reserved: u32,
}
pub type PKEY_TRUST_INFORMATION = *mut KEY_TRUST_INFORMATION;

#[repr(C)]
pub struct KEY_LAYER_INFORMATION {
    pub IsTombstone: u32,
    pub IsSupersedeLocal: u32,
    pub IsSupersedeTree: u32,
    pub ClassIsInherited: u32,
    pub Reserved: u32,
}
pub type PKEY_LAYER_INFORMATION = *mut KEY_LAYER_INFORMATION;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KEY_SET_INFORMATION_CLASS {
    KeyWriteTimeInformation = 0,
    KeyWow64FlagsInformation = 1,
    KeyControlFlagsInformation = 2,
    KeySetVirtualizationInformation = 3,
    KeySetDebugInformation = 4,
    KeySetHandleTagsInformation = 5,
    KeySetLayerInformation = 6,
    MaxKeySetInfoClass = 7,
}

#[repr(C)]
pub struct KEY_WRITE_TIME_INFORMATION {
    pub LastWriteTime: LARGE_INTEGER,
}
pub type PKEY_WRITE_TIME_INFORMATION = *mut KEY_WRITE_TIME_INFORMATION;

#[repr(C)]
pub struct KEY_WOW64_FLAGS_INFORMATION {
    pub UserFlags: ULONG,
}
pub type PKEY_WOW64_FLAGS_INFORMATION = *mut KEY_WOW64_FLAGS_INFORMATION;

#[repr(C)]
pub struct KEY_HANDLE_TAGS_INFORMATION {
    pub HandleTags: ULONG,
}
pub type PKEY_HANDLE_TAGS_INFORMATION = *mut KEY_HANDLE_TAGS_INFORMATION;

#[repr(C)]
pub struct KEY_SET_LAYER_INFORMATION {
    pub IsTombstone: u32,
    pub IsSupersedeLocal: u32,
    pub IsSupersedeTree: u32,
    pub ClassIsInherited: u32,
    pub Reserved: u32,
}
pub type PKEY_SET_LAYER_INFORMATION = *mut KEY_SET_LAYER_INFORMATION;

#[repr(C)]
pub struct KEY_CONTROL_FLAGS_INFORMATION {
    pub ControlFlags: ULONG,
}
pub type PKEY_CONTROL_FLAGS_INFORMATION = *mut KEY_CONTROL_FLAGS_INFORMATION;

#[repr(C)]
pub struct KEY_SET_VIRTUALIZATION_INFORMATION {
    pub VirtualTarget: u32,
    pub VirtualStore: u32,
    pub VirtualSource: u32,
    pub Reserved: u32,
}
pub type PKEY_SET_VIRTUALIZATION_INFORMATION = *mut KEY_SET_VIRTUALIZATION_INFORMATION;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KEY_VALUE_INFORMATION_CLASS {
    KeyValueBasicInformation = 0,
    KeyValueFullInformation = 1,
    KeyValuePartialInformation = 2,
    KeyValueFullInformationAlign64 = 3,
    KeyValuePartialInformationAlign64 = 4,
    KeyValueLayerInformation = 5,
    MaxKeyValueInfoClass = 6,
}

#[repr(C)]
pub struct KEY_VALUE_BASIC_INFORMATION {
    pub TitleIndex: ULONG,
    pub Type: ULONG,
    pub NameLength: ULONG,
    pub Name: [u16; 1],
}
pub type PKEY_VALUE_BASIC_INFORMATION = *mut KEY_VALUE_BASIC_INFORMATION;

#[repr(C)]
pub struct KEY_VALUE_FULL_INFORMATION {
    pub TitleIndex: ULONG,
    pub Type: ULONG,
    pub DataOffset: ULONG,
    pub DataLength: ULONG,
    pub NameLength: ULONG,
    pub Name: [u16; 1],
}
pub type PKEY_VALUE_FULL_INFORMATION = *mut KEY_VALUE_FULL_INFORMATION;

#[repr(C, align(8))]
pub struct KEY_VALUE_FULL_INFORMATION_ALIGN64 {
    pub TitleIndex: ULONG,
    pub Type: ULONG,
    pub DataOffset: ULONG,
    pub DataLength: ULONG,
    pub NameLength: ULONG,
    pub Name: [u16; 1],
}
pub type PKEY_VALUE_FULL_INFORMATION_ALIGN64 = *mut KEY_VALUE_FULL_INFORMATION_ALIGN64;

#[repr(C)]
pub struct KEY_VALUE_PARTIAL_INFORMATION {
    pub TitleIndex: ULONG,
    pub Type: ULONG,
    pub DataLength: ULONG,
    pub Data: [UCHAR; 1],
}
pub type PKEY_VALUE_PARTIAL_INFORMATION = *mut KEY_VALUE_PARTIAL_INFORMATION;

#[repr(C, align(8))]
pub struct KEY_VALUE_PARTIAL_INFORMATION_ALIGN64 {
    pub Type: ULONG,
    pub DataLength: ULONG,
    pub Data: [UCHAR; 1],
}
pub type PKEY_VALUE_PARTIAL_INFORMATION_ALIGN64 = *mut KEY_VALUE_PARTIAL_INFORMATION_ALIGN64;

#[repr(C)]
pub struct KEY_VALUE_LAYER_INFORMATION {
    pub IsTombstone: u32,
    pub Reserved: u32,
}
pub type PKEY_VALUE_LAYER_INFORMATION = *mut KEY_VALUE_LAYER_INFORMATION;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CM_EXTENDED_PARAMETER_TYPE {
    CmExtendedParameterInvalidType = 0,
    CmExtendedParameterTrustClassKey = 1,
    CmExtendedParameterEvent = 2,
    CmExtendedParameterFileAccessToken = 3,
    CmExtendedParameterMax = 4,
}

pub const CM_EXTENDED_PARAMETER_TYPE_BITS: u32 = 8;

#[repr(C, align(8))]
pub struct CM_EXTENDED_PARAMETER {
    pub Type: u64,
    pub Reserved: u64,
    pub Anonymous: CM_EXTENDED_PARAMETER_ANON,
}

#[repr(C)]
pub union CM_EXTENDED_PARAMETER_ANON {
    pub ULong64: u64,
    pub Pointer: PVOID,
    pub Size: SIZE_T,
    pub Handle: HANDLE,
    pub ULong: ULONG,
    pub AccessMask: ACCESS_MASK,
}
pub type PCM_EXTENDED_PARAMETER = *mut CM_EXTENDED_PARAMETER;

#[repr(C)]
pub struct KEY_VALUE_ENTRY {
    pub ValueName: PUNICODE_STRING,
    pub DataLength: ULONG,
    pub DataOffset: ULONG,
    pub Type: ULONG,
}
pub type PKEY_VALUE_ENTRY = *mut KEY_VALUE_ENTRY;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum REG_ACTION {
    KeyAdded = 0,
    KeyRemoved = 1,
    KeyModified = 2,
}