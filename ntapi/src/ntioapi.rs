use crate::ntdef::*;

pub const FILE_SHARE_NONE: u32 = 0x00000000;
pub const FILE_SHARE_READ: u32 = 0x00000001;
pub const FILE_SHARE_WRITE: u32 = 0x00000002;
pub const FILE_SHARE_DELETE: u32 = 0x00000004;

pub const FILE_SUPERSEDE: u32 = 0x00000000;
pub const FILE_OPEN: u32 = 0x00000001;
pub const FILE_CREATE: u32 = 0x00000002;
pub const FILE_OPEN_IF: u32 = 0x00000003;
pub const FILE_OVERWRITE: u32 = 0x00000004;
pub const FILE_OVERWRITE_IF: u32 = 0x00000005;
pub const FILE_MAXIMUM_DISPOSITION: u32 = 0x00000005;

pub const FILE_DIRECTORY_FILE: u32 = 0x00000001;
pub const FILE_WRITE_THROUGH: u32 = 0x00000002;
pub const FILE_SEQUENTIAL_ONLY: u32 = 0x00000004;
pub const FILE_NO_INTERMEDIATE_BUFFERING: u32 = 0x00000008;
pub const FILE_SYNCHRONOUS_IO_ALERT: u32 = 0x00000010;
pub const FILE_SYNCHRONOUS_IO_NONALERT: u32 = 0x00000020;
pub const FILE_NON_DIRECTORY_FILE: u32 = 0x00000040;
pub const FILE_CREATE_TREE_CONNECTION: u32 = 0x00000080;
pub const FILE_COMPLETE_IF_OPLOCKED: u32 = 0x00000100;
pub const FILE_NO_EA_KNOWLEDGE: u32 = 0x00000200;
pub const FILE_OPEN_REMOTE_INSTANCE: u32 = 0x00000400;
pub const FILE_RANDOM_ACCESS: u32 = 0x00000800;
pub const FILE_DELETE_ON_CLOSE: u32 = 0x00001000;
pub const FILE_OPEN_BY_FILE_ID: u32 = 0x00002000;
pub const FILE_OPEN_FOR_BACKUP_INTENT: u32 = 0x00004000;
pub const FILE_NO_COMPRESSION: u32 = 0x00008000;
pub const FILE_OPEN_REQUIRING_OPLOCK: u32 = 0x00010000;
pub const FILE_DISALLOW_EXCLUSIVE: u32 = 0x00020000;
pub const FILE_SESSION_AWARE: u32 = 0x00040000;
pub const FILE_RESERVE_OPFILTER: u32 = 0x00100000;
pub const FILE_OPEN_REPARSE_POINT: u32 = 0x00200000;
pub const FILE_OPEN_NO_RECALL: u32 = 0x00400000;
pub const FILE_OPEN_FOR_FREE_SPACE_QUERY: u32 = 0x00800000;
pub const TREE_CONNECT_WRITE_THROUGH: u32 = 0x00000002;
pub const TREE_CONNECT_NO_CLIENT_BUFFERING: u32 = 0x00000008;

pub const FILE_CONTAINS_EXTENDED_CREATE_INFORMATION: u32 = 0x10000000;
pub const FILE_VALID_EXTENDED_OPTION_FLAGS: u32 = 0x10000000;

#[repr(C)]
pub struct EXTENDED_CREATE_DUAL_OPLOCK_KEYS {
    pub ParentOplockKey: GUID,
    pub TargetOplockKey: GUID,
}

pub type PEXTENDED_CREATE_DUAL_OPLOCK_KEYS = *mut EXTENDED_CREATE_DUAL_OPLOCK_KEYS;

#[repr(C)]
pub struct EXTENDED_CREATE_INFORMATION {
    pub ExtendedCreateFlags: LONGLONG,
    pub EaBuffer: PVOID,
    pub EaLength: ULONG,
}

pub type PEXTENDED_CREATE_INFORMATION = *mut EXTENDED_CREATE_INFORMATION;

#[repr(C)]
pub struct EXTENDED_CREATE_INFORMATION_32 {
    pub ExtendedCreateFlags: LONGLONG,
    pub EaBuffer: *mut core::ffi::c_void,
    pub EaLength: ULONG,
}

pub type PEXTENDED_CREATE_INFORMATION_32 = *mut EXTENDED_CREATE_INFORMATION_32;

pub const EX_CREATE_FLAG_FILE_SOURCE_OPEN_FOR_COPY: u32 = 0x00000001;
pub const EX_CREATE_FLAG_FILE_DEST_OPEN_FOR_COPY: u32 = 0x00000002;

pub const FILE_VALID_OPTION_FLAGS: u32 = 0x00ffffff;
pub const FILE_VALID_PIPE_OPTION_FLAGS: u32 = 0x00000032;
pub const FILE_VALID_MAILSLOT_OPTION_FLAGS: u32 = 0x00000032;
pub const FILE_VALID_SET_FLAGS: u32 = 0x00000036;
pub const FILE_COPY_STRUCTURED_STORAGE: u32 = 0x00000041;
pub const FILE_STRUCTURED_STORAGE: u32 = 0x00000441;

pub const FILE_SUPERSEDED: u32 = 0x00000000;
pub const FILE_OPENED: u32 = 0x00000001;
pub const FILE_CREATED: u32 = 0x00000002;
pub const FILE_OVERWRITTEN: u32 = 0x00000003;
pub const FILE_EXISTS: u32 = 0x00000004;
pub const FILE_DOES_NOT_EXIST: u32 = 0x00000005;

pub const FILE_WRITE_TO_END_OF_FILE: u32 = 0xffffffff;
pub const FILE_USE_FILE_POINTER_POSITION: u32 = 0xfffffffe;

pub const FILE_BYTE_ALIGNMENT: u32 = 0x00000000;
pub const FILE_WORD_ALIGNMENT: u32 = 0x00000001;
pub const FILE_LONG_ALIGNMENT: u32 = 0x00000003;
pub const FILE_QUAD_ALIGNMENT: u32 = 0x00000007;
pub const FILE_OCTA_ALIGNMENT: u32 = 0x0000000f;
pub const FILE_32_BYTE_ALIGNMENT: u32 = 0x0000001f;
pub const FILE_64_BYTE_ALIGNMENT: u32 = 0x0000003f;
pub const FILE_128_BYTE_ALIGNMENT: u32 = 0x0000007f;
pub const FILE_256_BYTE_ALIGNMENT: u32 = 0x000000ff;
pub const FILE_512_BYTE_ALIGNMENT: u32 = 0x000001ff;

pub const DOS_MAX_COMPONENT_LENGTH: i32 = 255;
pub const DOS_MAX_PATH_LENGTH: i32 = DOS_MAX_COMPONENT_LENGTH + 5;
pub const MAXIMUM_FILENAME_LENGTH: i32 = 256;

pub const FILE_NEED_EA: u32 = 0x00000080;

pub const FILE_EA_TYPE_BINARY: u16 = 0xfffe;
pub const FILE_EA_TYPE_ASCII: u16 = 0xfffd;
pub const FILE_EA_TYPE_BITMAP: u16 = 0xfffb;
pub const FILE_EA_TYPE_METAFILE: u16 = 0xfffa;
pub const FILE_EA_TYPE_ICON: u16 = 0xfff9;
pub const FILE_EA_TYPE_EA: u16 = 0xffee;
pub const FILE_EA_TYPE_MVMT: u16 = 0xffdf;
pub const FILE_EA_TYPE_MVST: u16 = 0xffde;
pub const FILE_EA_TYPE_ASN1: u16 = 0xffdd;
pub const FILE_EA_TYPE_FAMILY_IDS: u16 = 0xff01;

pub const FILE_REMOVABLE_MEDIA: u32 = 0x00000001;
pub const FILE_READ_ONLY_DEVICE: u32 = 0x00000002;
pub const FILE_FLOPPY_DISKETTE: u32 = 0x00000004;
pub const FILE_WRITE_ONCE_MEDIA: u32 = 0x00000008;
pub const FILE_REMOTE_DEVICE: u32 = 0x00000010;
pub const FILE_DEVICE_IS_MOUNTED: u32 = 0x00000020;
pub const FILE_VIRTUAL_VOLUME: u32 = 0x00000040;
pub const FILE_AUTOGENERATED_DEVICE_NAME: u32 = 0x00000080;
pub const FILE_DEVICE_SECURE_OPEN: u32 = 0x00000100;
pub const FILE_CHARACTERISTIC_PNP_DEVICE: u32 = 0x00000800;
pub const FILE_CHARACTERISTIC_TS_DEVICE: u32 = 0x00001000;
pub const FILE_CHARACTERISTIC_WEBDAV_DEVICE: u32 = 0x00002000;
pub const FILE_CHARACTERISTIC_CSV: u32 = 0x00010000;
pub const FILE_DEVICE_ALLOW_APPCONTAINER_TRAVERSAL: u32 = 0x00020000;
pub const FILE_PORTABLE_DEVICE: u32 = 0x00040000;
pub const FILE_REMOTE_DEVICE_VSMB: u32 = 0x00080000;
pub const FILE_DEVICE_REQUIRE_SECURITY_CHECK: u32 = 0x00100000;

pub const FILE_PIPE_BYTE_STREAM_TYPE: u32 = 0x00000000;
pub const FILE_PIPE_MESSAGE_TYPE: u32 = 0x00000001;
pub const FILE_PIPE_ACCEPT_REMOTE_CLIENTS: u32 = 0x00000000;
pub const FILE_PIPE_REJECT_REMOTE_CLIENTS: u32 = 0x00000002;
pub const FILE_PIPE_TYPE_VALID_MASK: u32 = 0x00000003;

pub const FILE_PIPE_QUEUE_OPERATION: u32 = 0x00000000;
pub const FILE_PIPE_COMPLETE_OPERATION: u32 = 0x00000001;

pub const FILE_PIPE_BYTE_STREAM_MODE: u32 = 0x00000000;
pub const FILE_PIPE_MESSAGE_MODE: u32 = 0x00000001;

pub const FILE_PIPE_INBOUND: u32 = 0x00000000;
pub const FILE_PIPE_OUTBOUND: u32 = 0x00000001;
pub const FILE_PIPE_FULL_DUPLEX: u32 = 0x00000002;

pub const FILE_PIPE_DISCONNECTED_STATE: u32 = 0x00000001;
pub const FILE_PIPE_LISTENING_STATE: u32 = 0x00000002;
pub const FILE_PIPE_CONNECTED_STATE: u32 = 0x00000003;
pub const FILE_PIPE_CLOSING_STATE: u32 = 0x00000004;

pub const FILE_PIPE_CLIENT_END: u32 = 0x00000000;
pub const FILE_PIPE_SERVER_END: u32 = 0x00000001;

pub const FILE_PIPE_UNLIMITED_INSTANCES: u32 = 0xffffffff;

pub const MAILSLOT_SIZE_AUTO: i32 = 0;

#[repr(C)]
pub union IO_STATUS_BLOCK_u {
    pub Status: NTSTATUS,
    pub Pointer: PVOID,
}

#[repr(C)]
pub struct IO_STATUS_BLOCK {
    pub u: IO_STATUS_BLOCK_u,
    pub Information: ULONG_PTR,
}

pub type PIO_STATUS_BLOCK = *mut IO_STATUS_BLOCK;

pub type PIO_APC_ROUTINE = Option<unsafe extern "system" fn(ApcContext: PVOID, IoStatusBlock: PIO_STATUS_BLOCK, Reserved: ULONG)>;

