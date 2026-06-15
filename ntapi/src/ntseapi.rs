use crate::{ntdef::*, ntrtl::PACL};

pub const SE_MIN_WELL_KNOWN_PRIVILEGE: i32 = 2;
pub const SE_CREATE_TOKEN_PRIVILEGE: i32 = 2;
pub const SE_ASSIGNPRIMARYTOKEN_PRIVILEGE: i32 = 3;
pub const SE_LOCK_MEMORY_PRIVILEGE: i32 = 4;
pub const SE_INCREASE_QUOTA_PRIVILEGE: i32 = 5;
pub const SE_MACHINE_ACCOUNT_PRIVILEGE: i32 = 6;
pub const SE_TCB_PRIVILEGE: i32 = 7;
pub const SE_SECURITY_PRIVILEGE: i32 = 8;
pub const SE_TAKE_OWNERSHIP_PRIVILEGE: i32 = 9;
pub const SE_LOAD_DRIVER_PRIVILEGE: i32 = 10;
pub const SE_SYSTEM_PROFILE_PRIVILEGE: i32 = 11;
pub const SE_SYSTEMTIME_PRIVILEGE: i32 = 12;
pub const SE_PROF_SINGLE_PROCESS_PRIVILEGE: i32 = 13;
pub const SE_INC_BASE_PRIORITY_PRIVILEGE: i32 = 14;
pub const SE_CREATE_PAGEFILE_PRIVILEGE: i32 = 15;
pub const SE_CREATE_PERMANENT_PRIVILEGE: i32 = 16;
pub const SE_BACKUP_PRIVILEGE: i32 = 17;
pub const SE_RESTORE_PRIVILEGE: i32 = 18;
pub const SE_SHUTDOWN_PRIVILEGE: i32 = 19;
pub const SE_DEBUG_PRIVILEGE: i32 = 20;
pub const SE_AUDIT_PRIVILEGE: i32 = 21;
pub const SE_SYSTEM_ENVIRONMENT_PRIVILEGE: i32 = 22;
pub const SE_CHANGE_NOTIFY_PRIVILEGE: i32 = 23;
pub const SE_REMOTE_SHUTDOWN_PRIVILEGE: i32 = 24;
pub const SE_UNDOCK_PRIVILEGE: i32 = 25;
pub const SE_SYNC_AGENT_PRIVILEGE: i32 = 26;
pub const SE_ENABLE_DELEGATION_PRIVILEGE: i32 = 27;
pub const SE_MANAGE_VOLUME_PRIVILEGE: i32 = 28;
pub const SE_IMPERSONATE_PRIVILEGE: i32 = 29;
pub const SE_CREATE_GLOBAL_PRIVILEGE: i32 = 30;
pub const SE_TRUSTED_CREDMAN_ACCESS_PRIVILEGE: i32 = 31;
pub const SE_RELABEL_PRIVILEGE: i32 = 32;
pub const SE_INC_WORKING_SET_PRIVILEGE: i32 = 33;
pub const SE_TIME_ZONE_PRIVILEGE: i32 = 34;
pub const SE_CREATE_SYMBOLIC_LINK_PRIVILEGE: i32 = 35;
pub const SE_DELEGATE_SESSION_USER_IMPERSONATE_PRIVILEGE: i32 = 36;

#[repr(u32)]
pub enum TOKEN_INFORMATION_CLASS {
    TokenUser = 1,
    TokenGroups,
    TokenPrivileges,
    TokenOwner,
    TokenPrimaryGroup,
    TokenDefaultDacl,
    TokenSource,
    TokenType,
    TokenImpersonationLevel,
    TokenStatistics,
    TokenRestrictedSids,
    TokenSessionId,
    TokenGroupsAndPrivileges,
    TokenSessionReference,
    TokenSandBoxInert,
    TokenAuditPolicy,
    TokenOrigin,
    TokenElevationType,
    TokenLinkedToken,
    TokenElevation,
    TokenHasRestrictions,
    TokenAccessInformation,
    TokenVirtualizationAllowed,
    TokenVirtualizationEnabled,
    TokenIntegrityLevel,
    TokenUIAccess,
    TokenMandatoryPolicy,
    TokenLogonSid,
    TokenIsAppContainer,
    TokenCapabilities,
    TokenAppContainerSid,
    TokenAppContainerNumber,
    TokenUserClaimAttributes,
    TokenDeviceClaimAttributes,
    TokenRestrictedUserClaimAttributes,
    TokenRestrictedDeviceClaimAttributes,
    TokenDeviceGroups,
    TokenRestrictedDeviceGroups,
    TokenSecurityAttributes,
    TokenIsRestricted,
    TokenProcessTrustLevel,
    TokenPrivateNameSpace,
    TokenSingletonAttributes,
    TokenBnoIsolation,
    TokenChildProcessFlags,
    TokenIsLessPrivilegedAppContainer,
    TokenIsSandboxed,
    TokenIsAppSilo,
    TokenLoggingInformation,
    TokenLearningMode,
    MaxTokenInfoClass,
}

