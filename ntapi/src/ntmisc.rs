use crate::ntdef::*;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VDMSERVICECLASS {
    VdmStartExecution = 0,
    VdmQueueInterrupt = 1,
    VdmDelayInterrupt = 2,
    VdmInitialize = 3,
    VdmFeatures = 4,
    VdmSetInt21Handler = 5,
    VdmQueryDir = 6,
    VdmPrinterDirectIoOpen = 7,
    VdmPrinterDirectIoClose = 8,
    VdmPrinterInitialize = 9,
    VdmSetLdtEntries = 10,
    VdmSetProcessLdtInfo = 11,
    VdmAdlibEmulation = 12,
    VdmPMCliControl = 13,
    VdmQueryVdmProcess = 14,
    VdmPreInitialize = 15,
}
pub type PVDMSERVICECLASS = *mut VDMSERVICECLASS;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SECURE_SETTING_VALUE_TYPE {
    SecureSettingValueTypeBoolean = 0,
    SecureSettingValueTypeUlong = 1,
    SecureSettingValueTypeBinary = 2,
    SecureSettingValueTypeString = 3,
    SecureSettingValueTypeUnknown = 4,
}
pub type PSECURE_SETTING_VALUE_TYPE = *mut SECURE_SETTING_VALUE_TYPE;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AHC_SERVICE_CLASS {
    ApphelpCacheServiceLookup = 0,
    ApphelpCacheServiceRemove = 1,
    ApphelpCacheServiceUpdate = 2,
    ApphelpCacheServiceClear = 3,
    ApphelpCacheServiceSnapStatistics = 4,
    ApphelpCacheServiceSnapCache = 5,
    ApphelpCacheServiceLookupCdb = 6,
    ApphelpCacheServiceRefreshCdb = 7,
    ApphelpCacheServiceMapQuirks = 8,
    ApphelpCacheServiceHwIdQuery = 9,
    ApphelpCacheServiceInitProcessData = 10,
    ApphelpCacheServiceLookupAndWriteToProcess = 11,
    ApphelpCacheServiceMax = 12,
}

pub type AHC_INFO_CLASS = i32;

#[repr(C)]
pub struct AHC_SERVICE_LOOKUP {
    pub InfoClass: AHC_INFO_CLASS,
    pub HintFlags: UINT,
    pub PackageAlias: UNICODE_STRING,
    pub FileHandle: HANDLE,
    pub ProcessHandle: HANDLE,
    pub ExeType: USHORT,
    pub Padding: USHORT,
    pub ExeSignature: UNICODE_STRING,
    pub Environment: PCZZWSTR,
    pub EnvironmentSize: UINT,
}
pub type PAHC_SERVICE_LOOKUP = *mut AHC_SERVICE_LOOKUP;

#[repr(C)]
pub struct AHC_SERVICE_REMOVE {
    pub InfoClass: AHC_INFO_CLASS,
    pub PackageAlias: UNICODE_STRING,
    pub FileHandle: HANDLE,
    pub ExeSignature: UNICODE_STRING,
}
pub type PAHC_SERVICE_REMOVE = *mut AHC_SERVICE_REMOVE;

#[repr(C)]
pub struct AHC_SERVICE_UPDATE {
    pub InfoClass: AHC_INFO_CLASS,
    pub PackageAlias: UNICODE_STRING,
    pub FileHandle: HANDLE,
    pub ExeSignature: UNICODE_STRING,
    pub Data: PVOID,
    pub DataSize: ULONG,
}
pub type PAHC_SERVICE_UPDATE = *mut AHC_SERVICE_UPDATE;

#[repr(C)]
pub struct AHC_SERVICE_CLEAR {
    pub InfoClass: AHC_INFO_CLASS,
}
pub type PAHC_SERVICE_CLEAR = *mut AHC_SERVICE_CLEAR;

#[repr(C)]
pub struct AHC_SERVICE_LOOKUP_CDB {
    pub Name: UNICODE_STRING,
}
pub type PAHC_SERVICE_LOOKUP_CDB = *mut AHC_SERVICE_LOOKUP_CDB;