#[repr(u32)]
pub enum FILE_INFORMATION_CLASS {
    FileDirectoryInformation = 1,
    FileFullDirectoryInformation = 2,
    FileBothDirectoryInformation = 3,
    FileBasicInformation = 4,
    FileStandardInformation = 5,
    FileInternalInformation = 6,
    FileEaInformation = 7,
    FileAccessInformation = 8,
    FileNameInformation = 9,
    FileRenameInformation = 10,
    FileLinkInformation = 11,
    FileNamesInformation = 12,
    FileDispositionInformation = 13,
    FilePositionInformation = 14,
    FileFullEaInformation = 15,
    FileModeInformation = 16,
    FileAlignmentInformation = 17,
    FileAllInformation = 18,
    FileAllocationInformation = 19,
    FileEndOfFileInformation = 20,
    FileAlternateNameInformation = 21,
    FileStreamInformation = 22,
    FilePipeInformation = 23,
    FilePipeLocalInformation = 24,
    FilePipeRemoteInformation = 25,
    FileMailslotQueryInformation = 26,
    FileMailslotSetInformation = 27,
    FileCompressionInformation = 28,
    FileObjectIdInformation = 29,
    FileCompletionInformation = 30,
    FileMoveClusterInformation = 31,
    FileQuotaInformation = 32,
    FileReparsePointInformation = 33,
    FileNetworkOpenInformation = 34,
    FileAttributeTagInformation = 35,
    FileTrackingInformation = 36,
    FileIdBothDirectoryInformation = 37,
    FileIdFullDirectoryInformation = 38,
    FileValidDataLengthInformation = 39,
    FileShortNameInformation = 40,
    FileIoCompletionNotificationInformation = 41,
    FileIoStatusBlockRangeInformation = 42,
    FileIoPriorityHintInformation = 43,
    FileSfioReserveInformation = 44,
    FileSfioVolumeInformation = 45,
    FileHardLinkInformation = 46,
    FileProcessIdsUsingFileInformation = 47,
    FileNormalizedNameInformation = 48,
    FileNetworkPhysicalNameInformation = 49,
    FileIdGlobalTxDirectoryInformation = 50,
    FileIsRemoteDeviceInformation = 51,
    FileUnusedInformation = 52,
    FileNumaNodeInformation = 53,
    FileStandardLinkInformation = 54,
    FileRemoteProtocolInformation = 55,
    FileRenameInformationBypassAccessCheck = 56,
    FileLinkInformationBypassAccessCheck = 57,
    FileVolumeNameInformation = 58,
    FileIdInformation = 59,
    FileIdExtdDirectoryInformation = 60,
    FileReplaceCompletionInformation = 61,
    FileHardLinkFullIdInformation = 62,
    FileIdExtdBothDirectoryInformation = 63,
    FileDispositionInformationEx = 64,
    FileRenameInformationEx = 65,
    FileRenameInformationExBypassAccessCheck = 66,
    FileDesiredStorageClassInformation = 67,
    FileStatInformation = 68,
    FileMemoryPartitionInformation = 69,
    FileStatLxInformation = 70,
    FileCaseSensitiveInformation = 71,
    FileLinkInformationEx = 72,
    FileLinkInformationExBypassAccessCheck = 73,
    FileStorageReserveIdInformation = 74,
    FileCaseSensitiveInformationForceAccessCheck = 75,
    FileKnownFolderInformation = 76,
    FileStatBasicInformation = 77,
    FileId64ExtdDirectoryInformation = 78,
    FileId64ExtdBothDirectoryInformation = 79,
    FileIdAllExtdDirectoryInformation = 80,
    FileIdAllExtdBothDirectoryInformation = 81,
    FileStreamReservationInformation = 82,
    FileMupProviderInfo = 83,
    FileMaximumInformation = 84,
}

pub type PFILE_INFORMATION_CLASS = *mut FILE_INFORMATION_CLASS;

#[repr(C)]
pub struct FILE_BASIC_INFORMATION {
    pub CreationTime: LARGE_INTEGER,
    pub LastAccessTime: LARGE_INTEGER,
    pub LastWriteTime: LARGE_INTEGER,
    pub ChangeTime: LARGE_INTEGER,
    pub FileAttributes: ULONG,
}

pub type PFILE_BASIC_INFORMATION = *mut FILE_BASIC_INFORMATION;

#[repr(C)]
pub struct FILE_STANDARD_INFORMATION {
    pub AllocationSize: LARGE_INTEGER,
    pub EndOfFile: LARGE_INTEGER,
    pub NumberOfLinks: ULONG,
    pub DeletePending: BOOLEAN,
    pub Directory: BOOLEAN,
}

pub type PFILE_STANDARD_INFORMATION = *mut FILE_STANDARD_INFORMATION;

#[repr(C)]
pub struct FILE_STANDARD_INFORMATION_EX {
    pub AllocationSize: LARGE_INTEGER,
    pub EndOfFile: LARGE_INTEGER,
    pub NumberOfLinks: ULONG,
    pub DeletePending: BOOLEAN,
    pub Directory: BOOLEAN,
    pub AlternateStream: BOOLEAN,
    pub MetadataAttribute: BOOLEAN,
}

pub type PFILE_STANDARD_INFORMATION_EX = *mut FILE_STANDARD_INFORMATION_EX;

#[repr(C)]
pub struct FILE_INTERNAL_INFORMATION {
    pub IndexNumber: ULARGE_INTEGER,
}

pub type PFILE_INTERNAL_INFORMATION = *mut FILE_INTERNAL_INFORMATION;

#[repr(C)]
pub struct FILE_EA_INFORMATION {
    pub EaSize: ULONG,
}

pub type PFILE_EA_INFORMATION = *mut FILE_EA_INFORMATION;

#[repr(C)]
pub struct FILE_ACCESS_INFORMATION {
    pub AccessFlags: ACCESS_MASK,
}

pub type PFILE_ACCESS_INFORMATION = *mut FILE_ACCESS_INFORMATION;

#[repr(C)]
pub struct FILE_POSITION_INFORMATION {
    pub CurrentByteOffset: LARGE_INTEGER,
}

pub type PFILE_POSITION_INFORMATION = *mut FILE_POSITION_INFORMATION;

#[repr(C)]
pub struct FILE_MODE_INFORMATION {
    pub Mode: ULONG,
}

pub type PFILE_MODE_INFORMATION = *mut FILE_MODE_INFORMATION;

#[repr(C)]
pub struct FILE_ALIGNMENT_INFORMATION {
    pub AlignmentRequirement: ULONG,
}

pub type PFILE_ALIGNMENT_INFORMATION = *mut FILE_ALIGNMENT_INFORMATION;

#[repr(C)]
pub struct FILE_NAME_INFORMATION {
    pub FileNameLength: ULONG,
    pub FileName: [WCHAR; 1],
}

pub type PFILE_NAME_INFORMATION = *mut FILE_NAME_INFORMATION;

#[repr(C)]
pub struct FILE_ALL_INFORMATION {
    pub BasicInformation: FILE_BASIC_INFORMATION,
    pub StandardInformation: FILE_STANDARD_INFORMATION,
    pub InternalInformation: FILE_INTERNAL_INFORMATION,
    pub EaInformation: FILE_EA_INFORMATION,
    pub AccessInformation: FILE_ACCESS_INFORMATION,
    pub PositionInformation: FILE_POSITION_INFORMATION,
    pub ModeInformation: FILE_MODE_INFORMATION,
    pub AlignmentInformation: FILE_ALIGNMENT_INFORMATION,
    pub NameInformation: FILE_NAME_INFORMATION,
}

pub type PFILE_ALL_INFORMATION = *mut FILE_ALL_INFORMATION;

#[repr(C)]
pub struct FILE_NETWORK_OPEN_INFORMATION {
    pub CreationTime: LARGE_INTEGER,
    pub LastAccessTime: LARGE_INTEGER,
    pub LastWriteTime: LARGE_INTEGER,
    pub ChangeTime: LARGE_INTEGER,
    pub AllocationSize: LARGE_INTEGER,
    pub EndOfFile: LARGE_INTEGER,
    pub FileAttributes: ULONG,
}

pub type PFILE_NETWORK_OPEN_INFORMATION = *mut FILE_NETWORK_OPEN_INFORMATION;

#[repr(C)]
pub struct FILE_ATTRIBUTE_TAG_INFORMATION {
    pub FileAttributes: ULONG,
    pub ReparseTag: ULONG,
}

pub type PFILE_ATTRIBUTE_TAG_INFORMATION = *mut FILE_ATTRIBUTE_TAG_INFORMATION;

#[repr(C)]
pub struct FILE_ALLOCATION_INFORMATION {
    pub AllocationSize: LARGE_INTEGER,
}

pub type PFILE_ALLOCATION_INFORMATION = *mut FILE_ALLOCATION_INFORMATION;

#[repr(C)]
pub struct FILE_COMPRESSION_INFORMATION {
    pub CompressedFileSize: LARGE_INTEGER,
    pub CompressionFormat: USHORT,
    pub CompressionUnitShift: UCHAR,
    pub ChunkShift: UCHAR,
    pub ClusterShift: UCHAR,
    pub Reserved: [UCHAR; 3],
}

pub type PFILE_COMPRESSION_INFORMATION = *mut FILE_COMPRESSION_INFORMATION;

#[repr(C)]
pub struct FILE_DISPOSITION_INFORMATION {
    pub DeleteFile: BOOLEAN,
}

pub type PFILE_DISPOSITION_INFORMATION = *mut FILE_DISPOSITION_INFORMATION;

#[repr(C)]
pub struct FILE_END_OF_FILE_INFORMATION {
    pub EndOfFile: LARGE_INTEGER,
}

pub type PFILE_END_OF_FILE_INFORMATION = *mut FILE_END_OF_FILE_INFORMATION;

pub const FLAGS_END_OF_FILE_INFO_EX_EXTEND_PAGING: u32 = 0x00000001;
pub const FLAGS_END_OF_FILE_INFO_EX_NO_EXTRA_PAGING_EXTEND: u32 = 0x00000002;
pub const FLAGS_END_OF_FILE_INFO_EX_TIME_CONSTRAINED: u32 = 0x00000004;
pub const FLAGS_DELAY_REASONS_LOG_FILE_FULL: u32 = 0x00000001;
pub const FLAGS_DELAY_REASONS_BITMAP_SCANNED: u32 = 0x00000002;

#[repr(C)]
pub struct FILE_END_OF_FILE_INFORMATION_EX {
    pub EndOfFile: LARGE_INTEGER,
    pub PagingFileSizeInMM: LARGE_INTEGER,
    pub PagingFileMaxSize: LARGE_INTEGER,
    pub Flags: ULONG,
}

pub type PFILE_END_OF_FILE_INFORMATION_EX = *mut FILE_END_OF_FILE_INFORMATION_EX;

#[repr(C)]
pub struct FILE_VALID_DATA_LENGTH_INFORMATION {
    pub ValidDataLength: LARGE_INTEGER,
}

pub type PFILE_VALID_DATA_LENGTH_INFORMATION = *mut FILE_VALID_DATA_LENGTH_INFORMATION;

pub const FILE_LINK_REPLACE_IF_EXISTS: u32 = 0x00000001;
pub const FILE_LINK_POSIX_SEMANTICS: u32 = 0x00000002;
pub const FILE_LINK_SUPPRESS_STORAGE_RESERVE_INHERITANCE: u32 = 0x00000008;
pub const FILE_LINK_NO_INCREASE_AVAILABLE_SPACE: u32 = 0x00000010;
pub const FILE_LINK_NO_DECREASE_AVAILABLE_SPACE: u32 = 0x00000020;
pub const FILE_LINK_PRESERVE_AVAILABLE_SPACE: u32 = 0x00000030;
pub const FILE_LINK_IGNORE_READONLY_ATTRIBUTE: u32 = 0x00000040;
pub const FILE_LINK_FORCE_RESIZE_TARGET_SR: u32 = 0x00000080;
pub const FILE_LINK_FORCE_RESIZE_SOURCE_SR: u32 = 0x00000100;
pub const FILE_LINK_FORCE_RESIZE_SR: u32 = 0x00000180;

#[repr(C)]
pub struct FILE_LINK_INFORMATION {
    pub ReplaceIfExists: BOOLEAN,
    pub RootDirectory: HANDLE,
    pub FileNameLength: ULONG,
    pub FileName: [WCHAR; 1],
}

pub type PFILE_LINK_INFORMATION = *mut FILE_LINK_INFORMATION;

#[repr(C)]
pub struct FILE_LINK_INFORMATION_EX {
    pub Flags: ULONG,
    pub RootDirectory: HANDLE,
    pub FileNameLength: ULONG,
    pub FileName: [WCHAR; 1],
}

pub type PFILE_LINK_INFORMATION_EX = *mut FILE_LINK_INFORMATION_EX;

#[repr(C)]
pub struct FILE_MOVE_CLUSTER_INFORMATION {
    pub ClusterCount: ULONG,
    pub RootDirectory: HANDLE,
    pub FileNameLength: ULONG,
    pub FileName: [WCHAR; 1],
}

pub type PFILE_MOVE_CLUSTER_INFORMATION = *mut FILE_MOVE_CLUSTER_INFORMATION;

#[repr(C)]
pub struct FILE_RENAME_INFORMATION {
    pub ReplaceIfExists: BOOLEAN,
    pub RootDirectory: HANDLE,
    pub FileNameLength: ULONG,
    pub FileName: [WCHAR; 1],
}

pub type PFILE_RENAME_INFORMATION = *mut FILE_RENAME_INFORMATION;

pub const FILE_RENAME_REPLACE_IF_EXISTS: u32 = 0x00000001;
pub const FILE_RENAME_POSIX_SEMANTICS: u32 = 0x00000002;
pub const FILE_RENAME_SUPPRESS_PIN_STATE_INHERITANCE: u32 = 0x00000004;
pub const FILE_RENAME_SUPPRESS_STORAGE_RESERVE_INHERITANCE: u32 = 0x00000008;
pub const FILE_RENAME_NO_INCREASE_AVAILABLE_SPACE: u32 = 0x00000010;
pub const FILE_RENAME_NO_DECREASE_AVAILABLE_SPACE: u32 = 0x00000020;
pub const FILE_RENAME_PRESERVE_AVAILABLE_SPACE: u32 = 0x00000030;
pub const FILE_RENAME_IGNORE_READONLY_ATTRIBUTE: u32 = 0x00000040;
pub const FILE_RENAME_FORCE_RESIZE_TARGET_SR: u32 = 0x00000080;
pub const FILE_RENAME_FORCE_RESIZE_SOURCE_SR: u32 = 0x00000100;
pub const FILE_RENAME_FORCE_RESIZE_SR: u32 = 0x00000180;