pub type PTOKEN_INFORMATION_CLASS = *mut TOKEN_INFORMATION_CLASS;

#[repr(C)]
pub struct LUID_AND_ATTRIBUTES {
    pub Luid: LUID,
    pub Attributes: DWORD,
}

pub type PLUID_AND_ATTRIBUTES = *mut LUID_AND_ATTRIBUTES;
pub type LUID_AND_ATTRIBUTES_ARRAY = [LUID_AND_ATTRIBUTES; 1];
pub type PLUID_AND_ATTRIBUTES_ARRAY = *mut LUID_AND_ATTRIBUTES_ARRAY;

#[repr(u32)]
pub enum TOKEN_ELEVATION_TYPE {
    TokenElevationTypeDefault = 1,
    TokenElevationTypeFull,
    TokenElevationTypeLimited,
}

pub type PTOKEN_ELEVATION_TYPE = *mut TOKEN_ELEVATION_TYPE;

pub type PCLAIMS_BLOB = PVOID;

#[repr(C)]
pub struct SID_AND_ATTRIBUTES {
    pub Sid: PSID,
    pub Attributes: DWORD,
}

pub type PSID_AND_ATTRIBUTES = *mut SID_AND_ATTRIBUTES;

pub const SID_HASH_SIZE: usize = 32;

#[repr(C)]
pub struct ATTRIBUTES_AND_SID {
    pub Attributes: UINT32,
    pub SidStart: DWORD,
}

pub type PATTRIBUTES_AND_SID = *mut ATTRIBUTES_AND_SID;

#[repr(C)]
pub struct SID_AND_ATTRIBUTES_HASH {
    pub SidCount: DWORD,
    pub SidAttr: PSID_AND_ATTRIBUTES,
    pub Hash: [SID_HASH_ENTRY; SID_HASH_SIZE],
}

pub type PSID_AND_ATTRIBUTES_HASH = *mut SID_AND_ATTRIBUTES_HASH;

#[repr(u32)]
pub enum SECURITY_IMPERSONATION_LEVEL {
    SecurityAnonymous,
    SecurityIdentification,
    SecurityImpersonation,
    SecurityDelegation,
}

pub type PSECURITY_IMPERSONATION_LEVEL = *mut SECURITY_IMPERSONATION_LEVEL;

pub type SID_HASH_ENTRY = ULONG;
pub type PSID_HASH_ENTRY = *mut SID_HASH_ENTRY;

#[repr(C)]
pub struct TOKEN_USER {
    pub User: SID_AND_ATTRIBUTES,
}

pub type PTOKEN_USER = *mut TOKEN_USER;

#[repr(C)]
pub struct SE_TOKEN_USER {
    pub TokenUser: TOKEN_USER,
    pub Sid: SID,
    pub Buffer: [u8; SECURITY_MAX_SID_SIZE],
}

pub type PSE_TOKEN_USER = *mut SE_TOKEN_USER;

pub const TOKEN_USER_MAX_SIZE: usize = core::mem::size_of::<TOKEN_USER>() + SECURITY_MAX_SID_SIZE;

#[repr(C)]
pub struct TOKEN_GROUPS {
    pub GroupCount: ULONG,
    pub Groups: [SID_AND_ATTRIBUTES; 1],
}

pub type PTOKEN_GROUPS = *mut TOKEN_GROUPS;

#[repr(C)]
pub struct TOKEN_PRIVILEGES {
    pub PrivilegeCount: ULONG,
    pub Privileges: [LUID_AND_ATTRIBUTES; 1],
}