#[repr(C)]
pub struct FILE_RENAME_INFORMATION_EX {
    pub Flags: ULONG,
    pub RootDirectory: HANDLE,
    pub FileNameLength: ULONG,
    pub FileName: [WCHAR; 1],
}

pub type PFILE_RENAME_INFORMATION_EX = *mut FILE_RENAME_INFORMATION_EX;

#[repr(C)]
pub struct FILE_STREAM_INFORMATION {
    pub NextEntryOffset: ULONG,
    pub StreamNameLength: ULONG,
    pub StreamSize: LARGE_INTEGER,
    pub StreamAllocationSize: LARGE_INTEGER,
    pub StreamName: [WCHAR; 1],
}

pub type PFILE_STREAM_INFORMATION = *mut FILE_STREAM_INFORMATION;

#[repr(C)]
pub struct FILE_TRACKING_INFORMATION {
    pub DestinationFile: HANDLE,
    pub ObjectInformationLength: ULONG,
    pub ObjectInformation: [CHAR; 1],
}

pub type PFILE_TRACKING_INFORMATION = *mut FILE_TRACKING_INFORMATION;

#[repr(C)]
pub struct FILE_COMPLETION_INFORMATION {
    pub Port: HANDLE,
    pub Key: PVOID,
}

pub type PFILE_COMPLETION_INFORMATION = *mut FILE_COMPLETION_INFORMATION;

#[repr(C)]
pub struct FILE_PIPE_INFORMATION {
    pub ReadMode: ULONG,
    pub CompletionMode: ULONG,
}

pub type PFILE_PIPE_INFORMATION = *mut FILE_PIPE_INFORMATION;

#[repr(C)]
pub struct FILE_PIPE_LOCAL_INFORMATION {
    pub NamedPipeType: ULONG,
    pub NamedPipeConfiguration: ULONG,
    pub MaximumInstances: ULONG,
    pub CurrentInstances: ULONG,
    pub InboundQuota: ULONG,
    pub ReadDataAvailable: ULONG,
    pub OutboundQuota: ULONG,
    pub WriteQuotaAvailable: ULONG,
    pub NamedPipeState: ULONG,
    pub NamedPipeEnd: ULONG,
}

pub type PFILE_PIPE_LOCAL_INFORMATION = *mut FILE_PIPE_LOCAL_INFORMATION;

#[repr(C)]
pub struct FILE_PIPE_REMOTE_INFORMATION {
    pub CollectDataTime: LARGE_INTEGER,
    pub MaximumCollectionCount: ULONG,
}

pub type PFILE_PIPE_REMOTE_INFORMATION = *mut FILE_PIPE_REMOTE_INFORMATION;

#[repr(C)]
pub struct FILE_MAILSLOT_QUERY_INFORMATION {
    pub MaximumMessageSize: ULONG,
    pub MailslotQuota: ULONG,
    pub NextMessageSize: ULONG,
    pub MessagesAvailable: ULONG,
    pub ReadTimeout: LARGE_INTEGER,
}

pub type PFILE_MAILSLOT_QUERY_INFORMATION = *mut FILE_MAILSLOT_QUERY_INFORMATION;

#[repr(C)]
pub struct FILE_MAILSLOT_SET_INFORMATION {
    pub ReadTimeout: PLARGE_INTEGER,
}

pub type PFILE_MAILSLOT_SET_INFORMATION = *mut FILE_MAILSLOT_SET_INFORMATION;

#[repr(C)]
pub struct FILE_REPARSE_POINT_INFORMATION {
    pub FileReference: LONGLONG,
    pub Tag: ULONG,
}

pub type PFILE_REPARSE_POINT_INFORMATION = *mut FILE_REPARSE_POINT_INFORMATION;

#[repr(C)]
pub struct FILE_LINK_ENTRY_INFORMATION {
    pub NextEntryOffset: ULONG,
    pub ParentFileId: LONGLONG,
    pub FileNameLength: ULONG,
    pub FileName: [WCHAR; 1],
}

pub type PFILE_LINK_ENTRY_INFORMATION = *mut FILE_LINK_ENTRY_INFORMATION;

#[repr(C)]
pub struct FILE_LINKS_INFORMATION {
    pub BytesNeeded: ULONG,
    pub EntriesReturned: ULONG,
    pub Entry: FILE_LINK_ENTRY_INFORMATION,
}

pub type PFILE_LINKS_INFORMATION = *mut FILE_LINKS_INFORMATION;

#[repr(C)]
pub struct FILE_NETWORK_PHYSICAL_NAME_INFORMATION {
    pub FileNameLength: ULONG,
    pub FileName: [WCHAR; 1],
}

pub type PFILE_NETWORK_PHYSICAL_NAME_INFORMATION = *mut FILE_NETWORK_PHYSICAL_NAME_INFORMATION;

#[repr(C)]
pub struct FILE_STANDARD_LINK_INFORMATION {
    pub NumberOfAccessibleLinks: ULONG,
    pub TotalNumberOfLinks: ULONG,
    pub DeletePending: BOOLEAN,
    pub Directory: BOOLEAN,
}

pub type PFILE_STANDARD_LINK_INFORMATION = *mut FILE_STANDARD_LINK_INFORMATION;

#[repr(C)]
pub struct FILE_SFIO_RESERVE_INFORMATION {
    pub RequestsPerPeriod: ULONG,
    pub Period: ULONG,
    pub RetryFailures: BOOLEAN,
    pub Discardable: BOOLEAN,
    pub RequestSize: ULONG,
    pub NumOutstandingRequests: ULONG,
}

pub type PFILE_SFIO_RESERVE_INFORMATION = *mut FILE_SFIO_RESERVE_INFORMATION;

#[repr(C)]
pub struct FILE_SFIO_VOLUME_INFORMATION {
    pub MaximumRequestsPerPeriod: ULONG,
    pub MinimumPeriod: ULONG,
    pub MinimumTransferSize: ULONG,
}

pub type PFILE_SFIO_VOLUME_INFORMATION = *mut FILE_SFIO_VOLUME_INFORMATION;

#[repr(u32)]
pub enum IO_PRIORITY_HINT {
    IoPriorityVeryLow = 0,
    IoPriorityLow = 1,
    IoPriorityNormal = 2,
    IoPriorityHigh = 3,
    IoPriorityCritical = 4,
    MaxIoPriorityTypes = 5,
}

#[repr(C, align(8))]
pub struct FILE_IO_PRIORITY_HINT_INFORMATION {
    pub PriorityHint: IO_PRIORITY_HINT,
}

pub type PFILE_IO_PRIORITY_HINT_INFORMATION = *mut FILE_IO_PRIORITY_HINT_INFORMATION;

#[repr(C)]
pub struct FILE_IO_PRIORITY_HINT_INFORMATION_EX {
    pub PriorityHint: IO_PRIORITY_HINT,
    pub BoostOutstanding: BOOLEAN,
}

pub type PFILE_IO_PRIORITY_HINT_INFORMATION_EX = *mut FILE_IO_PRIORITY_HINT_INFORMATION_EX;

pub const FILE_SKIP_COMPLETION_PORT_ON_SUCCESS: u32 = 0x1;
pub const FILE_SKIP_SET_EVENT_ON_HANDLE: u32 = 0x2;
pub const FILE_SKIP_SET_USER_EVENT_ON_FAST_IO: u32 = 0x4;

#[repr(C)]
pub struct FILE_IO_COMPLETION_NOTIFICATION_INFORMATION {
    pub Flags: ULONG,
}

pub type PFILE_IO_COMPLETION_NOTIFICATION_INFORMATION = *mut FILE_IO_COMPLETION_NOTIFICATION_INFORMATION;

#[repr(C)]
pub struct FILE_PROCESS_IDS_USING_FILE_INFORMATION {
    pub NumberOfProcessIdsInList: ULONG,
    pub ProcessIdList: [HANDLE; 1],
}

pub type PFILE_PROCESS_IDS_USING_FILE_INFORMATION = *mut FILE_PROCESS_IDS_USING_FILE_INFORMATION;

#[repr(C)]
pub struct FILE_IS_REMOTE_DEVICE_INFORMATION {
    pub IsRemote: BOOLEAN,
}

pub type PFILE_IS_REMOTE_DEVICE_INFORMATION = *mut FILE_IS_REMOTE_DEVICE_INFORMATION;

#[repr(C)]
pub struct FILE_NUMA_NODE_INFORMATION {
    pub NodeNumber: USHORT,
}

pub type PFILE_NUMA_NODE_INFORMATION = *mut FILE_NUMA_NODE_INFORMATION;

#[repr(C)]
pub struct FILE_IOSTATUSBLOCK_RANGE_INFORMATION {
    pub IoStatusBlockRange: PUCHAR,
    pub Length: ULONG,
}

pub type PFILE_IOSTATUSBLOCK_RANGE_INFORMATION = *mut FILE_IOSTATUSBLOCK_RANGE_INFORMATION;

#[repr(C)]
pub struct FILE_REMOTE_PROTOCOL_INFORMATION {
    pub StructureVersion: USHORT,
    pub StructureSize: USHORT,
    pub Protocol: ULONG,
    pub ProtocolMajorVersion: USHORT,
    pub ProtocolMinorVersion: USHORT,
    pub ProtocolRevision: USHORT,
    pub Reserved: USHORT,
    pub Flags: ULONG,
    pub GenericReserved: [ULONG; 8],
    pub ProtocolSpecific: FILE_REMOTE_PROTOCOL_INFORMATION_SPECIFIC,
}