pub type PTOKEN_PRIVILEGES = *mut TOKEN_PRIVILEGES;

#[repr(C)]
pub struct TOKEN_OWNER {
    pub Owner: PSID,
}

pub type PTOKEN_OWNER = *mut TOKEN_OWNER;

pub const TOKEN_OWNER_MAX_SIZE: usize = core::mem::size_of::<TOKEN_OWNER>() + SECURITY_MAX_SID_SIZE;

#[repr(C)]
pub struct TOKEN_PRIMARY_GROUP {
    pub PrimaryGroup: PSID,
}

pub type PTOKEN_PRIMARY_GROUP = *mut TOKEN_PRIMARY_GROUP;

#[repr(C)]
pub struct TOKEN_DEFAULT_DACL {
    pub DefaultDacl: PACL,
}

pub type PTOKEN_DEFAULT_DACL = *mut TOKEN_DEFAULT_DACL;

pub const TOKEN_SOURCE_LENGTH: usize = 8;

#[repr(C)]
pub struct TOKEN_SOURCE {
    pub SourceName: [i8; TOKEN_SOURCE_LENGTH],
    pub SourceIdentifier: LUID,
}

pub type PTOKEN_SOURCE = *mut TOKEN_SOURCE;

#[repr(u32)]
pub enum TOKEN_TYPE {
    TokenPrimary = 1,
    TokenImpersonation,
}

#[repr(C)]
pub struct TOKEN_USER_CLAIMS {
    pub UserClaims: PCLAIMS_BLOB,
}

pub type PTOKEN_USER_CLAIMS = *mut TOKEN_USER_CLAIMS;

#[repr(C)]
pub struct TOKEN_DEVICE_CLAIMS {
    pub DeviceClaims: PCLAIMS_BLOB,
}

pub type PTOKEN_DEVICE_CLAIMS = *mut TOKEN_DEVICE_CLAIMS;

#[repr(C)]
pub struct TOKEN_GROUPS_AND_PRIVILEGES {
    pub SidCount: ULONG,
    pub SidLength: ULONG,
    pub Sids: PSID_AND_ATTRIBUTES,
    pub RestrictedSidCount: ULONG,
    pub RestrictedSidLength: ULONG,
    pub RestrictedSids: PSID_AND_ATTRIBUTES,
    pub PrivilegeCount: ULONG,
    pub PrivilegeLength: ULONG,
    pub Privileges: PLUID_AND_ATTRIBUTES,
    pub AuthenticationId: LUID,
}

pub type PTOKEN_GROUPS_AND_PRIVILEGES = *mut TOKEN_GROUPS_AND_PRIVILEGES;

#[repr(C)]
pub struct TOKEN_LINKED_TOKEN {
    pub LinkedToken: HANDLE,
}

pub type PTOKEN_LINKED_TOKEN = *mut TOKEN_LINKED_TOKEN;

#[repr(C)]
pub struct TOKEN_ELEVATION {
    pub TokenIsElevated: ULONG,
}

pub type PTOKEN_ELEVATION = *mut TOKEN_ELEVATION;

#[repr(C)]
pub struct TOKEN_MANDATORY_LABEL {
    pub Label: SID_AND_ATTRIBUTES,
}

pub type PTOKEN_MANDATORY_LABEL = *mut TOKEN_MANDATORY_LABEL;

pub const TOKEN_MANDATORY_POLICY_OFF: u32 = 0x0;
pub const TOKEN_MANDATORY_POLICY_NO_WRITE_UP: u32 = 0x1;
pub const TOKEN_MANDATORY_POLICY_NEW_PROCESS_MIN: u32 = 0x2;
pub const TOKEN_MANDATORY_POLICY_VALID_MASK: u32 = TOKEN_MANDATORY_POLICY_NO_WRITE_UP | TOKEN_MANDATORY_POLICY_NEW_PROCESS_MIN;

#[repr(C)]
pub struct TOKEN_MANDATORY_POLICY {
    pub Policy: ULONG,
}

pub type PTOKEN_MANDATORY_POLICY = *mut TOKEN_MANDATORY_POLICY;

pub type PSECURITY_ATTRIBUTES_OPAQUE = PVOID;

#[repr(C)]
pub struct TOKEN_ACCESS_INFORMATION {
    pub SidHash: PSID_AND_ATTRIBUTES_HASH,
    pub RestrictedSidHash: PSID_AND_ATTRIBUTES_HASH,
    pub Privileges: PTOKEN_PRIVILEGES,
    pub AuthenticationId: LUID,
    pub TokenType: TOKEN_TYPE,
    pub ImpersonationLevel: SECURITY_IMPERSONATION_LEVEL,
    pub MandatoryPolicy: TOKEN_MANDATORY_POLICY,
    pub Flags: ULONG,
    pub AppContainerNumber: ULONG,
    pub PackageSid: PSID,
    pub CapabilitiesHash: PSID_AND_ATTRIBUTES_HASH,
    pub TrustLevelSid: PSID,
    pub SecurityAttributes: PSECURITY_ATTRIBUTES_OPAQUE,
}

pub type PTOKEN_ACCESS_INFORMATION = *mut TOKEN_ACCESS_INFORMATION;

#[repr(C)]
pub struct TOKEN_LOGGING_INFORMATION {
    pub TokenType: TOKEN_TYPE,
    pub TokenElevation: TOKEN_ELEVATION,
    pub TokenElevationType: TOKEN_ELEVATION_TYPE,
    pub ImpersonationLevel: SECURITY_IMPERSONATION_LEVEL,
    pub IntegrityLevel: ULONG,
    pub User: SID_AND_ATTRIBUTES,
    pub TrustLevelSid: PSID,
    pub SessionId: ULONG,
    pub AppContainerNumber: ULONG,
    pub AuthenticationId: LUID,
    pub GroupCount: ULONG,
    pub GroupsLength: ULONG,
    pub Groups: PSID_AND_ATTRIBUTES,
}

pub type PTOKEN_LOGGING_INFORMATION = *mut TOKEN_LOGGING_INFORMATION;

pub const TOKEN_SECURITY_ATTRIBUTE_TYPE_INVALID: u16 = 0x00;
pub const TOKEN_SECURITY_ATTRIBUTE_TYPE_INT64: u16 = 0x01;
pub const TOKEN_SECURITY_ATTRIBUTE_TYPE_UINT64: u16 = 0x02;
pub const TOKEN_SECURITY_ATTRIBUTE_TYPE_STRING: u16 = 0x03;
pub const TOKEN_SECURITY_ATTRIBUTE_TYPE_FQBN: u16 = 0x04;
pub const TOKEN_SECURITY_ATTRIBUTE_TYPE_SID: u16 = 0x05;
pub const TOKEN_SECURITY_ATTRIBUTE_TYPE_BOOLEAN: u16 = 0x06;
pub const TOKEN_SECURITY_ATTRIBUTE_TYPE_OCTET_STRING: u16 = 0x10;

pub const TOKEN_SECURITY_ATTRIBUTE_NON_INHERITABLE: u32 = 0x0001;
pub const TOKEN_SECURITY_ATTRIBUTE_VALUE_CASE_SENSITIVE: u32 = 0x0002;
pub const TOKEN_SECURITY_ATTRIBUTE_USE_FOR_DENY_ONLY: u32 = 0x0004;
pub const TOKEN_SECURITY_ATTRIBUTE_DISABLED_BY_DEFAULT: u32 = 0x0008;
pub const TOKEN_SECURITY_ATTRIBUTE_DISABLED: u32 = 0x0010;
pub const TOKEN_SECURITY_ATTRIBUTE_MANDATORY: u32 = 0x0020;
pub const TOKEN_SECURITY_ATTRIBUTE_COMPARE_IGNORE: u32 = 0x0040;

pub const TOKEN_SECURITY_ATTRIBUTE_VALID_FLAGS: u32 = TOKEN_SECURITY_ATTRIBUTE_NON_INHERITABLE
    | TOKEN_SECURITY_ATTRIBUTE_VALUE_CASE_SENSITIVE
    | TOKEN_SECURITY_ATTRIBUTE_USE_FOR_DENY_ONLY
    | TOKEN_SECURITY_ATTRIBUTE_DISABLED_BY_DEFAULT
    | TOKEN_SECURITY_ATTRIBUTE_DISABLED
    | TOKEN_SECURITY_ATTRIBUTE_MANDATORY;

pub const TOKEN_SECURITY_ATTRIBUTE_CUSTOM_FLAGS: u32 = 0xffff0000;