#[repr(C)]
pub union FILE_REMOTE_PROTOCOL_INFORMATION_SPECIFIC {
    pub Smb2: FILE_REMOTE_PROTOCOL_INFORMATION_SMB2,
    pub Reserved: [ULONG; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_REMOTE_PROTOCOL_INFORMATION_SMB2 {
    pub Server: FILE_REMOTE_PROTOCOL_INFORMATION_SMB2_SERVER,
    pub Share: FILE_REMOTE_PROTOCOL_INFORMATION_SMB2_SHARE,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_REMOTE_PROTOCOL_INFORMATION_SMB2_SERVER {
    pub Capabilities: ULONG,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_REMOTE_PROTOCOL_INFORMATION_SMB2_SHARE {
    pub Capabilities: ULONG,
    pub ShareFlags: ULONG,
    pub ShareType: UCHAR,
    pub Reserved0: [UCHAR; 3],
    pub Reserved1: ULONG,
}

pub type PFILE_REMOTE_PROTOCOL_INFORMATION = *mut FILE_REMOTE_PROTOCOL_INFORMATION;

pub const CHECKSUM_ENFORCEMENT_OFF: u32 = 0x00000001;

#[repr(C)]
pub struct FILE_INTEGRITY_STREAM_INFORMATION {
    pub ChecksumAlgorithm: USHORT,
    pub ChecksumChunkShift: UCHAR,
    pub ClusterShift: UCHAR,
    pub Flags: ULONG,
}

pub type PFILE_INTEGRITY_STREAM_INFORMATION = *mut FILE_INTEGRITY_STREAM_INFORMATION;

#[repr(C)]
pub struct FILE_VOLUME_NAME_INFORMATION {
    pub DeviceNameLength: ULONG,
    pub DeviceName: [WCHAR; 1],
}

pub type PFILE_VOLUME_NAME_INFORMATION = *mut FILE_VOLUME_NAME_INFORMATION;

pub const FILE_INVALID_FILE_ID: LONGLONG = -1;
pub const FILE_ID_IS_INVALID: fn(FID: LARGE_INTEGER) -> bool = |FID| unsafe { FID.QuadPart == FILE_INVALID_FILE_ID };

#[repr(C)]
pub struct FILE_ID_INFORMATION {
    pub VolumeSerialNumber: ULONGLONG,
    pub FileId: LARGE_INTEGER,
}

pub type PFILE_ID_INFORMATION = *mut FILE_ID_INFORMATION;

#[repr(C)]
pub struct FILE_ID_EXTD_DIR_INFORMATION {
    pub NextEntryOffset: ULONG,
    pub FileIndex: ULONG,
    pub CreationTime: LARGE_INTEGER,
    pub LastAccessTime: LARGE_INTEGER,
    pub LastWriteTime: LARGE_INTEGER,
    pub ChangeTime: LARGE_INTEGER,
    pub EndOfFile: LARGE_INTEGER,
    pub AllocationSize: LARGE_INTEGER,
    pub FileAttributes: ULONG,
    pub FileNameLength: ULONG,
    pub EaSize: ULONG,
    pub ReparsePointTag: ULONG,
    pub FileId: FILE_ID_128,
    pub FileName: [WCHAR; 1],
}

pub type PFILE_ID_EXTD_DIR_INFORMATION = *mut FILE_ID_EXTD_DIR_INFORMATION;

#[repr(C)]
pub struct FILE_LINK_ENTRY_FULL_ID_INFORMATION {
    pub NextEntryOffset: ULONG,
    pub ParentFileId: FILE_ID_128,
    pub FileNameLength: ULONG,
    pub FileName: [WCHAR; 1],
}

pub type PFILE_LINK_ENTRY_FULL_ID_INFORMATION = *mut FILE_LINK_ENTRY_FULL_ID_INFORMATION;

#[repr(C)]
pub struct FILE_LINKS_FULL_ID_INFORMATION {
    pub BytesNeeded: ULONG,
    pub EntriesReturned: ULONG,
    pub Entry: FILE_LINK_ENTRY_FULL_ID_INFORMATION,
}

pub type PFILE_LINKS_FULL_ID_INFORMATION = *mut FILE_LINKS_FULL_ID_INFORMATION;

#[repr(C)]
pub struct FILE_ID_EXTD_BOTH_DIR_INFORMATION {
    pub NextEntryOffset: ULONG,
    pub FileIndex: ULONG,
    pub CreationTime: LARGE_INTEGER,
    pub LastAccessTime: LARGE_INTEGER,
    pub LastWriteTime: LARGE_INTEGER,
    pub ChangeTime: LARGE_INTEGER,
    pub EndOfFile: LARGE_INTEGER,
    pub AllocationSize: LARGE_INTEGER,
    pub FileAttributes: ULONG,
    pub FileNameLength: ULONG,
    pub EaSize: ULONG,
    pub ReparsePointTag: ULONG,
    pub FileId: FILE_ID_128,
    pub ShortNameLength: CHAR,
    pub ShortName: [WCHAR; 12],
    pub FileName: [WCHAR; 1],
}

pub type PFILE_ID_EXTD_BOTH_DIR_INFORMATION = *mut FILE_ID_EXTD_BOTH_DIR_INFORMATION;

#[repr(C)]
pub struct FILE_ID_64_EXTD_DIR_INFORMATION {
    pub NextEntryOffset: ULONG,
    pub FileIndex: ULONG,
    pub CreationTime: LARGE_INTEGER,
    pub LastAccessTime: LARGE_INTEGER,
    pub LastWriteTime: LARGE_INTEGER,
    pub ChangeTime: LARGE_INTEGER,
    pub EndOfFile: LARGE_INTEGER,
    pub AllocationSize: LARGE_INTEGER,
    pub FileAttributes: ULONG,
    pub FileNameLength: ULONG,
    pub EaSize: ULONG,
    pub ReparsePointTag: ULONG,
    pub FileId: LARGE_INTEGER,
    pub FileName: [WCHAR; 1],
}

pub type PFILE_ID_64_EXTD_DIR_INFORMATION = *mut FILE_ID_64_EXTD_DIR_INFORMATION;

#[repr(C)]
pub struct FILE_ID_64_EXTD_BOTH_DIR_INFORMATION {
    pub NextEntryOffset: ULONG,
    pub FileIndex: ULONG,
    pub CreationTime: LARGE_INTEGER,
    pub LastAccessTime: LARGE_INTEGER,
    pub LastWriteTime: LARGE_INTEGER,
    pub ChangeTime: LARGE_INTEGER,
    pub EndOfFile: LARGE_INTEGER,
    pub AllocationSize: LARGE_INTEGER,
    pub FileAttributes: ULONG,
    pub FileNameLength: ULONG,
    pub EaSize: ULONG,
    pub ReparsePointTag: ULONG,
    pub FileId: LARGE_INTEGER,
    pub ShortNameLength: CHAR,
    pub ShortName: [WCHAR; 12],
    pub FileName: [WCHAR; 1],
}

pub type PFILE_ID_64_EXTD_BOTH_DIR_INFORMATION = *mut FILE_ID_64_EXTD_BOTH_DIR_INFORMATION;

#[repr(C)]
pub struct FILE_ID_ALL_EXTD_DIR_INFORMATION {
    pub NextEntryOffset: ULONG,
    pub FileIndex: ULONG,
    pub CreationTime: LARGE_INTEGER,
    pub LastAccessTime: LARGE_INTEGER,
    pub LastWriteTime: LARGE_INTEGER,
    pub ChangeTime: LARGE_INTEGER,
    pub EndOfFile: LARGE_INTEGER,
    pub AllocationSize: LARGE_INTEGER,
    pub FileAttributes: ULONG,
    pub FileNameLength: ULONG,
    pub EaSize: ULONG,
    pub ReparsePointTag: ULONG,
    pub FileId: LARGE_INTEGER,
    pub FileId128: FILE_ID_128,
    pub FileName: [WCHAR; 1],
}

pub type PFILE_ID_ALL_EXTD_DIR_INFORMATION = *mut FILE_ID_ALL_EXTD_DIR_INFORMATION;

#[repr(C)]
pub struct FILE_ID_ALL_EXTD_BOTH_DIR_INFORMATION {
    pub NextEntryOffset: ULONG,
    pub FileIndex: ULONG,
    pub CreationTime: LARGE_INTEGER,
    pub LastAccessTime: LARGE_INTEGER,
    pub LastWriteTime: LARGE_INTEGER,
    pub ChangeTime: LARGE_INTEGER,
    pub EndOfFile: LARGE_INTEGER,
    pub AllocationSize: LARGE_INTEGER,
    pub FileAttributes: ULONG,
    pub FileNameLength: ULONG,
    pub EaSize: ULONG,
    pub ReparsePointTag: ULONG,
    pub FileId: LARGE_INTEGER,
    pub FileId128: FILE_ID_128,
    pub ShortNameLength: CHAR,
    pub ShortName: [WCHAR; 12],
    pub FileName: [WCHAR; 1],
}

pub type PFILE_ID_ALL_EXTD_BOTH_DIR_INFORMATION = *mut FILE_ID_ALL_EXTD_BOTH_DIR_INFORMATION;

#[repr(C)]
pub struct FILE_STAT_INFORMATION {
    pub FileId: LARGE_INTEGER,
    pub CreationTime: LARGE_INTEGER,
    pub LastAccessTime: LARGE_INTEGER,
    pub LastWriteTime: LARGE_INTEGER,
    pub ChangeTime: LARGE_INTEGER,
    pub AllocationSize: LARGE_INTEGER,
    pub EndOfFile: LARGE_INTEGER,
    pub FileAttributes: ULONG,
    pub ReparseTag: ULONG,
    pub NumberOfLinks: ULONG,
    pub EffectiveAccess: ACCESS_MASK,
}

pub type PFILE_STAT_INFORMATION = *mut FILE_STAT_INFORMATION;

#[repr(C)]
pub struct FILE_STAT_BASIC_INFORMATION {
    pub FileId: LARGE_INTEGER,
    pub CreationTime: LARGE_INTEGER,
    pub LastAccessTime: LARGE_INTEGER,
    pub LastWriteTime: LARGE_INTEGER,
    pub ChangeTime: LARGE_INTEGER,
    pub AllocationSize: LARGE_INTEGER,
    pub EndOfFile: LARGE_INTEGER,
    pub FileAttributes: ULONG,
    pub ReparseTag: ULONG,
    pub NumberOfLinks: ULONG,
    pub DeviceType: ULONG,
    pub DeviceCharacteristics: ULONG,
    pub Reserved: ULONG,
    pub VolumeSerialNumber: LARGE_INTEGER,
    pub FileId128: FILE_ID_128,
}

pub type PFILE_STAT_BASIC_INFORMATION = *mut FILE_STAT_BASIC_INFORMATION;

#[repr(C)]
pub struct FILE_MEMORY_PARTITION_INFORMATION {
    pub OwnerPartitionHandle: HANDLE,
    pub Flags: FILE_MEMORY_PARTITION_INFORMATION_FLAGS,
}

#[repr(C)]
pub union FILE_MEMORY_PARTITION_INFORMATION_FLAGS {
    pub AllFlags: ULONG,
    pub bits: FILE_MEMORY_PARTITION_INFORMATION_FLAGS_BITS,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_MEMORY_PARTITION_INFORMATION_FLAGS_BITS {
    pub NoCrossPartitionAccess: u32,
    pub Spare: [u32; 3],
}

pub type PFILE_MEMORY_PARTITION_INFORMATION = *mut FILE_MEMORY_PARTITION_INFORMATION;

pub const LX_FILE_METADATA_HAS_UID: u32 = 0x1;
pub const LX_FILE_METADATA_HAS_GID: u32 = 0x2;
pub const LX_FILE_METADATA_HAS_MODE: u32 = 0x4;
pub const LX_FILE_METADATA_HAS_DEVICE_ID: u32 = 0x8;
pub const LX_FILE_CASE_SENSITIVE_DIR: u32 = 0x10;

#[repr(C)]
pub struct FILE_STAT_LX_INFORMATION {
    pub FileId: FILE_INTERNAL_INFORMATION,
    pub CreationTime: LARGE_INTEGER,
    pub LastAccessTime: LARGE_INTEGER,
    pub LastWriteTime: LARGE_INTEGER,
    pub ChangeTime: LARGE_INTEGER,
    pub AllocationSize: LARGE_INTEGER,
    pub EndOfFile: LARGE_INTEGER,
    pub FileAttributes: ULONG,
    pub ReparseTag: ULONG,
    pub NumberOfLinks: ULONG,
    pub EffectiveAccess: ACCESS_MASK,
    pub LxFlags: ULONG,
    pub LxUid: ULONG,
    pub LxGid: ULONG,
    pub LxMode: ULONG,
    pub LxDeviceIdMajor: ULONG,
    pub LxDeviceIdMinor: ULONG,
}

pub type PFILE_STAT_LX_INFORMATION = *mut FILE_STAT_LX_INFORMATION;

#[repr(C)]
pub struct FILE_STORAGE_RESERVE_ID_INFORMATION {
    pub StorageReserveId: STORAGE_RESERVE_ID,
}

pub type PFILE_STORAGE_RESERVE_ID_INFORMATION = *mut FILE_STORAGE_RESERVE_ID_INFORMATION;

pub const FILE_CS_FLAG_CASE_SENSITIVE_DIR: u32 = 0x00000001;

#[repr(C)]
pub struct FILE_CASE_SENSITIVE_INFORMATION {
    pub Flags: ULONG,
}

pub type PFILE_CASE_SENSITIVE_INFORMATION = *mut FILE_CASE_SENSITIVE_INFORMATION;

#[repr(u32)]
pub enum FILE_KNOWN_FOLDER_TYPE {
    KnownFolderNone = 0,
    KnownFolderDesktop = 1,
    KnownFolderDocuments = 2,
    KnownFolderDownloads = 3,
    KnownFolderMusic = 4,
    KnownFolderPictures = 5,
    KnownFolderVideos = 6,
    KnownFolderOther = 7,
    KnownFolderMax = 8,
}

#[repr(C)]
pub struct FILE_KNOWN_FOLDER_INFORMATION {
    pub Type: FILE_KNOWN_FOLDER_TYPE,
}

pub type PFILE_KNOWN_FOLDER_INFORMATION = *mut FILE_KNOWN_FOLDER_INFORMATION;

#[repr(C)]
pub struct FILE_STREAM_RESERVATION_INFORMATION {
    pub TrackedReservation: ULONG_PTR,
    pub EnforcedReservation: ULONG_PTR,
}

pub type PFILE_STREAM_RESERVATION_INFORMATION = *mut FILE_STREAM_RESERVATION_INFORMATION;

#[repr(C)]
pub struct MUP_PROVIDER_INFORMATION {
    pub Level: ULONG,
    pub Buffer: PVOID,
    pub BufferSize: PULONG,
}

pub type PMUP_PROVIDER_INFORMATION = *mut MUP_PROVIDER_INFORMATION;

#[repr(C)]
pub struct FILE_INFORMATION_DEFINITION {
    pub Class: FILE_INFORMATION_CLASS,
    pub NextEntryOffset: ULONG,
    pub FileNameOffset: ULONG,
    pub FileNameLengthOffset: ULONG,
}

pub type PFILE_INFORMATION_DEFINITION = *mut FILE_INFORMATION_DEFINITION;

#[repr(C)]
pub struct FILE_DIRECTORY_INFORMATION {
    pub NextEntryOffset: ULONG,
    pub FileIndex: ULONG,
    pub CreationTime: LARGE_INTEGER,
    pub LastAccessTime: LARGE_INTEGER,
    pub LastWriteTime: LARGE_INTEGER,
    pub ChangeTime: LARGE_INTEGER,
    pub EndOfFile: LARGE_INTEGER,
    pub AllocationSize: LARGE_INTEGER,
    pub FileAttributes: ULONG,
    pub FileNameLength: ULONG,
    pub FileName: [WCHAR; 1],
}

pub type PFILE_DIRECTORY_INFORMATION = *mut FILE_DIRECTORY_INFORMATION;

#[repr(C)]
pub struct FILE_FULL_DIR_INFORMATION {
    pub NextEntryOffset: ULONG,
    pub FileIndex: ULONG,
    pub CreationTime: LARGE_INTEGER,
    pub LastAccessTime: LARGE_INTEGER,
    pub LastWriteTime: LARGE_INTEGER,
    pub ChangeTime: LARGE_INTEGER,
    pub EndOfFile: LARGE_INTEGER,
    pub AllocationSize: LARGE_INTEGER,
    pub FileAttributes: ULONG,
    pub FileNameLength: ULONG,
    pub EaSize: ULONG,
    pub FileName: [WCHAR; 1],
}

pub type PFILE_FULL_DIR_INFORMATION = *mut FILE_FULL_DIR_INFORMATION;

#[repr(C)]
pub struct FILE_ID_FULL_DIR_INFORMATION {
    pub NextEntryOffset: ULONG,
    pub FileIndex: ULONG,
    pub CreationTime: LARGE_INTEGER,
    pub LastAccessTime: LARGE_INTEGER,
    pub LastWriteTime: LARGE_INTEGER,
    pub ChangeTime: LARGE_INTEGER,
    pub EndOfFile: LARGE_INTEGER,
    pub AllocationSize: LARGE_INTEGER,
    pub FileAttributes: ULONG,
    pub FileNameLength: ULONG,
    pub EaSize: ULONG,
    pub FileId: LARGE_INTEGER,
    pub FileName: [WCHAR; 1],
}

pub type PFILE_ID_FULL_DIR_INFORMATION = *mut FILE_ID_FULL_DIR_INFORMATION;

#[repr(C)]
pub struct FILE_BOTH_DIR_INFORMATION {
    pub NextEntryOffset: ULONG,
    pub FileIndex: ULONG,
    pub CreationTime: LARGE_INTEGER,
    pub LastAccessTime: LARGE_INTEGER,
    pub LastWriteTime: LARGE_INTEGER,
    pub ChangeTime: LARGE_INTEGER,
    pub EndOfFile: LARGE_INTEGER,
    pub AllocationSize: LARGE_INTEGER,
    pub FileAttributes: ULONG,
    pub FileNameLength: ULONG,
    pub EaSize: ULONG,
    pub ShortNameLength: CHAR,
    pub ShortName: [WCHAR; 12],
    pub FileName: [WCHAR; 1],
}

pub type PFILE_BOTH_DIR_INFORMATION = *mut FILE_BOTH_DIR_INFORMATION;

#[repr(C)]
pub struct FILE_ID_BOTH_DIR_INFORMATION {
    pub NextEntryOffset: ULONG,
    pub FileIndex: ULONG,
    pub CreationTime: LARGE_INTEGER,
    pub LastAccessTime: LARGE_INTEGER,
    pub LastWriteTime: LARGE_INTEGER,
    pub ChangeTime: LARGE_INTEGER,
    pub EndOfFile: LARGE_INTEGER,
    pub AllocationSize: LARGE_INTEGER,
    pub FileAttributes: ULONG,
    pub FileNameLength: ULONG,
    pub EaSize: ULONG,
    pub ShortNameLength: CHAR,
    pub ShortName: [WCHAR; 12],
    pub FileId: LARGE_INTEGER,
    pub FileName: [WCHAR; 1],
}

pub type PFILE_ID_BOTH_DIR_INFORMATION = *mut FILE_ID_BOTH_DIR_INFORMATION;

#[repr(C)]
pub struct FILE_NAMES_INFORMATION {
    pub NextEntryOffset: ULONG,
    pub FileIndex: ULONG,
    pub FileNameLength: ULONG,
    pub FileName: [WCHAR; 1],
}

pub type PFILE_NAMES_INFORMATION = *mut FILE_NAMES_INFORMATION;

#[repr(C)]
pub struct FILE_ID_GLOBAL_TX_DIR_INFORMATION {
    pub NextEntryOffset: ULONG,
    pub FileIndex: ULONG,
    pub CreationTime: LARGE_INTEGER,
    pub LastAccessTime: LARGE_INTEGER,
    pub LastWriteTime: LARGE_INTEGER,
    pub ChangeTime: LARGE_INTEGER,
    pub EndOfFile: LARGE_INTEGER,
    pub AllocationSize: LARGE_INTEGER,
    pub FileAttributes: ULONG,
    pub FileNameLength: ULONG,
    pub FileId: LARGE_INTEGER,
    pub LockingTransactionId: GUID,
    pub TxInfoFlags: ULONG,
    pub FileName: [WCHAR; 1],
}

pub type PFILE_ID_GLOBAL_TX_DIR_INFORMATION = *mut FILE_ID_GLOBAL_TX_DIR_INFORMATION;

pub const FILE_ID_GLOBAL_TX_DIR_INFO_FLAG_WRITELOCKED: u32 = 0x00000001;
pub const FILE_ID_GLOBAL_TX_DIR_INFO_FLAG_VISIBLE_TO_TX: u32 = 0x00000002;
pub const FILE_ID_GLOBAL_TX_DIR_INFO_FLAG_VISIBLE_OUTSIDE_TX: u32 = 0x00000004;

#[repr(C)]
pub struct FILE_OBJECTID_INFORMATION {
    pub FileReference: ULONGLONG,
    pub ObjectId: [UCHAR; 16],
    pub ExtendedInfo: [UCHAR; 48],
}

pub type PFILE_OBJECTID_INFORMATION = *mut FILE_OBJECTID_INFORMATION;

#[repr(C)]
pub struct FILE_DIRECTORY_NEXT_INFORMATION {
    pub NextEntryOffset: ULONG,
}

pub type PFILE_DIRECTORY_NEXT_INFORMATION = *mut FILE_DIRECTORY_NEXT_INFORMATION;

#[repr(C)]
pub struct FILE_FULL_EA_INFORMATION {
    pub NextEntryOffset: ULONG,
    pub Flags: UCHAR,
    pub EaNameLength: UCHAR,
    pub EaValueLength: USHORT,
    pub EaName: [CHAR; 1],
}

pub type PFILE_FULL_EA_INFORMATION = *mut FILE_FULL_EA_INFORMATION;

#[repr(C)]
pub struct FILE_GET_EA_INFORMATION {
    pub NextEntryOffset: ULONG,
    pub EaNameLength: UCHAR,
    pub EaName: [CHAR; 1],
}

pub type PFILE_GET_EA_INFORMATION = *mut FILE_GET_EA_INFORMATION;

#[repr(C)]
pub struct FILE_GET_QUOTA_INFORMATION {
    pub NextEntryOffset: ULONG,
    pub SidLength: ULONG,
    pub Sid: SID,
}

pub type PFILE_GET_QUOTA_INFORMATION = *mut FILE_GET_QUOTA_INFORMATION;

#[repr(C)]
pub struct FILE_QUOTA_INFORMATION {
    pub NextEntryOffset: ULONG,
    pub SidLength: ULONG,
    pub ChangeTime: LARGE_INTEGER,
    pub QuotaUsed: LARGE_INTEGER,
    pub QuotaThreshold: LARGE_INTEGER,
    pub QuotaLimit: LARGE_INTEGER,
    pub Sid: SID,
}

pub type PFILE_QUOTA_INFORMATION = *mut FILE_QUOTA_INFORMATION;

#[repr(u32)]
pub enum FS_INFORMATION_CLASS {
    FileFsVolumeInformation = 1,
    FileFsLabelInformation = 2,
    FileFsSizeInformation = 3,
    FileFsDeviceInformation = 4,
    FileFsAttributeInformation = 5,
    FileFsControlInformation = 6,
    FileFsFullSizeInformation = 7,
    FileFsObjectIdInformation = 8,
    FileFsDriverPathInformation = 9,
    FileFsVolumeFlagsInformation = 10,
    FileFsSectorSizeInformation = 11,
    FileFsDataCopyInformation = 12,
    FileFsMetadataSizeInformation = 13,
    FileFsFullSizeInformationEx = 14,
    FileFsGuidInformation = 15,
}

pub type PFS_INFORMATION_CLASS = *mut FS_INFORMATION_CLASS;

#[repr(C)]
pub struct FILE_FS_VOLUME_INFORMATION {
    pub VolumeCreationTime: LARGE_INTEGER,
    pub VolumeSerialNumber: ULONG,
    pub VolumeLabelLength: ULONG,
    pub SupportsObjects: BOOLEAN,
    pub VolumeLabel: [WCHAR; 1],
}

pub type PFILE_FS_VOLUME_INFORMATION = *mut FILE_FS_VOLUME_INFORMATION;

#[repr(C)]
pub struct FILE_FS_LABEL_INFORMATION {
    pub VolumeLabelLength: ULONG,
    pub VolumeLabel: [WCHAR; 1],
}

pub type PFILE_FS_LABEL_INFORMATION = *mut FILE_FS_LABEL_INFORMATION;

#[repr(C)]
pub struct FILE_FS_SIZE_INFORMATION {
    pub TotalAllocationUnits: LARGE_INTEGER,
    pub AvailableAllocationUnits: LARGE_INTEGER,
    pub SectorsPerAllocationUnit: ULONG,
    pub BytesPerSector: ULONG,
}

pub type PFILE_FS_SIZE_INFORMATION = *mut FILE_FS_SIZE_INFORMATION;

pub const FILE_VC_QUOTA_NONE: u32 = 0x00000000;
pub const FILE_VC_QUOTA_TRACK: u32 = 0x00000001;
pub const FILE_VC_QUOTA_ENFORCE: u32 = 0x00000002;
pub const FILE_VC_QUOTA_MASK: u32 = 0x00000003;
pub const FILE_VC_CONTENT_INDEX_DISABLED: u32 = 0x00000008;
pub const FILE_VC_LOG_QUOTA_THRESHOLD: u32 = 0x00000010;
pub const FILE_VC_LOG_QUOTA_LIMIT: u32 = 0x00000020;
pub const FILE_VC_LOG_VOLUME_THRESHOLD: u32 = 0x00000040;
pub const FILE_VC_LOG_VOLUME_LIMIT: u32 = 0x00000080;
pub const FILE_VC_QUOTAS_INCOMPLETE: u32 = 0x00000100;
pub const FILE_VC_QUOTAS_REBUILDING: u32 = 0x00000200;
pub const FILE_VC_VALID_MASK: u32 = 0x000003ff;

#[repr(C)]
pub struct FILE_FS_CONTROL_INFORMATION {
    pub FreeSpaceStartFiltering: LARGE_INTEGER,
    pub FreeSpaceThreshold: LARGE_INTEGER,
    pub FreeSpaceStopFiltering: LARGE_INTEGER,
    pub DefaultQuotaThreshold: LARGE_INTEGER,
    pub DefaultQuotaLimit: LARGE_INTEGER,
    pub FileSystemControlFlags: ULONG,
}

pub type PFILE_FS_CONTROL_INFORMATION = *mut FILE_FS_CONTROL_INFORMATION;

#[repr(C)]
pub struct FILE_FS_FULL_SIZE_INFORMATION {
    pub TotalAllocationUnits: LARGE_INTEGER,
    pub CallerAvailableAllocationUnits: LARGE_INTEGER,
    pub ActualAvailableAllocationUnits: LARGE_INTEGER,
    pub SectorsPerAllocationUnit: ULONG,
    pub BytesPerSector: ULONG,
}

pub type PFILE_FS_FULL_SIZE_INFORMATION = *mut FILE_FS_FULL_SIZE_INFORMATION;

#[repr(C)]
pub struct FILE_FS_OBJECTID_INFORMATION {
    pub ObjectId: [UCHAR; 16],
    pub ExtendedInfo: [UCHAR; 48],
}

pub type PFILE_FS_OBJECTID_INFORMATION = *mut FILE_FS_OBJECTID_INFORMATION;

#[repr(C)]
pub struct FILE_FS_DEVICE_INFORMATION {
    pub DeviceType: DEVICE_TYPE,
    pub Characteristics: ULONG,
}

pub type PFILE_FS_DEVICE_INFORMATION = *mut FILE_FS_DEVICE_INFORMATION;

#[repr(C)]
pub struct FILE_FS_ATTRIBUTE_INFORMATION {
    pub FileSystemAttributes: ULONG,
    pub MaximumComponentNameLength: LONG,
    pub FileSystemNameLength: ULONG,
    pub FileSystemName: [WCHAR; 1],
}

pub type PFILE_FS_ATTRIBUTE_INFORMATION = *mut FILE_FS_ATTRIBUTE_INFORMATION;

#[repr(C)]
pub struct FILE_FS_DRIVER_PATH_INFORMATION {
    pub DriverInPath: BOOLEAN,
    pub DriverNameLength: ULONG,
    pub DriverName: [WCHAR; 1],
}

pub type PFILE_FS_DRIVER_PATH_INFORMATION = *mut FILE_FS_DRIVER_PATH_INFORMATION;

#[repr(C)]
pub struct FILE_FS_VOLUME_FLAGS_INFORMATION {
    pub Flags: ULONG,
}

pub type PFILE_FS_VOLUME_FLAGS_INFORMATION = *mut FILE_FS_VOLUME_FLAGS_INFORMATION;

pub const SSINFO_FLAGS_ALIGNED_DEVICE: u32 = 0x00000001;
pub const SSINFO_FLAGS_PARTITION_ALIGNED_ON_DEVICE: u32 = 0x00000002;
pub const SSINFO_FLAGS_NO_SEEK_PENALTY: u32 = 0x00000004;
pub const SSINFO_FLAGS_TRIM_ENABLED: u32 = 0x00000008;
pub const SSINFO_FLAGS_BYTE_ADDRESSABLE: u32 = 0x00000010;
pub const SSINFO_OFFSET_UNKNOWN: u32 = 0xffffffff;

#[repr(C)]
pub struct FILE_FS_SECTOR_SIZE_INFORMATION {
    pub LogicalBytesPerSector: ULONG,
    pub PhysicalBytesPerSectorForAtomicity: ULONG,
    pub PhysicalBytesPerSectorForPerformance: ULONG,
    pub FileSystemEffectivePhysicalBytesPerSectorForAtomicity: ULONG,
    pub Flags: ULONG,
    pub ByteOffsetForSectorAlignment: ULONG,
    pub ByteOffsetForPartitionAlignment: ULONG,
}

pub type PFILE_FS_SECTOR_SIZE_INFORMATION = *mut FILE_FS_SECTOR_SIZE_INFORMATION;

#[repr(C)]
pub struct FILE_FS_DATA_COPY_INFORMATION {
    pub NumberOfCopies: ULONG,
}

pub type PFILE_FS_DATA_COPY_INFORMATION = *mut FILE_FS_DATA_COPY_INFORMATION;

#[repr(C)]
pub struct FILE_FS_METADATA_SIZE_INFORMATION {
    pub TotalMetadataAllocationUnits: LARGE_INTEGER,
    pub SectorsPerAllocationUnit: ULONG,
    pub BytesPerSector: ULONG,
}

pub type PFILE_FS_METADATA_SIZE_INFORMATION = *mut FILE_FS_METADATA_SIZE_INFORMATION;

#[repr(C)]
pub struct FILE_FS_FULL_SIZE_INFORMATION_EX {
    pub ActualTotalAllocationUnits: ULONGLONG,
    pub ActualAvailableAllocationUnits: ULONGLONG,
    pub ActualPoolUnavailableAllocationUnits: ULONGLONG,
    pub CallerTotalAllocationUnits: ULONGLONG,
    pub CallerAvailableAllocationUnits: ULONGLONG,
    pub CallerPoolUnavailableAllocationUnits: ULONGLONG,
    pub UsedAllocationUnits: ULONGLONG,
    pub TotalReservedAllocationUnits: ULONGLONG,
    pub VolumeStorageReserveAllocationUnits: ULONGLONG,
    pub AvailableCommittedAllocationUnits: ULONGLONG,
    pub PoolAvailableAllocationUnits: ULONGLONG,
    pub SectorsPerAllocationUnit: ULONG,
    pub BytesPerSector: ULONG,
}

pub type PFILE_FS_FULL_SIZE_INFORMATION_EX = *mut FILE_FS_FULL_SIZE_INFORMATION_EX;

#[repr(C)]
pub struct FILE_FS_GUID_INFORMATION {
    pub FsGuid: GUID,
}

pub type PFILE_FS_GUID_INFORMATION = *mut FILE_FS_GUID_INFORMATION;

pub type DEVICE_TYPE = ULONG;

#[repr(C)]
pub struct FILE_ID_128 {
    pub Identifier: [UCHAR; 16],
}

pub type STORAGE_RESERVE_ID = ULONG;

unsafe extern "system" {
    pub fn NtCreateFile(
        FileHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
        IoStatusBlock: PIO_STATUS_BLOCK,
        AllocationSize: PLARGE_INTEGER,
        FileAttributes: ULONG,
        ShareAccess: ULONG,
        CreateDisposition: ULONG,
        CreateOptions: ULONG,
        EaBuffer: PVOID,
        EaLength: ULONG,
    ) -> NTSTATUS;

    pub fn NtCreateNamedPipeFile(
        FileHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
        IoStatusBlock: PIO_STATUS_BLOCK,
        ShareAccess: ULONG,
        CreateDisposition: ULONG,
        CreateOptions: ULONG,
        NamedPipeType: ULONG,
        ReadMode: ULONG,
        CompletionMode: ULONG,
        MaximumInstances: ULONG,
        InboundQuota: ULONG,
        OutboundQuota: ULONG,
        DefaultTimeout: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn NtCreateMailslotFile(
        FileHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
        IoStatusBlock: PIO_STATUS_BLOCK,
        CreateOptions: ULONG,
        MailslotQuota: ULONG,
        MaximumMessageSize: ULONG,
        ReadTimeout: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn NtOpenFile(
        FileHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
        IoStatusBlock: PIO_STATUS_BLOCK,
        ShareAccess: ULONG,
        OpenOptions: ULONG,
    ) -> NTSTATUS;

    pub fn NtDeleteFile(
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn NtFlushBuffersFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
    ) -> NTSTATUS;

    pub fn NtFlushBuffersFileEx(
        FileHandle: HANDLE,
        Flags: ULONG,
        Parameters: PVOID,
        ParametersSize: ULONG,
        IoStatusBlock: PIO_STATUS_BLOCK,
    ) -> NTSTATUS;

    pub fn NtQueryInformationFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        FileInformation: PVOID,
        Length: ULONG,
        FileInformationClass: FILE_INFORMATION_CLASS,
    ) -> NTSTATUS;

    pub fn NtQueryInformationByName(
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
        IoStatusBlock: PIO_STATUS_BLOCK,
        FileInformation: PVOID,
        Length: ULONG,
        FileInformationClass: FILE_INFORMATION_CLASS,
    ) -> NTSTATUS;

    pub fn NtSetInformationFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        FileInformation: PVOID,
        Length: ULONG,
        FileInformationClass: FILE_INFORMATION_CLASS,
    ) -> NTSTATUS;

    pub fn NtQueryDirectoryFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        FileInformation: PVOID,
        Length: ULONG,
        FileInformationClass: FILE_INFORMATION_CLASS,
        ReturnSingleEntry: BOOLEAN,
        FileName: PCUNICODE_STRING,
        RestartScan: BOOLEAN,
    ) -> NTSTATUS;

    pub fn NtQueryDirectoryFileEx(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        FileInformation: PVOID,
        Length: ULONG,
        FileInformationClass: FILE_INFORMATION_CLASS,
        QueryFlags: ULONG,
        FileName: PCUNICODE_STRING,
    ) -> NTSTATUS;

    pub fn NtQueryEaFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        Buffer: PVOID,
        Length: ULONG,
        ReturnSingleEntry: BOOLEAN,
        EaList: PVOID,
        EaListLength: ULONG,
        EaIndex: PULONG,
        RestartScan: BOOLEAN,
    ) -> NTSTATUS;

    pub fn NtSetEaFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        Buffer: PVOID,
        Length: ULONG,
    ) -> NTSTATUS;

    pub fn NtQueryQuotaInformationFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        Buffer: PVOID,
        Length: ULONG,
        ReturnSingleEntry: BOOLEAN,
        SidList: PVOID,
        SidListLength: ULONG,
        StartSid: PSID,
        RestartScan: BOOLEAN,
    ) -> NTSTATUS;

    pub fn NtSetQuotaInformationFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        Buffer: PVOID,
        Length: ULONG,
    ) -> NTSTATUS;

    pub fn NtQueryVolumeInformationFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        FsInformation: PVOID,
        Length: ULONG,
        FsInformationClass: FS_INFORMATION_CLASS,
    ) -> NTSTATUS;

    pub fn NtSetVolumeInformationFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        FsInformation: PVOID,
        Length: ULONG,
        FsInformationClass: FS_INFORMATION_CLASS,
    ) -> NTSTATUS;

    pub fn NtCancelIoFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
    ) -> NTSTATUS;

    pub fn NtCancelIoFileEx(
        FileHandle: HANDLE,
        IoRequestToCancel: PIO_STATUS_BLOCK,
        IoStatusBlock: PIO_STATUS_BLOCK,
    ) -> NTSTATUS;

    pub fn NtCancelSynchronousIoFile(
        ThreadHandle: HANDLE,
        IoRequestToCancel: PIO_STATUS_BLOCK,
        IoStatusBlock: PIO_STATUS_BLOCK,
    ) -> NTSTATUS;

    pub fn NtDeviceIoControlFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        IoControlCode: ULONG,
        InputBuffer: PVOID,
        InputBufferLength: ULONG,
        OutputBuffer: PVOID,
        OutputBufferLength: ULONG,
    ) -> NTSTATUS;

    pub fn NtFsControlFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        FsControlCode: ULONG,
        InputBuffer: PVOID,
        InputBufferLength: ULONG,
        OutputBuffer: PVOID,
        OutputBufferLength: ULONG,
    ) -> NTSTATUS;

    pub fn NtReadFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        Buffer: PVOID,
        Length: ULONG,
        ByteOffset: PLARGE_INTEGER,
        Key: PULONG,
    ) -> NTSTATUS;

    pub fn NtWriteFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        Buffer: PVOID,
        Length: ULONG,
        ByteOffset: PLARGE_INTEGER,
        Key: PULONG,
    ) -> NTSTATUS;

    pub fn NtReadFileScatter(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        SegmentArray: PFILE_SEGMENT_ELEMENT,
        Length: ULONG,
        ByteOffset: PLARGE_INTEGER,
        Key: PULONG,
    ) -> NTSTATUS;

    pub fn NtWriteFileGather(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        SegmentArray: PFILE_SEGMENT_ELEMENT,
        Length: ULONG,
        ByteOffset: PLARGE_INTEGER,
        Key: PULONG,
    ) -> NTSTATUS;

    pub fn NtLockFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        ByteOffset: PLARGE_INTEGER,
        Length: PLARGE_INTEGER,
        Key: ULONG,
        FailImmediately: BOOLEAN,
        ExclusiveLock: BOOLEAN,
    ) -> NTSTATUS;

    pub fn NtUnlockFile(
        FileHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        ByteOffset: PLARGE_INTEGER,
        Length: PLARGE_INTEGER,
        Key: ULONG,
    ) -> NTSTATUS;

    pub fn NtQueryAttributesFile(
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
        FileInformation: PFILE_BASIC_INFORMATION,
    ) -> NTSTATUS;

    pub fn NtQueryFullAttributesFile(
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
        FileInformation: PFILE_NETWORK_OPEN_INFORMATION,
    ) -> NTSTATUS;

    pub fn NtNotifyChangeDirectoryFile(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        Buffer: PVOID,
        Length: ULONG,
        CompletionFilter: ULONG,
        WatchTree: BOOLEAN,
    ) -> NTSTATUS;

    pub fn NtNotifyChangeDirectoryFileEx(
        FileHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        Buffer: PVOID,
        Length: ULONG,
        CompletionFilter: ULONG,
        WatchTree: BOOLEAN,
        DirectoryNotifyInformationClass: DIRECTORY_NOTIFY_INFORMATION_CLASS,
    ) -> NTSTATUS;

    pub fn NtLoadDriver(
        DriverServiceName: PCUNICODE_STRING,
    ) -> NTSTATUS;

    pub fn NtUnloadDriver(
        DriverServiceName: PCUNICODE_STRING,
    ) -> NTSTATUS;

    pub fn NtCreateIoCompletion(
        IoCompletionHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
        NumberOfConcurrentThreads: ULONG,
    ) -> NTSTATUS;

    pub fn NtOpenIoCompletion(
        IoCompletionHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn NtQueryIoCompletion(
        IoCompletionHandle: HANDLE,
        IoCompletionInformationClass: IO_COMPLETION_INFORMATION_CLASS,
        IoCompletionInformation: PVOID,
        IoCompletionInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;

    pub fn NtSetIoCompletion(
        IoCompletionHandle: HANDLE,
        KeyContext: PVOID,
        ApcContext: PVOID,
        IoStatus: NTSTATUS,
        IoStatusInformation: ULONG_PTR,
    ) -> NTSTATUS;

    pub fn NtSetIoCompletionEx(
        IoCompletionHandle: HANDLE,
        IoCompletionPacketHandle: HANDLE,
        KeyContext: PVOID,
        ApcContext: PVOID,
        IoStatus: NTSTATUS,
        IoStatusInformation: ULONG_PTR,
    ) -> NTSTATUS;

    pub fn NtRemoveIoCompletion(
        IoCompletionHandle: HANDLE,
        KeyContext: *mut PVOID,
        ApcContext: *mut PVOID,
        IoStatusBlock: PIO_STATUS_BLOCK,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn NtRemoveIoCompletionEx(
        IoCompletionHandle: HANDLE,
        IoCompletionInformation: *mut FILE_IO_COMPLETION_INFORMATION,
        Count: ULONG,
        NumEntriesRemoved: PULONG,
        Timeout: PLARGE_INTEGER,
        Alertable: BOOLEAN,
    ) -> NTSTATUS;

    pub fn NtCreateWaitCompletionPacket(
        WaitCompletionPacketHandle: PHANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: PCOBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn NtAssociateWaitCompletionPacket(
        WaitCompletionPacketHandle: HANDLE,
        IoCompletionHandle: HANDLE,
        TargetObjectHandle: HANDLE,
        KeyContext: PVOID,
        ApcContext: PVOID,
        IoStatus: NTSTATUS,
        IoStatusInformation: ULONG_PTR,
        AlreadySignaled: PBOOLEAN,
    ) -> NTSTATUS;

    pub fn NtCancelWaitCompletionPacket(
        WaitCompletionPacketHandle: HANDLE,
        RemoveSignaledPacket: BOOLEAN,
    ) -> NTSTATUS;

    pub fn NtCopyFileChunk(
        SourceHandle: HANDLE,
        DestinationHandle: HANDLE,
        EventHandle: HANDLE,
        IoStatusBlock: PIO_STATUS_BLOCK,
        Length: ULONG,
        SourceOffset: PLARGE_INTEGER,
        DestOffset: PLARGE_INTEGER,
        SourceKey: PULONG,
        DestKey: PULONG,
        Flags: ULONG,
    ) -> NTSTATUS;

    pub fn NtCreateIoRing(
        IoRingHandle: PHANDLE,
        CreateParametersLength: ULONG,
        CreateParameters: PVOID,
        OutputParametersLength: ULONG,
        OutputParameters: PVOID,
    ) -> NTSTATUS;

    pub fn NtSubmitIoRing(
        IoRingHandle: HANDLE,
        Flags: ULONG,
        WaitOperations: ULONG,
        Timeout: PLARGE_INTEGER,
    ) -> NTSTATUS;

    pub fn NtQueryIoRingCapabilities(
        IoRingCapabilitiesLength: SIZE_T,
        IoRingCapabilities: PVOID,
    ) -> NTSTATUS;

    pub fn NtSetInformationIoRing(
        IoRingHandle: HANDLE,
        IoRingInformationClass: ULONG,
        IoRingInformationLength: ULONG,
        IoRingInformation: PVOID,
    ) -> NTSTATUS;
}

#[repr(u32)]
pub enum INTERFACE_TYPE {
    InterfaceTypeUndefined = -1i32 as u32,
    Internal = 0,
    Isa = 1,
    Eisa = 2,
    MicroChannel = 3,
    TurboChannel = 4,
    PCIBus = 5,
    VMEBus = 6,
    NuBus = 7,
    PCMCIABus = 8,
    CBus = 9,
    MPIBus = 10,
    MPSABus = 11,
    ProcessorInternal = 12,
    InternalPowerBus = 13,
    PNPISABus = 14,
    PNPBus = 15,
    Vmcs = 16,
    ACPIBus = 17,
    MaximumInterfaceType = 18,
}

pub type PINTERFACE_TYPE = *mut INTERFACE_TYPE;

#[repr(u32)]
pub enum DMA_WIDTH {
    Width8Bits = 0,
    Width16Bits = 1,
    Width32Bits = 2,
    Width64Bits = 3,
    WidthNoWrap = 4,
    MaximumDmaWidth = 5,
}

pub type PDMA_WIDTH = *mut DMA_WIDTH;

#[repr(u32)]
pub enum DMA_SPEED {
    Compatible = 0,
    TypeA = 1,
    TypeB = 2,
    TypeC = 3,
    TypeF = 4,
    MaximumDmaSpeed = 5,
}

pub type PDMA_SPEED = *mut DMA_SPEED;

#[repr(u32)]
pub enum BUS_DATA_TYPE {
    ConfigurationSpaceUndefined = -1i32 as u32,
    Cmos = 0,
    EisaConfiguration = 1,
    Pos = 2,
    CbusConfiguration = 3,
    PCIConfiguration = 4,
    VMEConfiguration = 5,
    NuBusConfiguration = 6,
    PCMCIAConfiguration = 7,
    MPIConfiguration = 8,
    MPSAConfiguration = 9,
    PNPISAConfiguration = 10,
    SgiInternalConfiguration = 11,
    MaximumBusDataType = 12,
}

pub type PBUS_DATA_TYPE = *mut BUS_DATA_TYPE;

pub const SYMLINK_FLAG_RELATIVE: u32 = 0x00000001;
pub const SYMLINK_DIRECTORY: u32 = 0x80000000;
pub const SYMLINK_FILE: u32 = 0x40000000;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct REPARSE_DATA_BUFFER {
    pub ReparseTag: ULONG,
    pub ReparseDataLength: USHORT,
    pub Reserved: USHORT,
    pub GenericReparseBuffer: REPARSE_DATA_BUFFER_GENERIC,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union REPARSE_DATA_BUFFER_GENERIC {
    pub SymbolicLinkReparseBuffer: REPARSE_DATA_BUFFER_SYMBOLIC_LINK,
    pub MountPointReparseBuffer: REPARSE_DATA_BUFFER_MOUNT_POINT,
    pub AppExecLinkReparseBuffer: REPARSE_DATA_BUFFER_APP_EXEC_LINK,
    pub GenericReparseBuffer: REPARSE_DATA_BUFFER_GENERIC_DATA,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct REPARSE_DATA_BUFFER_SYMBOLIC_LINK {
    pub SubstituteNameOffset: USHORT,
    pub SubstituteNameLength: USHORT,
    pub PrintNameOffset: USHORT,
    pub PrintNameLength: USHORT,
    pub Flags: ULONG,
    pub PathBuffer: [WCHAR; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct REPARSE_DATA_BUFFER_MOUNT_POINT {
    pub SubstituteNameOffset: USHORT,
    pub SubstituteNameLength: USHORT,
    pub PrintNameOffset: USHORT,
    pub PrintNameLength: USHORT,
    pub PathBuffer: [WCHAR; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct REPARSE_DATA_BUFFER_APP_EXEC_LINK {
    pub StringCount: ULONG,
    pub StringList: [WCHAR; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct REPARSE_DATA_BUFFER_GENERIC_DATA {
    pub DataBuffer: [UCHAR; 1],
}

pub type PREPARSE_DATA_BUFFER = *mut REPARSE_DATA_BUFFER;

pub const REPARSE_DATA_BUFFER_HEADER_SIZE: usize = core::mem::offset_of!(REPARSE_DATA_BUFFER, GenericReparseBuffer);

#[repr(C)]
pub struct REPARSE_DATA_BUFFER_EX {
    pub Flags: ULONG,
    pub ExistingReparseTag: ULONG,
    pub ExistingReparseGuid: GUID,
    pub Reserved: ULONGLONG,
    pub ReparseBuffer: REPARSE_DATA_BUFFER_EX_REPARSE_BUFFER,
}

#[repr(C)]
pub union REPARSE_DATA_BUFFER_EX_REPARSE_BUFFER {
    pub ReparseDataBuffer: REPARSE_DATA_BUFFER,
    pub ReparseGuidDataBuffer: REPARSE_GUID_DATA_BUFFER,
}

pub type PREPARSE_DATA_BUFFER_EX = *mut REPARSE_DATA_BUFFER_EX;

pub const REPARSE_DATA_EX_FLAG_GIVEN_TAG_OR_NONE: u32 = 0x00000001;

// pub const REPARSE_GUID_DATA_BUFFER_EX_HEADER_SIZE: usize = core::mem::offset_of!(REPARSE_DATA_BUFFER_EX, ReparseBuffer.ReparseGuidDataBuffer.GenericReparseBuffer);
// pub const REPARSE_DATA_BUFFER_EX_HEADER_SIZE: usize = core::mem::offset_of!(REPARSE_DATA_BUFFER_EX, ReparseBuffer.ReparseDataBuffer.ReparseBuffer);
pub const REPARSE_GUID_DATA_BUFFER_EX_HEADER_SIZE: usize = 32;
pub const REPARSE_DATA_BUFFER_EX_HEADER_SIZE: usize = 32;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct REPARSE_GUID_DATA_BUFFER {
    pub ReparseTag: ULONG,
    pub ReparseDataLength: USHORT,
    pub Reserved: USHORT,
    pub ReparseGuid: GUID,
    pub GenericReparseBuffer: REPARSE_DATA_BUFFER_GENERIC,
}

pub type PREPARSE_GUID_DATA_BUFFER = *mut REPARSE_GUID_DATA_BUFFER;

#[repr(C)]
pub struct FILE_PIPE_ASSIGN_EVENT_BUFFER {
    pub EventHandle: HANDLE,
    pub KeyValue: ULONG,
}

pub type PFILE_PIPE_ASSIGN_EVENT_BUFFER = *mut FILE_PIPE_ASSIGN_EVENT_BUFFER;

#[repr(C)]
pub struct FILE_PIPE_PEEK_BUFFER {
    pub NamedPipeState: ULONG,
    pub ReadDataAvailable: ULONG,
    pub NumberOfMessages: ULONG,
    pub MessageLength: ULONG,
    pub Data: [CHAR; 1],
}

pub type PFILE_PIPE_PEEK_BUFFER = *mut FILE_PIPE_PEEK_BUFFER;

#[repr(C)]
pub struct FILE_PIPE_EVENT_BUFFER {
    pub NamedPipeState: ULONG,
    pub EntryType: ULONG,
    pub ByteCount: ULONG,
    pub KeyValue: ULONG,
    pub NumberRequests: ULONG,
}

pub type PFILE_PIPE_EVENT_BUFFER = *mut FILE_PIPE_EVENT_BUFFER;

#[repr(C)]
pub struct FILE_PIPE_WAIT_FOR_BUFFER {
    pub Timeout: LARGE_INTEGER,
    pub NameLength: ULONG,
    pub TimeoutSpecified: BOOLEAN,
    pub Name: [WCHAR; 1],
}

pub type PFILE_PIPE_WAIT_FOR_BUFFER = *mut FILE_PIPE_WAIT_FOR_BUFFER;

#[repr(C)]
pub struct FILE_PIPE_CLIENT_PROCESS_BUFFER {
    pub ClientSession: PVOID,
    pub ClientProcess: PVOID,
}

pub type PFILE_PIPE_CLIENT_PROCESS_BUFFER = *mut FILE_PIPE_CLIENT_PROCESS_BUFFER;

#[repr(C)]
pub struct FILE_PIPE_CLIENT_PROCESS_BUFFER_V2 {
    pub ClientSession: ULONGLONG,
    pub ClientProcess: ULONGLONG,
}

pub type PFILE_PIPE_CLIENT_PROCESS_BUFFER_V2 = *mut FILE_PIPE_CLIENT_PROCESS_BUFFER_V2;

pub const FILE_PIPE_COMPUTER_NAME_LENGTH: usize = 15;

#[repr(C)]
pub struct FILE_PIPE_CLIENT_PROCESS_BUFFER_EX {
    pub ClientSession: PVOID,
    pub ClientProcess: PVOID,
    pub ClientComputerNameLength: USHORT,
    pub ClientComputerBuffer: [WCHAR; FILE_PIPE_COMPUTER_NAME_LENGTH + 1],
}

pub type PFILE_PIPE_CLIENT_PROCESS_BUFFER_EX = *mut FILE_PIPE_CLIENT_PROCESS_BUFFER_EX;

#[repr(C)]
pub struct FILE_PIPE_SILO_ARRIVAL_INPUT {
    pub JobHandle: HANDLE,
}

pub type PFILE_PIPE_SILO_ARRIVAL_INPUT = *mut FILE_PIPE_SILO_ARRIVAL_INPUT;

pub const FILE_PIPE_SYMLINK_FLAG_GLOBAL: u32 = 0x1;
pub const FILE_PIPE_SYMLINK_FLAG_RELATIVE: u32 = 0x2;
pub const FILE_PIPE_SYMLINK_VALID_FLAGS: u32 = FILE_PIPE_SYMLINK_FLAG_GLOBAL | FILE_PIPE_SYMLINK_FLAG_RELATIVE;

#[repr(C)]
pub struct FILE_PIPE_CREATE_SYMLINK_INPUT {
    pub NameOffset: USHORT,
    pub NameLength: USHORT,
    pub SubstituteNameOffset: USHORT,
    pub SubstituteNameLength: USHORT,
    pub Flags: ULONG,
}

pub type PFILE_PIPE_CREATE_SYMLINK_INPUT = *mut FILE_PIPE_CREATE_SYMLINK_INPUT;

#[repr(C)]
pub struct FILE_PIPE_DELETE_SYMLINK_INPUT {
    pub NameOffset: USHORT,
    pub NameLength: USHORT,
}

pub type PFILE_PIPE_DELETE_SYMLINK_INPUT = *mut FILE_PIPE_DELETE_SYMLINK_INPUT;

pub const MAILSLOT_CLASS_FIRSTCLASS: i32 = 1;
pub const MAILSLOT_CLASS_SECONDCLASS: i32 = 2;

#[repr(C)]
pub struct FILE_MAILSLOT_PEEK_BUFFER {
    pub ReadDataAvailable: ULONG,
    pub NumberOfMessages: ULONG,
    pub MessageLength: ULONG,
}

pub type PFILE_MAILSLOT_PEEK_BUFFER = *mut FILE_MAILSLOT_PEEK_BUFFER;

pub const MOUNTMGRCONTROLTYPE: u32 = 0x0000006D;
pub const MOUNTDEVCONTROLTYPE: u32 = 0x0000004D;

#[repr(C)]
pub struct MOUNTMGR_CREATE_POINT_INPUT {
    pub SymbolicLinkNameOffset: USHORT,
    pub SymbolicLinkNameLength: USHORT,
    pub DeviceNameOffset: USHORT,
    pub DeviceNameLength: USHORT,
}

pub type PMOUNTMGR_CREATE_POINT_INPUT = *mut MOUNTMGR_CREATE_POINT_INPUT;

#[repr(C)]
pub struct MOUNTMGR_MOUNT_POINT {
    pub SymbolicLinkNameOffset: ULONG,
    pub SymbolicLinkNameLength: USHORT,
    pub Reserved1: USHORT,
    pub UniqueIdOffset: ULONG,
    pub UniqueIdLength: USHORT,
    pub Reserved2: USHORT,
    pub DeviceNameOffset: ULONG,
    pub DeviceNameLength: USHORT,
    pub Reserved3: USHORT,
}

pub type PMOUNTMGR_MOUNT_POINT = *mut MOUNTMGR_MOUNT_POINT;

#[repr(C)]
pub struct MOUNTMGR_MOUNT_POINTS {
    pub Size: ULONG,
    pub NumberOfMountPoints: ULONG,
    pub MountPoints: [MOUNTMGR_MOUNT_POINT; 1],
}

pub type PMOUNTMGR_MOUNT_POINTS = *mut MOUNTMGR_MOUNT_POINTS;

#[repr(C)]
pub struct MOUNTMGR_DRIVE_LETTER_TARGET {
    pub DeviceNameLength: USHORT,
    pub DeviceName: [WCHAR; 1],
}

pub type PMOUNTMGR_DRIVE_LETTER_TARGET = *mut MOUNTMGR_DRIVE_LETTER_TARGET;

#[repr(C)]
pub struct MOUNTMGR_DRIVE_LETTER_INFORMATION {
    pub DriveLetterWasAssigned: BOOLEAN,
    pub CurrentDriveLetter: UCHAR,
}

pub type PMOUNTMGR_DRIVE_LETTER_INFORMATION = *mut MOUNTMGR_DRIVE_LETTER_INFORMATION;

#[repr(C)]
pub struct MOUNTMGR_VOLUME_MOUNT_POINT {
    pub SourceVolumeNameOffset: USHORT,
    pub SourceVolumeNameLength: USHORT,
    pub TargetVolumeNameOffset: USHORT,
    pub TargetVolumeNameLength: USHORT,
}

pub type PMOUNTMGR_VOLUME_MOUNT_POINT = *mut MOUNTMGR_VOLUME_MOUNT_POINT;

#[repr(C)]
pub struct MOUNTMGR_CHANGE_NOTIFY_INFO {
    pub EpicNumber: ULONG,
}

pub type PMOUNTMGR_CHANGE_NOTIFY_INFO = *mut MOUNTMGR_CHANGE_NOTIFY_INFO;

#[repr(C)]
pub struct MOUNTMGR_TARGET_NAME {
    pub DeviceNameLength: USHORT,
    pub DeviceName: [WCHAR; 1],
}

pub type PMOUNTMGR_TARGET_NAME = *mut MOUNTMGR_TARGET_NAME;

#[repr(u32)]
pub enum MOUNTMGR_AUTO_MOUNT_STATE {
    Disabled = 0,
    Enabled = 1,
}

#[repr(C)]
pub struct MOUNTMGR_QUERY_AUTO_MOUNT {
    pub CurrentState: MOUNTMGR_AUTO_MOUNT_STATE,
}

pub type PMOUNTMGR_QUERY_AUTO_MOUNT = *mut MOUNTMGR_QUERY_AUTO_MOUNT;

#[repr(C)]
pub struct MOUNTMGR_SET_AUTO_MOUNT {
    pub NewState: MOUNTMGR_AUTO_MOUNT_STATE,
}

pub type PMOUNTMGR_SET_AUTO_MOUNT = *mut MOUNTMGR_SET_AUTO_MOUNT;

#[repr(C)]
pub struct MOUNTMGR_SILO_ARRIVAL_INPUT {
    pub JobHandle: HANDLE,
}

pub type PMOUNTMGR_SILO_ARRIVAL_INPUT = *mut MOUNTMGR_SILO_ARRIVAL_INPUT;

#[repr(C)]
pub struct MOUNTDEV_NAME {
    pub NameLength: USHORT,
    pub Name: [WCHAR; 1],
}

pub type PMOUNTDEV_NAME = *mut MOUNTDEV_NAME;

#[repr(C)]
pub struct MOUNTMGR_VOLUME_PATHS {
    pub MultiSzLength: ULONG,
    pub MultiSz: [WCHAR; 1],
}

pub type PMOUNTMGR_VOLUME_PATHS = *mut MOUNTMGR_VOLUME_PATHS;

#[repr(u32)]
pub enum IO_COMPLETION_INFORMATION_CLASS {
    IoCompletionBasicInformation = 0,
}

#[repr(C)]
pub struct IO_COMPLETION_BASIC_INFORMATION {
    pub Depth: LONG,
}

pub type PIO_COMPLETION_BASIC_INFORMATION = *mut IO_COMPLETION_BASIC_INFORMATION;

#[repr(C)]
pub struct FILE_IO_COMPLETION_INFORMATION {
    pub KeyContext: PVOID,
    pub ApcContext: PVOID,
    pub IoStatusBlock: IO_STATUS_BLOCK,
}

pub type PFILE_IO_COMPLETION_INFORMATION = *mut FILE_IO_COMPLETION_INFORMATION;

#[repr(u32)]
pub enum DIRECTORY_NOTIFY_INFORMATION_CLASS {
    DirectoryNotifyInformation = 1,
    DirectoryNotifyExtendedInformation = 2,
    DirectoryNotifyFullInformation = 3,
    DirectoryNotifyMaximumInformation = 4,
}

pub type PDIRECTORY_NOTIFY_INFORMATION_CLASS = *mut DIRECTORY_NOTIFY_INFORMATION_CLASS;

#[repr(C)]
pub struct FILE_NOTIFY_INFORMATION {
    pub NextEntryOffset: ULONG,
    pub Action: ULONG,
    pub FileNameLength: ULONG,
    pub FileName: [WCHAR; 1],
}

pub type PFILE_NOTIFY_INFORMATION = *mut FILE_NOTIFY_INFORMATION;

#[repr(C)]
pub struct FILE_NOTIFY_EXTENDED_INFORMATION {
    pub NextEntryOffset: ULONG,
    pub Action: ULONG,
    pub CreationTime: LARGE_INTEGER,
    pub LastModificationTime: LARGE_INTEGER,
    pub LastChangeTime: LARGE_INTEGER,
    pub LastAccessTime: LARGE_INTEGER,
    pub AllocatedLength: LARGE_INTEGER,
    pub FileSize: LARGE_INTEGER,
    pub FileAttributes: ULONG,
    pub ReparsePointTag: ULONG,
    pub FileId: FILE_INTERNAL_INFORMATION,
    pub ParentFileId: FILE_INTERNAL_INFORMATION,
    pub FileNameLength: ULONG,
    pub FileName: [WCHAR; 1],
}

pub type PFILE_NOTIFY_EXTENDED_INFORMATION = *mut FILE_NOTIFY_EXTENDED_INFORMATION;

pub const FILE_NAME_FLAG_HARDLINK: u32 = 0;
pub const FILE_NAME_FLAG_NTFS: u32 = 0x01;
pub const FILE_NAME_FLAG_DOS: u32 = 0x02;
pub const FILE_NAME_FLAG_BOTH: u32 = 0x03;
pub const FILE_NAME_FLAGS_UNSPECIFIED: u32 = 0x80;

#[repr(C)]
pub struct FILE_NOTIFY_FULL_INFORMATION {
    pub NextEntryOffset: ULONG,
    pub Action: ULONG,
    pub CreationTime: LARGE_INTEGER,
    pub LastModificationTime: LARGE_INTEGER,
    pub LastChangeTime: LARGE_INTEGER,
    pub LastAccessTime: LARGE_INTEGER,
    pub AllocatedLength: LARGE_INTEGER,
    pub FileSize: LARGE_INTEGER,
    pub FileAttributes: ULONG,
    pub ReparsePointTag: ULONG,
    pub FileId: FILE_INTERNAL_INFORMATION,
    pub ParentFileId: FILE_INTERNAL_INFORMATION,
    pub FileNameLength: USHORT,
    pub FileNameFlags: BYTE,
    pub Reserved: BYTE,
    pub FileName: [WCHAR; 1],
}

pub type PFILE_NOTIFY_FULL_INFORMATION = *mut FILE_NOTIFY_FULL_INFORMATION;

pub const FILE_NOTIFY_CHANGE_FILE_NAME: u32 = 0x00000001;
pub const FILE_NOTIFY_CHANGE_DIR_NAME: u32 = 0x00000002;
pub const FILE_NOTIFY_CHANGE_NAME: u32 = 0x00000003;
pub const FILE_NOTIFY_CHANGE_ATTRIBUTES: u32 = 0x00000004;
pub const FILE_NOTIFY_CHANGE_SIZE: u32 = 0x00000008;
pub const FILE_NOTIFY_CHANGE_LAST_WRITE: u32 = 0x00000010;
pub const FILE_NOTIFY_CHANGE_LAST_ACCESS: u32 = 0x00000020;
pub const FILE_NOTIFY_CHANGE_CREATION: u32 = 0x00000040;
pub const FILE_NOTIFY_CHANGE_EA: u32 = 0x00000080;
pub const FILE_NOTIFY_CHANGE_SECURITY: u32 = 0x00000100;
pub const FILE_NOTIFY_CHANGE_STREAM_NAME: u32 = 0x00000200;
pub const FILE_NOTIFY_CHANGE_STREAM_SIZE: u32 = 0x00000400;
pub const FILE_NOTIFY_CHANGE_STREAM_WRITE: u32 = 0x00000800;
pub const FILE_NOTIFY_VALID_MASK: u32 = 0x00000fff;

pub const FILE_ACTION_ADDED: u32 = 0x00000001;
pub const FILE_ACTION_REMOVED: u32 = 0x00000002;
pub const FILE_ACTION_MODIFIED: u32 = 0x00000003;
pub const FILE_ACTION_RENAMED_OLD_NAME: u32 = 0x00000004;
pub const FILE_ACTION_RENAMED_NEW_NAME: u32 = 0x00000005;
pub const FILE_ACTION_ADDED_STREAM: u32 = 0x00000006;
pub const FILE_ACTION_REMOVED_STREAM: u32 = 0x00000007;
pub const FILE_ACTION_MODIFIED_STREAM: u32 = 0x00000008;
pub const FILE_ACTION_REMOVED_BY_DELETE: u32 = 0x00000009;
pub const FILE_ACTION_ID_NOT_TUNNELLED: u32 = 0x0000000A;
pub const FILE_ACTION_TUNNELLED_ID_COLLISION: u32 = 0x0000000B;

pub const FILE_QUERY_RESTART_SCAN: u32 = 0x00000001;
pub const FILE_QUERY_RETURN_SINGLE_ENTRY: u32 = 0x00000002;
pub const FILE_QUERY_INDEX_SPECIFIED: u32 = 0x00000004;
pub const FILE_QUERY_RETURN_ON_DISK_ENTRIES_ONLY: u32 = 0x00000008;
pub const FILE_QUERY_NO_CURSOR_UPDATE: u32 = 0x00000010;

pub const FLUSH_FLAGS_FILE_DATA_ONLY: u32 = 0x00000001;
pub const FLUSH_FLAGS_NO_SYNC: u32 = 0x00000002;
pub const FLUSH_FLAGS_FILE_DATA_SYNC_ONLY: u32 = 0x00000004;
pub const FLUSH_FLAGS_FLUSH_AND_PURGE: u32 = 0x00000008;

pub const FILE_COMPUTER_NAME_LENGTH: usize = 15;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FSINFOCLASS {
    FileFsVolumeInformation = 1,
    FileFsLabelInformation = 2,
    FileFsSizeInformation = 3,
    FileFsDeviceInformation = 4,
    FileFsAttributeInformation = 5,
    FileFsControlInformation = 6,
    FileFsFullSizeInformation = 7,
    FileFsObjectIdInformation = 8,
    FileFsDriverPathInformation = 9,
    FileFsVolumeFlagsInformation = 10,
    FileFsSectorSizeInformation = 11,
    FileFsDataCopyInformation = 12,
    FileFsMetadataSizeInformation = 13,
    FileFsFullSizeInformationEx = 14,
    FileFsGuidInformation = 15,
    FileFsMaximumInformation = 16,
}
pub type PFSINFOCLASS = *mut FSINFOCLASS;