#[repr(C)]
pub struct TOKEN_SECURITY_ATTRIBUTE_FQBN_VALUE {
    pub Version: ULONG64,
    pub Name: UNICODE_STRING,
}

pub type PTOKEN_SECURITY_ATTRIBUTE_FQBN_VALUE = *mut TOKEN_SECURITY_ATTRIBUTE_FQBN_VALUE;

#[repr(C)]
pub struct TOKEN_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    pub Value: PVOID,
    pub ValueLength: ULONG,
}

pub type PTOKEN_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE = *mut TOKEN_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE;

#[repr(C)]
pub struct TOKEN_SECURITY_ATTRIBUTE_V1 {
    pub Name: UNICODE_STRING,
    pub ValueType: USHORT,
    pub Reserved: USHORT,
    pub Flags: ULONG,
    pub ValueCount: ULONG,
    pub Values: TOKEN_SECURITY_ATTRIBUTE_V1_VALUES,
}

#[repr(C)]
pub union TOKEN_SECURITY_ATTRIBUTE_V1_VALUES {
    pub Int64: PLONG64,
    pub Uint64: PULONG64,
    pub String: PUNICODE_STRING,
    pub Fqbn: PTOKEN_SECURITY_ATTRIBUTE_FQBN_VALUE,
    pub OctetString: PTOKEN_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE,
}

pub type PTOKEN_SECURITY_ATTRIBUTE_V1 = *mut TOKEN_SECURITY_ATTRIBUTE_V1;

#[repr(C)]
pub struct TOKEN_SECURITY_ATTRIBUTE_RELATIVE_V1 {
    pub Name: UNICODE_STRING,
    pub ValueType: USHORT,
    pub Reserved: USHORT,
    pub Flags: ULONG,
    pub ValueCount: ULONG,
    pub Values: TOKEN_SECURITY_ATTRIBUTE_RELATIVE_V1_VALUES,
}

#[repr(C)]
pub union TOKEN_SECURITY_ATTRIBUTE_RELATIVE_V1_VALUES {
    pub Int64: [ULONG; 1],
    pub Uint64: [ULONG; 1],
    pub String: [ULONG; 1],
    pub Fqbn: [ULONG; 1],
    pub OctetString: [ULONG; 1],
}

pub type PTOKEN_SECURITY_ATTRIBUTE_RELATIVE_V1 = *mut TOKEN_SECURITY_ATTRIBUTE_RELATIVE_V1;

pub const TOKEN_SECURITY_ATTRIBUTES_INFORMATION_VERSION_V1: u16 = 1;

#[repr(C)]
pub struct TOKEN_SECURITY_ATTRIBUTES_INFORMATION {
    pub Version: USHORT,
    pub Reserved: USHORT,
    pub AttributeCount: ULONG,
    pub AttributeV1: PTOKEN_SECURITY_ATTRIBUTE_V1,
}

pub type PTOKEN_SECURITY_ATTRIBUTES_INFORMATION = *mut TOKEN_SECURITY_ATTRIBUTES_INFORMATION;

#[repr(u32)]
pub enum TOKEN_SECURITY_ATTRIBUTE_OPERATION {
    TOKEN_SECURITY_ATTRIBUTE_OPERATION_NONE,
    TOKEN_SECURITY_ATTRIBUTE_OPERATION_REPLACE_ALL,
    TOKEN_SECURITY_ATTRIBUTE_OPERATION_ADD,
    TOKEN_SECURITY_ATTRIBUTE_OPERATION_DELETE,
    TOKEN_SECURITY_ATTRIBUTE_OPERATION_REPLACE,
}

pub type PTOKEN_SECURITY_ATTRIBUTE_OPERATION = *mut TOKEN_SECURITY_ATTRIBUTE_OPERATION;

#[repr(C)]
pub struct TOKEN_SECURITY_ATTRIBUTES_AND_OPERATION_INFORMATION {
    pub Attributes: PTOKEN_SECURITY_ATTRIBUTES_INFORMATION,
    pub Operations: PTOKEN_SECURITY_ATTRIBUTE_OPERATION,
}

pub type PTOKEN_SECURITY_ATTRIBUTES_AND_OPERATION_INFORMATION = *mut TOKEN_SECURITY_ATTRIBUTES_AND_OPERATION_INFORMATION;

#[repr(C)]
pub struct TOKEN_PROCESS_TRUST_LEVEL {
    pub TrustLevelSid: PSID,
}

pub type PTOKEN_PROCESS_TRUST_LEVEL = *mut TOKEN_PROCESS_TRUST_LEVEL;

#[repr(C)]
pub struct SE_FILE_CACHE_CLAIM_INFORMATION {
    pub Size: ULONG,
    pub Claim: PVOID,
}
pub type PSE_FILE_CACHE_CLAIM_INFORMATION = *mut SE_FILE_CACHE_CLAIM_INFORMATION;

#[repr(C)]
pub struct SE_SET_FILE_CACHE_INFORMATION {
    pub Size: ULONG,
    pub CatalogDirectoryPath: UNICODE_STRING,
    pub OriginClaimInfo: SE_FILE_CACHE_CLAIM_INFORMATION,
}
pub type PSE_SET_FILE_CACHE_INFORMATION = *mut SE_SET_FILE_CACHE_INFORMATION;

pub const DISABLE_MAX_PRIVILEGE: u32 = 0x1;
pub const SANDBOX_INERT: u32 = 0x2;
pub const LUA_TOKEN: u32 = 0x4;
pub const WRITE_RESTRICTED: u32 = 0x8;

pub const KSEC_DEVICE_NAME: &str = "\\Device\\KSecDD";

pub const IOCTL_KSEC_CONNECT_LSA: u32 = 0x00090000;
pub const IOCTL_KSEC_RNG: u32 = 0x00090004;
pub const IOCTL_KSEC_RNG_REKEY: u32 = 0x00090008;
pub const IOCTL_KSEC_ENCRYPT_MEMORY: u32 = 0x0009000C;
pub const IOCTL_KSEC_DECRYPT_MEMORY: u32 = 0x00090010;
pub const IOCTL_KSEC_ENCRYPT_MEMORY_CROSS_PROC: u32 = 0x00090014;
pub const IOCTL_KSEC_DECRYPT_MEMORY_CROSS_PROC: u32 = 0x00090018;
pub const IOCTL_KSEC_ENCRYPT_MEMORY_SAME_LOGON: u32 = 0x0009001C;
pub const IOCTL_KSEC_DECRYPT_MEMORY_SAME_LOGON: u32 = 0x00090020;
pub const IOCTL_KSEC_FIPS_GET_FUNCTION_TABLE: u32 = 0x00090024;
pub const IOCTL_KSEC_ALLOC_POOL: u32 = 0x00090028;
pub const IOCTL_KSEC_FREE_POOL: u32 = 0x0009002C;
pub const IOCTL_KSEC_COPY_POOL: u32 = 0x00090030;
pub const IOCTL_KSEC_DUPLICATE_HANDLE: u32 = 0x00090034;
pub const IOCTL_KSEC_REGISTER_EXTENSION: u32 = 0x00090038;
pub const IOCTL_KSEC_CLIENT_CALLBACK: u32 = 0x0009003C;
pub const IOCTL_KSEC_GET_BCRYPT_EXTENSION: u32 = 0x00090040;
pub const IOCTL_KSEC_GET_SSL_EXTENSION: u32 = 0x00090044;
pub const IOCTL_KSEC_GET_DEVICECONTROL_EXTENSION: u32 = 0x00090048;
pub const IOCTL_KSEC_ALLOC_VM: u32 = 0x0009004C;
pub const IOCTL_KSEC_FREE_VM: u32 = 0x00090050;
pub const IOCTL_KSEC_COPY_VM: u32 = 0x00090054;
pub const IOCTL_KSEC_CLIENT_FREE_VM: u32 = 0x00090058;
pub const IOCTL_KSEC_INSERT_PROTECTED_PROCESS_ADDRESS: u32 = 0x0009005C;
pub const IOCTL_KSEC_REMOVE_PROTECTED_PROCESS_ADDRESS: u32 = 0x00090060;
pub const IOCTL_KSEC_GET_BCRYPT_EXTENSION2: u32 = 0x00090064;
pub const IOCTL_KSEC_IPC_GET_QUEUED_FUNCTION_CALLS: u32 = 0x00090068;
pub const IOCTL_KSEC_IPC_SET_FUNCTION_RETURN: u32 = 0x0009006C;