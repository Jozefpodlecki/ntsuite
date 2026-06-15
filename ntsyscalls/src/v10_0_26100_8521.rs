use ntapi::{ntdef::*, ntexapi::*, ntioapi::*, ntmmapi::*, ntrtl::POBJECT_TYPE_LIST, ntseapi::*, nttmapi::*};
#[unsafe(naked)]
pub fn NtQueryFullAttributesFile(
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	FileInformation: PFILE_NETWORK_OPEN_INFORMATION) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 337",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSubscribeWnfStateChange(
	StateName: PWNF_STATE_NAME, 
	ChangeStamp: WNF_CHANGE_STAMP, 
	Flags: ULONG, 
	EventHandle: HANDLE, 
	ApcRoutine: PIO_APC_ROUTINE, 
	ApcContext: PVOID, 
	Subscription: PWNF_USER_SUBSCRIPTION) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 461",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwRequestWaitReplyPort(
	PortHandle: HANDLE, 
	RequestMessage: PVOID, 
	ReplyMessage: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 34",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSignalAndWaitForSingleObject(
	SignalHandle: HANDLE, 
	WaitHandle: HANDLE, 
	Alertable: BOOLEAN, 
	Timeout: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 456",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtFreezeTransactions(
	TmHandle: PHANDLE, 
	Timeout: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 248",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
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
	ExclusiveLock: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 276",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAssignProcessToJobObject(
	JobHandle: HANDLE, 
	ProcessHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 145",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryInformationEnlistment(
	EnlistmentHandle: HANDLE, 
	EnlistmentInformationClass: ENLISTMENT_INFORMATION_CLASS, 
	EnlistmentInformation: PVOID, 
	EnlistmentInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 341",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtManagePartition(
	PartitionHandle: HANDLE, 
	Operation: ULONG, 
	Parameters: PVOID, 
	ParametersSize: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 283",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwFindAtom(
	AtomName: PWSTR, 
	Length: ULONG, 
	Atom: PUSHORT) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 20",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwEnumerateKey(
	KeyHandle: HANDLE, 
	Index: ULONG, 
	KeyInformationClass: KEY_INFORMATION_CLASS, 
	KeyInformation: PVOID, 
	Length: ULONG, 
	ResultLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 50",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateProcess(
	ProcessHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	ParentProcess: HANDLE, 
	InheritObjectTable: BOOLEAN, 
	SectionHandle: HANDLE, 
	DebugPort: HANDLE, 
	ExceptionPort: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 192",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwMapUserPhysicalPagesScatter(
	VirtualAddresses: PVOID, 
	NumberOfPages: ULONG_PTR, 
	UserPfnArray: PULONG_PTR) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 3",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtDisableLastKnownGood() -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 227",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwDeleteWnfStateName(
	StateName: PWNF_STATE_NAME) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 225",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAccessCheckByTypeResultListAndAuditAlarm(
	SubsystemName: PUNICODE_STRING, 
	HandleId: PVOID, 
	ObjectTypeName: PUNICODE_STRING, 
	ObjectName: PUNICODE_STRING, 
	SecurityDescriptor: PSECURITY_DESCRIPTOR, 
	ClientToken: HANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectTypeList: POBJECT_TYPE_LIST, 
	ObjectTypeListLength: ULONG, 
	GenericMapping: PGENERIC_MAPPING, 
	ObjectCreation: BOOLEAN, 
	GrantedAccessList: PULONG, 
	AccessStatusList: PULONG, 
	GenerateOnClose: PBOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 101",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAcquireProcessActivityReference(
	ProcessHandle: HANDLE, 
	Reference: PREFERENCE_LIST) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 104",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwWaitForAlertByThreadId(
	ThreadId: HANDLE, 
	Timeout: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 483",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryWnfStateData(
	StateName: PWNF_STATE_NAME, 
	TypeId: PWNF_TYPE_ID, 
	Buffer: PVOID, 
	BufferSize: PULONG, 
	ChangeStamp: PWNF_CHANGE_STAMP) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 368",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetValueKey(
	KeyHandle: HANDLE, 
	ValueName: PUNICODE_STRING, 
	TitleIndex: ULONG, 
	Type: ULONG, 
	Data: PVOID, 
	DataSize: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 96",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateIoCompletion(
	IoCompletionHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	Count: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 178",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAssociateWaitCompletionPacket(
	WaitCompletionPacketHandle: HANDLE, 
	CompletionPacketHandle: HANDLE, 
	WaitObject: HANDLE, 
	Flags: ULONG, 
	KeyContext: ULONG_PTR, 
	ApcContext: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 146",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateEnlistment(
	EnlistmentHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	ResourceManagerHandle: HANDLE, 
	TransactionHandle: HANDLE, 
	CreateOptions: ULONG, 
	NotificationMask: ULONG, 
	EnlistmentKey: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 175",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtOpenJobObject(
	JobHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 298",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwGetNotificationResourceManager(
	ResourceManagerHandle: HANDLE, 
	TransactionNotification: PVOID, 
	NotificationLength: ULONG, 
	Timeout: PLARGE_INTEGER, 
	ReturnLength: BOOLEAN, 
	ReturnLength: PULONG, 
	Asynchronous: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 259",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateProcess(
	ProcessHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	ParentProcess: HANDLE, 
	InheritObjectTable: BOOLEAN, 
	SectionHandle: HANDLE, 
	DebugPort: HANDLE, 
	ExceptionPort: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 192",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetVolumeInformationFile(
	FileHandle: HANDLE, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	FsInformation: PVOID, 
	Length: ULONG, 
	FsInformationClass: FSINFOCLASS) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 452",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwDeleteWnfStateData(
	StateName: PWNF_STATE_NAME, 
	TypeId: PWNF_TYPE_ID, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 224",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateToken(
	TokenHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	Type: TOKEN_TYPE, 
	AuthenticationId: PLUID, 
	ExpirationTime: PLARGE_INTEGER, 
	User: PTOKEN_USER, 
	Groups: PTOKEN_GROUPS, 
	Privileges: PTOKEN_PRIVILEGES, 
	Owner: PTOKEN_OWNER, 
	PrimaryGroup: PTOKEN_PRIMARY_GROUP, 
	DefaultDacl: PTOKEN_DEFAULT_DACL, 
	Source: PTOKEN_SOURCE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 205",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwDeleteKey(
	KeyHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 220",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtFilterToken(
	ExistingTokenHandle: HANDLE, 
	Flags: ULONG, 
	SidsToDisable: PTOKEN_GROUPS, 
	PrivilegesToDelete: PTOKEN_PRIVILEGES, 
	RestrictedSids: PTOKEN_GROUPS, 
	NewTokenHandle: PHANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 237",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwFilterBootOption(
	BootOptionId: ULONG, 
	Buffer: PVOID, 
	BufferSize: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 236",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtOpenKeyTransacted(
	KeyHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	TransactionHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 300",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateWnfStateName(
	StateName: PWNF_STATE_NAME, 
	NameLifetime: ULONG, 
	DataScope: ULONG, 
	PermanentData: BOOLEAN, 
	TypeId: PWNF_TYPE_ID, 
	MaximumStateSize: ULONG, 
	SecurityDescriptor: PSECURITY_DESCRIPTOR) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 212",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateSymbolicLinkObject(
	LinkHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	LinkTarget: PUNICODE_STRING) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 200",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryEvent(
	EventHandle: HANDLE, 
	EventInformationClass: EVENT_INFORMATION_CLASS, 
	EventInformation: PVOID, 
	EventInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 86",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateThreadEx(
	ThreadHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	ProcessHandle: HANDLE, 
	StartRoutine: PVOID, 
	Argument: PVOID, 
	CreateFlags: ULONG, 
	ZeroBits: SIZE_T, 
	StackSize: SIZE_T, 
	MaximumStackSize: SIZE_T, 
	AttributeList: PPS_ATTRIBUTE_LIST) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 201",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtInitiatePowerAction(
	SystemAction: POWER_ACTION, 
	LightestSystemState: SYSTEM_POWER_STATE, 
	Flags: ULONG, 
	Asynchronous: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 266",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtChangeThreadState(
	ThreadHandle: HANDLE, 
	TargetState: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 153",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwDisplayString(
	String: PUNICODE_STRING) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 228",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwReleaseMutant(
	MutantHandle: HANDLE, 
	PreviousCount: PLONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 32",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtFindAtom(
	AtomName: PWSTR, 
	Length: ULONG, 
	Atom: PUSHORT) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 20",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwApphelpCacheControl(
	ControlCode: ULONG, 
	Data: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 76",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwPssCaptureVaSpaceBulk(
	SnapshotHandle: HANDLE, 
	PageCount: PULONG, 
	Buffer: PVOID, 
	BufferSize: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 327",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateIRTimer(
	TimerHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 177",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryInformationTransaction(
	TransactionHandle: HANDLE, 
	TransactionInformationClass: TRANSACTION_INFORMATION_CLASS, 
	TransactionInformation: PVOID, 
	TransactionInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 345",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateThreadStateChange(
	StateChangeHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	ThreadHandle: HANDLE, 
	ProcessHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 202",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtUnloadKey(
	TargetKey: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 474",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwOpenResourceManager(
	ResourceManagerHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	TmHandle: HANDLE, 
	ResourceManagerId: LUID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 309",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwPrepareComplete(
	EnlistmentHandle: HANDLE, 
	TmVirtualClock: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 320",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateThreadEx(
	ThreadHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	ProcessHandle: HANDLE, 
	StartRoutine: PVOID, 
	Argument: PVOID, 
	CreateFlags: ULONG, 
	ZeroBits: SIZE_T, 
	StackSize: SIZE_T, 
	MaximumStackSize: SIZE_T, 
	AttributeList: PPS_ATTRIBUTE_LIST) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 201",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtOpenMutant(
	MutantHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 303",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwOpenKeyTransactedEx(
	KeyHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	OpenOptions: ULONG, 
	TransactionHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 301",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCancelIoFile(
	FileHandle: HANDLE, 
	IoStatusBlock: PIO_STATUS_BLOCK) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 93",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtFlushWriteBuffer() -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 245",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetCachedSigningLevel2(
	FileHandle: HANDLE, 
	Flags: ULONG, 
	SigningLevel: ULONG, 
	Thumbprint: PUCHAR, 
	ThumbprintSize: ULONG, 
	SectionFlags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 409",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetEventEx(
	EventHandle: HANDLE, 
	EventNumber: LONG, 
	PreviousState: PLONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 417",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtRollbackRegistryTransaction(
	TransactionHandle: HANDLE, 
	TmVirtualClock: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 398",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSaveKeyEx(
	KeyHandle: HANDLE, 
	FileHandle: HANDLE, 
	Format: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 402",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtDeleteDriverEntry(
	DriverEntry: PDRIVER_ENTRY) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 218",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtRollbackComplete(
	EnlistmentHandle: HANDLE, 
	TmVirtualClock: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 396",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCommitComplete(
	EnlistmentHandle: HANDLE, 
	TmVirtualClock: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 154",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtLockProductActivationKeys(
	Unknown: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 277",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateProfile(
	ProfileHandle: PHANDLE, 
	Process: HANDLE, 
	RangeBase: PVOID, 
	RangeSize: SIZE_T, 
	BucketSize: ULONG, 
	ProfileInfo: PULONG, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 194",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQuerySystemInformation(
	SystemInformationClass: SYSTEM_INFORMATION_CLASS, 
	SystemInformation: PVOID, 
	SystemInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 54",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryDriverEntryOrder(
	LoadOrder: PULONG, 
	Number: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 335",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtGetNextThread(
	ThreadHandle: HANDLE, 
	DesiredAccess: ACCESS_MASK, 
	HandleAttributes: ULONG, 
	Flags: ULONG, 
	NextThreadHandle: PHANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 257",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwNotifyChangeKey(
	KeyHandle: HANDLE, 
	Event: HANDLE, 
	ApcRoutine: PIO_APC_ROUTINE, 
	ApcContext: PVOID, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	CompletionFilter: ULONG, 
	WatchTree: BOOLEAN, 
	ChangeBuffer: PVOID, 
	BufferSize: ULONG, 
	Asynchronous: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 291",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryFullAttributesFile(
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	FileInformation: PFILE_NETWORK_OPEN_INFORMATION) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 337",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryPortInformationProcess(
	PortFlags: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 357",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtReleaseSemaphore(
	SemaphoreHandle: HANDLE, 
	ReleaseCount: LONG, 
	PreviousCount: PLONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 10",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwReadVirtualMemoryEx(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	Buffer: PVOID, 
	BufferSize: SIZE_T, 
	NumberOfBytesRead: PSIZE_T, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 375",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryVirtualMemory(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	MemoryInformationClass: MEMORY_INFORMATION_CLASS, 
	MemoryInformation: PVOID, 
	MemoryInformationLength: SIZE_T, 
	ReturnLength: PSIZE_T) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 35",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryIntervalProfile(
	Source: KPROFILE_SOURCE, 
	Interval: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 349",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateEventPair(
	EventPairHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 176",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwMakePermanentObject(
	Handle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 280",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAlpcCreatePortSection(
	PortHandle: HANDLE, 
	Flags: ULONG, 
	SectionHandle: HANDLE, 
	SectionSize: SIZE_T, 
	AlpcSectionHandle: PALPC_HANDLE, 
	SectionView: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 126",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateThread(
	ThreadHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	ProcessHandle: HANDLE, 
	ClientId: PCLIENT_ID, 
	ThreadContext: PCONTEXT, 
	InitialTeb: PUSER_STACK, 
	CreateSuspended: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 78",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSubscribeWnfStateChange(
	StateName: PWNF_STATE_NAME, 
	ChangeStamp: WNF_CHANGE_STAMP, 
	Flags: ULONG, 
	EventHandle: HANDLE, 
	ApcRoutine: PIO_APC_ROUTINE, 
	ApcContext: PVOID, 
	Subscription: PWNF_USER_SUBSCRIPTION) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 461",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtPssCaptureVaSpaceBulk(
	SnapshotHandle: HANDLE, 
	PageCount: PULONG, 
	Buffer: PVOID, 
	BufferSize: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 327",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtTerminateEnclave(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 465",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryTimer(
	TimerHandle: HANDLE, 
	TimerInformationClass: TIMER_INFORMATION_CLASS, 
	TimerInformation: PVOID, 
	TimerInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 56",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwUnloadDriver(
	DriverServiceName: PUNICODE_STRING) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 473",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetWnfProcessNotificationEvent(
	EventHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 453",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAccessCheckByType(
	SecurityDescriptor: PSECURITY_DESCRIPTOR, 
	ClientToken: HANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectTypeList: POBJECT_TYPE_LIST, 
	ObjectTypeListLength: ULONG, 
	GenericMapping: PGENERIC_MAPPING, 
	PrivilegeSet: PPRIVILEGE_SET, 
	PrivilegeSetLength: PULONG, 
	GrantedAccess: PACCESS_MASK, 
	AccessStatus: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 99",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtOpenProcess(
	ProcessHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	ClientId: PCLIENT_ID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 38",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtClose(
	Handle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 15",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCancelIoFileEx(
	FileHandle: HANDLE, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	CancelContext: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 148",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAcceptConnectPort(
	PortHandle: PHANDLE, 
	PortContext: PVOID, 
	ConnectionPortHandle: HANDLE, 
	SecurityQos: PSECURITY_QUALITY_OF_SERVICE, 
	ServerView: PVOID, 
	ServerViewSize: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 2",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtPrivilegeCheck(
	ClientToken: HANDLE, 
	RequiredPrivileges: PPRIVILEGE_SET, 
	SubjectContextLocked: PBOOLEAN, 
	Result: PBOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 322",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAddBootEntry(
	BootEntry: PBOOT_ENTRY) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 106",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateSectionEx(
	SectionHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	MaximumSize: PLARGE_INTEGER, 
	SectionPageProtection: ULONG, 
	AllocationAttributes: ULONG, 
	FileHandle: HANDLE, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 198",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwTerminateProcess(
	ProcessHandle: HANDLE, 
	ExitStatus: NTSTATUS) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 44",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateWaitCompletionPacket(
	WaitCompletionPacketHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 210",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateIoRing(
	IoRingHandle: PHANDLE, 
	Version: ULONG, 
	SubmissionQueueSize: ULONG, 
	CompletionQueueSize: ULONG, 
	Flags: ULONG, 
	SubmissionQueue: PVOID, 
	CompletionQueue: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 179",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtDirectGraphicsCall(
	Command: ULONG, 
	InputBuffer: PVOID, 
	InputBufferSize: ULONG, 
	OutputBuffer: PVOID, 
	OutputBufferSize: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 226",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAlpcDeleteResourceReserve(
	PortHandle: HANDLE, 
	Flags: ULONG, 
	ResourceReserve: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 131",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtFlushBuffersFile(
	FileHandle: HANDLE, 
	IoStatusBlock: PIO_STATUS_BLOCK) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 75",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwTraceControl(
	Operation: ULONG, 
	InputBuffer: PVOID, 
	InputBufferSize: ULONG, 
	OutputBuffer: PVOID, 
	OutputBufferSize: ULONG, 
	OutputBufferUsed: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 470",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAlpcSendWaitReceivePort(
	PortHandle: HANDLE, 
	Flags: ULONG, 
	SendMessage: PVOID, 
	SendAttributes: PALPC_MESSAGE_ATTRIBUTES, 
	ReceiveMessage: PVOID, 
	ReceiveMessageLength: PULONG, 
	ReceiveAttributes: PALPC_MESSAGE_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 142",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtMapUserPhysicalPagesScatter(
	VirtualAddresses: PVOID, 
	NumberOfPages: ULONG_PTR, 
	UserPfnArray: PULONG_PTR) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 3",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryVolumeInformationFile(
	FileHandle: HANDLE, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	FsInformation: PVOID, 
	Length: ULONG, 
	FsInformationClass: FSINFOCLASS) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 73",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtReleaseMutant(
	MutantHandle: HANDLE, 
	PreviousCount: PLONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 32",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryInstallUILanguage(
	InstallUILanguageId: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 348",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwPropagationComplete(
	ResourceManagerHandle: HANDLE, 
	PropagationId: ULONG, 
	PropagationStatus: NTSTATUS, 
	PropagationCount: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 325",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSuspendThread(
	ThreadHandle: HANDLE, 
	PreviousSuspendCount: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 463",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtNotifyChangeMultipleKeys(
	MasterKeyHandle: HANDLE, 
	Count: ULONG, 
	KeyObjects: POBJECT_ATTRIBUTES, 
	Event: HANDLE, 
	ApcRoutine: PIO_APC_ROUTINE, 
	ApcContext: PVOID, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	CompletionFilter: ULONG, 
	WatchTree: BOOLEAN, 
	ChangeBuffer: PVOID, 
	BufferSize: ULONG, 
	Asynchronous: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 292",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetCachedSigningLevel(
	FileHandle: HANDLE, 
	Flags: ULONG, 
	SigningLevel: ULONG, 
	Thumbprint: PUCHAR, 
	ThumbprintSize: ULONG, 
	SectionFlags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 408",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtLoadKeyEx(
	TargetKey: POBJECT_ATTRIBUTES, 
	SourceFile: POBJECT_ATTRIBUTES, 
	Flags: ULONG, 
	TrustClassKey: HANDLE, 
	Event: HANDLE, 
	DesiredAccess: ULONG, 
	RootHandle: PHANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 275",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtInitializeEnclave(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	ZeroBits: SIZE_T, 
	EnclaveInformation: PVOID, 
	InformationLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 263",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSuspendProcess(
	ProcessHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 462",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryInformationThread(
	ThreadHandle: HANDLE, 
	ThreadInformationClass: THREADINFOCLASS, 
	ThreadInformation: PVOID, 
	ThreadInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 37",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryInformationThread(
	ThreadHandle: HANDLE, 
	ThreadInformationClass: THREADINFOCLASS, 
	ThreadInformation: PVOID, 
	ThreadInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 37",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAlpcSetInformation(
	AlpcHandle: HANDLE, 
	AlpcInformationClass: ALPC_INFO_CLASS, 
	AlpcInformation: PVOID, 
	AlpcInformationLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 143",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwDrawText(
	WindowHandle: HANDLE, 
	Text: PUNICODE_STRING, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 229",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetSystemPowerState(
	SystemAction: POWER_ACTION, 
	LightestSystemState: SYSTEM_POWER_STATE, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 445",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateDirectoryObject(
	DirectoryHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 172",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwRevertContainerImpersonation() -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 395",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryLicenseValue(
	ValueName: PWSTR, 
	Type: PULONG, 
	Data: PVOID, 
	DataSize: ULONG, 
	ResultDataSize: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 352",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtRemoveIoCompletionEx(
	IoCompletionHandle: HANDLE, 
	Buffer: PVOID, 
	Count: ULONG, 
	Removed: PULONG, 
	Timeout: PLARGE_INTEGER, 
	Alertable: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 383",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryInformationByName(
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	FileInformation: PVOID, 
	Length: ULONG, 
	FileInformationClass: FILE_INFORMATION_CLASS) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 339",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwDeleteBootEntry(
	BootEntryId: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 217",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtDeleteWnfStateData(
	StateName: PWNF_STATE_NAME, 
	TypeId: PWNF_TYPE_ID, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 224",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtShutdownSystem(
	Action: SHUTDOWN_ACTION) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 454",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetContextThread(
	ThreadHandle: HANDLE, 
	Context: PCONTEXT) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 410",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAllocateVirtualMemoryEx(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	RegionSize: PSIZE_T, 
	AllocationType: ULONG, 
	Protect: ULONG, 
	ExtendedParameters: PULONG, 
	ParameterCount: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 120",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwOpenKeyedEvent(
	KeyedEventHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 302",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetLowWaitHighEventPair(
	EventPairHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 439",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetSystemInformation(
	SystemInformationClass: SYSTEM_INFORMATION_CLASS, 
	SystemInformation: PVOID, 
	SystemInformationLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 444",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAlpcCreateSectionView(
	PortHandle: HANDLE, 
	Flags: ULONG, 
	SectionView: PALPC_SECTION_VIEW) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 128",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwPrivilegedServiceAuditAlarm(
	SubsystemName: PUNICODE_STRING, 
	ServiceName: PUNICODE_STRING, 
	TokenHandle: HANDLE, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	GenerateOnClose: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 324",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateWaitablePort(
	PortHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 211",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAllocateUserPhysicalPagesEx(
	ProcessHandle: HANDLE, 
	NumberOfPages: PULONG_PTR, 
	UserPfnArray: PULONG_PTR, 
	ExtendedParameters: PULONG, 
	ParameterCount: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 118",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtWaitForDebugEvent(
	DebugObjectHandle: HANDLE, 
	Alertable: BOOLEAN, 
	Timeout: PLARGE_INTEGER, 
	WaitStateChange: PDBGUI_WAIT_STATE_CHANGE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 484",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQuerySystemEnvironmentValueEx(
	VariableName: PUNICODE_STRING, 
	VariableValue: PWSTR, 
	VariableValueLength: PUSHORT, 
	Attributes: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 365",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAlpcDeleteSecurityContext(
	PortHandle: HANDLE, 
	Flags: ULONG, 
	Context: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 133",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwFlushKey(
	KeyHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 242",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryOpenSubKeys(
	KeyHandle: HANDLE, 
	SubKeyCount: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 355",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryInformationFile(
	FileHandle: HANDLE, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	FileInformation: PVOID, 
	Length: ULONG, 
	FileInformationClass: FILE_INFORMATION_CLASS) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 17",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSecureConnectPort(
	PortHandle: PHANDLE, 
	PortName: PUNICODE_STRING, 
	SecurityQos: PSECURITY_QUALITY_OF_SERVICE, 
	ClientView: PVOID, 
	RequiredServerSid: PSID, 
	ConnectData: PVOID, 
	DataLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 404",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateRegistryTransaction(
	TransactionHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 196",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtMapUserPhysicalPages(
	VirtualAddress: PVOID, 
	NumberOfPages: ULONG_PTR, 
	UserPfnArray: PULONG_PTR) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 285",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtOpenObjectAuditAlarm(
	SubsystemName: PUNICODE_STRING, 
	HandleId: PVOID, 
	ObjectTypeName: PUNICODE_STRING, 
	ObjectName: PUNICODE_STRING, 
	SecurityDescriptor: PSECURITY_DESCRIPTOR, 
	ClientToken: HANDLE, 
	DesiredAccess: ACCESS_MASK, 
	GrantedAccess: ACCESS_MASK, 
	Privileges: PPRIVILEGE_SET, 
	ObjectCreation: BOOLEAN, 
	AccessGranted: BOOLEAN, 
	GenerateOnClose: PBOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 304",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtEnumerateValueKey(
	KeyHandle: HANDLE, 
	Index: ULONG, 
	KeyValueInformationClass: KEY_VALUE_INFORMATION_CLASS, 
	KeyValueInformation: PVOID, 
	Length: ULONG, 
	ResultLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 19",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCancelIoFileEx(
	FileHandle: HANDLE, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	CancelContext: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 148",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAlpcImpersonateClientContainerOfPort(
	PortHandle: HANDLE, 
	PortMessage: PVOID, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 135",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwWaitForWorkViaWorkerFactory(
	WorkerFactoryHandle: HANDLE, 
	WorkerCount: PULONG, 
	ProcessHandle: HANDLE, 
	WorkerParameter: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 486",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwOpenSession(
	SessionHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 311",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtLoadEnclaveData(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	Buffer: PVOID, 
	BufferSize: SIZE_T, 
	LoadInformation: PVOID, 
	InformationLength: ULONG, 
	EnclaveInformation: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 271",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtDebugContinue(
	DebugObjectHandle: HANDLE, 
	ClientId: CLIENT_ID, 
	ContinueStatus: NTSTATUS) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 215",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreatePort(
	PortHandle: PHANDLE, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	MaxConnectionInfoLength: ULONG, 
	MaxMessageLength: ULONG, 
	MaxPoolUsage: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 190",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtRecoverTransactionManager(
	TransactionManagerHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 378",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwClearEvent(
	EventHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 62",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAccessCheckAndAuditAlarm(
	SubsystemName: PUNICODE_STRING, 
	HandleId: PVOID, 
	ObjectTypeName: PUNICODE_STRING, 
	ObjectName: PUNICODE_STRING, 
	SecurityDescriptor: PSECURITY_DESCRIPTOR, 
	DesiredAccess: ACCESS_MASK, 
	GenericMapping: PGENERIC_MAPPING, 
	ObjectCreation: BOOLEAN, 
	GrantedAccess: PACCESS_MASK, 
	AccessStatus: PBOOLEAN, 
	GenerateOnClose: PBOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 41",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAdjustPrivilegesToken(
	TokenHandle: HANDLE, 
	DisableAllPrivileges: BOOLEAN, 
	NewState: PTOKEN_PRIVILEGES, 
	BufferLength: ULONG, 
	PreviousState: PTOKEN_PRIVILEGES, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 65",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtInitializeRegistry(
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 265",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryWnfStateData(
	StateName: PWNF_STATE_NAME, 
	TypeId: PWNF_TYPE_ID, 
	Buffer: PVOID, 
	BufferSize: PULONG, 
	ChangeStamp: PWNF_CHANGE_STAMP) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 368",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwMapViewOfSection(
	SectionHandle: HANDLE, 
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	ZeroBits: ULONG_PTR, 
	CommitSize: SIZE_T, 
	SectionOffset: PLARGE_INTEGER, 
	ViewSize: PSIZE_T, 
	InheritDisposition: ULONG, 
	AllocationType: ULONG, 
	Win32Protect: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 40",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetInformationWorkerFactory(
	WorkerFactoryHandle: HANDLE, 
	WorkerFactoryInformationClass: WORKERFACTORYINFOCLASS, 
	WorkerFactoryInformation: PVOID, 
	WorkerFactoryInformationLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 433",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwOpenThreadTokenEx(
	ThreadHandle: HANDLE, 
	DesiredAccess: ACCESS_MASK, 
	OpenAsSelf: BOOLEAN, 
	HandleAttributes: ULONG, 
	TokenHandle: PHANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 47",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtIsUILanguageComitted() -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 268",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwReadVirtualMemory(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	Buffer: PVOID, 
	BufferSize: SIZE_T, 
	NumberOfBytesRead: PSIZE_T) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 63",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetSystemTime(
	SystemTime: PLARGE_INTEGER, 
	PreviousTime: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 446",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwWaitForMultipleObjects32(
	Count: ULONG, 
	Handles: PHANDLE, 
	WaitType: WAIT_TYPE, 
	Alertable: BOOLEAN, 
	Timeout: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 26",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreatePort(
	PortHandle: PHANDLE, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	MaxConnectionInfoLength: ULONG, 
	MaxMessageLength: ULONG, 
	MaxPoolUsage: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 190",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryLicenseValue(
	ValueName: PWSTR, 
	Type: PULONG, 
	Data: PVOID, 
	DataSize: ULONG, 
	ResultDataSize: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 352",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwInitializeRegistry(
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 265",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtUnloadDriver(
	DriverServiceName: PUNICODE_STRING) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 473",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtOpenTimer(
	TimerHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 314",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwOpenEventPair(
	EventPairHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 296",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwThawTransactions(
	TmHandle: PHANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 469",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSuspendThread(
	ThreadHandle: HANDLE, 
	PreviousSuspendCount: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 463",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreatePartition(
	PartitionHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 189",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryInformationFile(
	FileHandle: HANDLE, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	FileInformation: PVOID, 
	Length: ULONG, 
	FileInformationClass: FILE_INFORMATION_CLASS) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 17",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQuerySystemInformationEx(
	SystemInformationClass: SYSTEM_INFORMATION_CLASS, 
	InputBuffer: PVOID, 
	InputBufferLength: ULONG, 
	SystemInformation: PVOID, 
	SystemInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 366",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwResumeThread(
	ThreadHandle: HANDLE, 
	PreviousSuspendCount: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 82",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQuerySecurityPolicy(
	SecurityPolicyId: ULONG, 
	Buffer: PVOID, 
	BufferSize: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 361",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCompareObjects(
	FirstObjectHandle: HANDLE, 
	SecondObjectHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 159",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAllocateVirtualMemoryEx(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	RegionSize: PSIZE_T, 
	AllocationType: ULONG, 
	Protect: ULONG, 
	ExtendedParameters: PULONG, 
	ParameterCount: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 120",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetUuidSeed(
	Seed: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 451",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtPrepareComplete(
	EnlistmentHandle: HANDLE, 
	TmVirtualClock: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 320",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetInformationTransaction(
	TransactionHandle: HANDLE, 
	TransactionInformationClass: TRANSACTION_INFORMATION_CLASS, 
	TransactionInformation: PVOID, 
	TransactionInformationLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 430",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwDuplicateToken(
	ExistingTokenHandle: HANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	EffectiveOnly: BOOLEAN, 
	TokenType: TOKEN_TYPE, 
	NewTokenHandle: PHANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 66",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtEnumerateDriverEntries(
	LoadOrder: PULONG, 
	Number: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 232",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateCpuPartition(
	CpuPartitionHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 168",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAcquireProcessActivityReference(
	ProcessHandle: HANDLE, 
	Reference: PREFERENCE_LIST) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 104",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetInformationResourceManager(
	ResourceManagerHandle: HANDLE, 
	ResourceManagerInformationClass: RESOURCEMANAGER_INFORMATION_CLASS, 
	ResourceManagerInformation: PVOID, 
	ResourceManagerInformationLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 427",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtUnmapViewOfSectionEx(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 479",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtWaitForSingleObject(
	Handle: HANDLE, 
	Alertable: BOOLEAN, 
	Timeout: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 4",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwEnumerateValueKey(
	KeyHandle: HANDLE, 
	Index: ULONG, 
	KeyValueInformationClass: KEY_VALUE_INFORMATION_CLASS, 
	KeyValueInformation: PVOID, 
	Length: ULONG, 
	ResultLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 19",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetInformationTransactionManager(
	TransactionManagerHandle: HANDLE, 
	TransactionManagerInformationClass: TRANSACTIONMANAGER_INFORMATION_CLASS, 
	TransactionManagerInformation: PVOID, 
	TransactionManagerInformationLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 431",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtReadVirtualMemory(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	Buffer: PVOID, 
	BufferSize: SIZE_T, 
	NumberOfBytesRead: PSIZE_T) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 63",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryIoCompletion(
	IoCompletionHandle: HANDLE, 
	IoCompletionInformationClass: IO_COMPLETION_INFORMATION_CLASS, 
	IoCompletionInformation: PVOID, 
	IoCompletionInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 350",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAccessCheckByTypeResultListAndAuditAlarmByHandle(
	SubsystemName: PUNICODE_STRING, 
	HandleId: PVOID, 
	ClientToken: HANDLE, 
	ObjectTypeName: PUNICODE_STRING, 
	ObjectName: PUNICODE_STRING, 
	SecurityDescriptor: PSECURITY_DESCRIPTOR, 
	DesiredAccess: ACCESS_MASK, 
	ObjectTypeList: POBJECT_TYPE_LIST, 
	ObjectTypeListLength: ULONG, 
	GenericMapping: PGENERIC_MAPPING, 
	ObjectCreation: BOOLEAN, 
	GrantedAccessList: PULONG, 
	AccessStatusList: PULONG, 
	GenerateOnClose: PBOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 102",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtRestoreKey(
	KeyHandle: HANDLE, 
	FileHandle: HANDLE, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 393",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetInformationThread(
	ThreadHandle: HANDLE, 
	ThreadInformationClass: THREADINFOCLASS, 
	ThreadInformation: PVOID, 
	ThreadInformationLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 13",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwOpenThread(
	ThreadHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	ClientId: PCLIENT_ID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 313",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateTokenEx(
	TokenHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	Type: TOKEN_TYPE, 
	AuthenticationId: PLUID, 
	ExpirationTime: PLARGE_INTEGER, 
	User: PTOKEN_USER, 
	Groups: PTOKEN_GROUPS, 
	Privileges: PTOKEN_PRIVILEGES, 
	UserAttributes: PTOKEN_SECURITY_ATTRIBUTES_INFORMATION, 
	DeviceAttributes: PTOKEN_SECURITY_ATTRIBUTES_INFORMATION, 
	DeviceGroups: PTOKEN_GROUPS, 
	MandatoryPolicy: PTOKEN_MANDATORY_POLICY, 
	Owner: PTOKEN_OWNER, 
	PrimaryGroup: PTOKEN_PRIMARY_GROUP, 
	DefaultDacl: PTOKEN_DEFAULT_DACL, 
	Source: PTOKEN_SOURCE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 206",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwLoadKeyEx(
	TargetKey: POBJECT_ATTRIBUTES, 
	SourceFile: POBJECT_ATTRIBUTES, 
	Flags: ULONG, 
	TrustClassKey: HANDLE, 
	Event: HANDLE, 
	DesiredAccess: ULONG, 
	RootHandle: PHANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 275",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwFsControlFile(
	FileHandle: HANDLE, 
	Event: HANDLE, 
	ApcRoutine: PIO_APC_ROUTINE, 
	ApcContext: PVOID, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	FsControlCode: ULONG, 
	InputBuffer: PVOID, 
	InputBufferLength: ULONG, 
	OutputBuffer: PVOID, 
	OutputBufferLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 57",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwRecoverEnlistment(
	EnlistmentHandle: HANDLE, 
	EnlistmentKey: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 376",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwEnableLastKnownGood() -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 230",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetInformationKey(
	KeyHandle: HANDLE, 
	KeySetInformationClass: KEY_SET_INFORMATION_CLASS, 
	KeySetInformation: PVOID, 
	Length: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 426",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAlpcRevokeSecurityContext(
	PortHandle: HANDLE, 
	Flags: ULONG, 
	Context: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 141",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwReplaceKey(
	FileHandle: POBJECT_ATTRIBUTES, 
	KeyHandle: HANDLE, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 387",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwUnloadKeyEx(
	KeyHandle: HANDLE, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 476",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwOpenKeyEx(
	KeyHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	OpenOptions: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 299",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAlpcOpenSenderProcess(
	ProcessHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	PortHandle: HANDLE, 
	PortMessage: PVOID, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 137",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtGetCachedSigningLevel(
	FileHandle: HANDLE, 
	Flags: PULONG, 
	SigningLevel: PULONG, 
	Thumbprint: PUCHAR, 
	ThumbprintSize: PULONG, 
	SectionFlags: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 249",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCallbackReturn(
	Result: PVOID, 
	ResultLength: ULONG, 
	Status: NTSTATUS) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 5",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwFreeUserPhysicalPages(
	ProcessHandle: HANDLE, 
	NumberOfPages: PULONG_PTR, 
	UserPfnArray: PULONG_PTR) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 246",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwNotifyChangeSession(
	SessionHandle: HANDLE, 
	EventHandle: HANDLE, 
	ApcRoutine: PVOID, 
	ApcContext: PVOID, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	CompletionFilter: ULONG, 
	WatchTree: BOOLEAN, 
	ChangeBuffer: PVOID, 
	BufferSize: ULONG, 
	Asynchronous: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 293",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSerializeBoot(
	SerializeBootValue: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 405",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryInformationProcess(
	ProcessHandle: HANDLE, 
	ProcessInformationClass: PROCESSINFOCLASS, 
	ProcessInformation: PVOID, 
	ProcessInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 25",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwStartProfile(
	ProfileObject: HANDLE, 
	ProfileSource: KPROFILE_SOURCE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 458",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtGetNotificationResourceManager(
	ResourceManagerHandle: HANDLE, 
	TransactionNotification: PVOID, 
	NotificationLength: ULONG, 
	Timeout: PLARGE_INTEGER, 
	ReturnLength: BOOLEAN, 
	ReturnLength: PULONG, 
	Asynchronous: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 259",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryInformationPort(
	PortHandle: HANDLE, 
	PortInformationClass: PORT_INFORMATION_CLASS, 
	PortInformation: PVOID, 
	Length: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 343",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQuerySecurityAttributesToken(
	TokenHandle: HANDLE, 
	AttributeName: PUNICODE_STRING, 
	Information: PVOID, 
	Length: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 359",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateJobSet(
	NumJob: ULONG, 
	UserJobSet: PJOB_SET_ARRAY, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 181",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwLockProductActivationKeys(
	Unknown: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 277",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwImpersonateAnonymousToken(
	ThreadHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 261",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateTimer2(
	TimerHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	Attributes: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 204",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwIsSystemResumeAutomatic() -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 267",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwContinue(
	ContextRecord: PCONTEXT, 
	TestAlert: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 67",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwModifyBootEntry(
	BootEntry: PBOOT_ENTRY) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 287",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetBootOptions(
	BootOptions: PVOID, 
	Length: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 407",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtFlushBuffersFileEx(
	FileHandle: HANDLE, 
	Flags: ULONG, 
	Parameters: PVOID, 
	ParametersSize: ULONG, 
	IoStatusBlock: PIO_STATUS_BLOCK) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 239",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateTransaction(
	TransactionHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	Uow: LUID, 
	TmHandle: HANDLE, 
	CreateOptions: ULONG, 
	IsolationLevel: ULONG, 
	IsolationFlags: ULONG, 
	Timeout: PLARGE_INTEGER, 
	Description: PWSTR) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 207",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryInformationByName(
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	FileInformation: PVOID, 
	Length: ULONG, 
	FileInformationClass: FILE_INFORMATION_CLASS) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 339",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSinglePhaseReject(
	EnlistmentHandle: HANDLE, 
	TmVirtualClock: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 457",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtNotifyChangeDirectoryFile(
	FileHandle: HANDLE, 
	Event: HANDLE, 
	ApcRoutine: PIO_APC_ROUTINE, 
	ApcContext: PVOID, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	Buffer: PVOID, 
	Length: ULONG, 
	CompletionFilter: ULONG, 
	WatchTree: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 289",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwReplacePartitionUnit(
	PartitionHandle: HANDLE, 
	TargetHandle: HANDLE, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 388",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtStartProfile(
	ProfileObject: HANDLE, 
	ProfileSource: KPROFILE_SOURCE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 458",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetInformationDebugObject(
	DebugObjectHandle: HANDLE, 
	DebugObjectInformationClass: DEBUGOBJECTINFOCLASS, 
	DebugObjectInformation: PVOID, 
	DebugObjectInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 422",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtOpenThread(
	ThreadHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	ClientId: PCLIENT_ID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 313",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwLockRegistryKey(
	KeyHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 278",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetInformationSymbolicLink(
	LinkHandle: HANDLE, 
	LinkInformationClass: SYMBOLIC_LINK_INFORMATION_CLASS, 
	LinkInformation: PVOID, 
	LinkInformationLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 428",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetEvent(
	EventHandle: HANDLE, 
	PreviousState: PLONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 14",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetSystemEnvironmentValue(
	VariableName: PUNICODE_STRING, 
	VariableValue: PUNICODE_STRING) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 442",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAddDriverEntry(
	DriverEntry: PDRIVER_ENTRY) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 107",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAllocateReserveObject(
	ReserveObject: PHANDLE, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	Type: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 116",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtReadFile(
	FileHandle: HANDLE, 
	Event: HANDLE, 
	ApcRoutine: PIO_APC_ROUTINE, 
	ApcContext: PVOID, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	Buffer: PVOID, 
	Length: ULONG, 
	ByteOffset: PLARGE_INTEGER, 
	Key: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 6",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwTerminateEnclave(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 465",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryTimerResolution(
	MinimumResolution: PULONG, 
	MaximumResolution: PULONG, 
	CurrentResolution: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 367",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtWaitForKeyedEvent(
	KeyedEventHandle: HANDLE, 
	ProcessHandle: HANDLE, 
	Key: PVOID, 
	Alertable: BOOLEAN, 
	Timeout: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 485",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAlpcImpersonateClientOfPort(
	PortHandle: HANDLE, 
	PortMessage: PVOID, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 136",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryDirectoryObject(
	DirectoryHandle: HANDLE, 
	Buffer: PVOID, 
	BufferLength: ULONG, 
	ReturnSingleEntry: BOOLEAN, 
	RestartScan: BOOLEAN, 
	Context: PULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 334",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtReplaceKey(
	NewFile: POBJECT_ATTRIBUTES, 
	KeyHandle: HANDLE, 
	OldFile: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 387",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwRollbackEnlistment(
	EnlistmentHandle: HANDLE, 
	TmVirtualClock: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 397",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetInformationProcess(
	ProcessHandle: HANDLE, 
	ProcessInformationClass: PROCESSINFOCLASS, 
	ProcessInformation: PVOID, 
	ProcessInformationLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 28",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryInformationWorkerFactory(
	WorkerFactoryHandle: HANDLE, 
	WorkerFactoryInformationClass: WORKERFACTORYINFOCLASS, 
	WorkerFactoryInformation: PVOID, 
	WorkerFactoryInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 347",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAlertThread(
	ThreadHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 112",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtWriteRequestData(
	PortHandle: HANDLE, 
	PortMessage: PVOID, 
	DataIndex: ULONG, 
	Buffer: PVOID, 
	BufferSize: ULONG, 
	OutputBufferSize: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 87",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtGetWriteWatch(
	ProcessHandle: HANDLE, 
	Flags: ULONG, 
	BaseAddress: PVOID, 
	RegionSize: SIZE_T, 
	UserAddressArray: PVOID, 
	EntriesCount: PULONG, 
	Granularity: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 260",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAlpcDeleteSecurityContext(
	PortHandle: HANDLE, 
	Flags: ULONG, 
	Context: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 133",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAlertThreadByThreadIdEx(
	ThreadId: HANDLE, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 114",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateRegistryTransaction(
	TransactionHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 196",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtRequestPort(
	PortHandle: HANDLE, 
	PortMessage: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 390",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetDriverEntryOrder(
	LoadOrder: PULONG, 
	Number: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 415",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtTranslateFilePath(
	FileHandle: HANDLE, 
	Operation: ULONG, 
	Buffer: PVOID, 
	BufferLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 471",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryDirectoryFile(
	FileHandle: HANDLE, 
	Event: HANDLE, 
	ApcRoutine: PIO_APC_ROUTINE, 
	ApcContext: PVOID, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	FileInformation: PVOID, 
	Length: ULONG, 
	FileInformationClass: FILE_INFORMATION_CLASS, 
	ReturnSingleEntry: BOOLEAN, 
	FileName: PUNICODE_STRING, 
	RestartScan: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 53",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSaveKey(
	KeyHandle: HANDLE, 
	FileHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 401",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtExtendSection(
	SectionHandle: HANDLE, 
	SectionSize: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 235",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtDebugActiveProcess(
	ProcessHandle: HANDLE, 
	DebugObjectHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 214",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetWnfProcessNotificationEvent(
	EventHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 453",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateProcessEx(
	ProcessHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	ParentProcess: HANDLE, 
	Flags: ULONG, 
	SectionHandle: HANDLE, 
	DebugPort: HANDLE, 
	ExceptionPort: HANDLE, 
	JobMemberLevel: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 77",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCompareTokens(
	FirstTokenHandle: HANDLE, 
	SecondTokenHandle: HANDLE, 
	Equal: PBOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 161",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQuerySemaphore(
	SemaphoreHandle: HANDLE, 
	SemaphoreInformationClass: SEMAPHORE_INFORMATION_CLASS, 
	SemaphoreInformation: PVOID, 
	SemaphoreInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 362",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateMutant(
	MutantHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	InitialOwner: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 186",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtOpenResourceManager(
	ResourceManagerHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	TmHandle: HANDLE, 
	ResourceManagerId: LUID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 309",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryIoRingCapabilities(
	Capabilities: PIORING_CAPABILITIES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 351",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetSystemInformation(
	SystemInformationClass: SYSTEM_INFORMATION_CLASS, 
	SystemInformation: PVOID, 
	SystemInformationLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 444",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwRollbackRegistryTransaction(
	TransactionHandle: HANDLE, 
	TmVirtualClock: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 398",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtWriteFile(
	FileHandle: HANDLE, 
	Event: HANDLE, 
	ApcRoutine: PIO_APC_ROUTINE, 
	ApcContext: PVOID, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	Buffer: PVOID, 
	Length: ULONG, 
	ByteOffset: PLARGE_INTEGER, 
	Key: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 8",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueueApcThread(
	ThreadHandle: HANDLE, 
	ApcRoutine: PIO_APC_ROUTINE, 
	ApcArgument1: PVOID, 
	ApcArgument2: PVOID, 
	ApcArgument3: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 69",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQuerySecurityObject(
	Handle: HANDLE, 
	SecurityInformation: SECURITY_INFORMATION, 
	SecurityDescriptor: PSECURITY_DESCRIPTOR, 
	Length: ULONG, 
	LengthNeeded: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 360",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwUnlockFile(
	FileHandle: HANDLE, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	ByteOffset: PLARGE_INTEGER, 
	Length: PLARGE_INTEGER, 
	Key: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 477",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwDisableLastKnownGood() -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 227",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwThawRegistry() -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 468",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtFlushVirtualMemory(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	RegionSize: PSIZE_T, 
	IoStatusBlock: PIO_STATUS_BLOCK) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 244",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtLockVirtualMemory(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	RegionSize: PSIZE_T, 
	MapType: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 279",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryOpenSubKeysEx(
	KeyHandle: HANDLE, 
	Format: ULONG, 
	Info: PVOID, 
	InfoLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 356",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtDuplicateToken(
	ExistingTokenHandle: HANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	EffectiveOnly: BOOLEAN, 
	TokenType: TOKEN_TYPE, 
	NewTokenHandle: PHANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 66",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryOpenSubKeys(
	KeyHandle: HANDLE, 
	SubKeyCount: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 355",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCompareSigningLevels(
	LeftSigningLevel: ULONG, 
	RightSigningLevel: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 160",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwOpenCpuPartition(
	CpuPartitionHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 294",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryDefaultLocale(
	UserProfile: BOOLEAN, 
	DefaultLocaleId: PLCID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 21",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAccessCheckByTypeResultListAndAuditAlarm(
	SubsystemName: PUNICODE_STRING, 
	HandleId: PVOID, 
	ObjectTypeName: PUNICODE_STRING, 
	ObjectName: PUNICODE_STRING, 
	SecurityDescriptor: PSECURITY_DESCRIPTOR, 
	ClientToken: HANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectTypeList: POBJECT_TYPE_LIST, 
	ObjectTypeListLength: ULONG, 
	GenericMapping: PGENERIC_MAPPING, 
	ObjectCreation: BOOLEAN, 
	GrantedAccessList: PULONG, 
	AccessStatusList: PULONG, 
	GenerateOnClose: PBOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 101",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetIoCompletionEx(
	IoCompletionHandle: HANDLE, 
	FileHandle: HANDLE, 
	KeyContext: PVOID, 
	ApcContext: PVOID, 
	IoStatus: NTSTATUS, 
	Information: ULONG_PTR, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 436",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwWriteFileGather(
	FileHandle: HANDLE, 
	Event: HANDLE, 
	ApcRoutine: PIO_APC_ROUTINE, 
	ApcContext: PVOID, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	SegmentArray: PFILE_SEGMENT_ELEMENT, 
	Length: ULONG, 
	ByteOffset: PLARGE_INTEGER, 
	Key: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 27",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtUpdateWnfStateData(
	StateName: PWNF_STATE_NAME, 
	Buffer: PVOID, 
	BufferSize: ULONG, 
	TypeId: PWNF_TYPE_ID, 
	ExplicitScope: PVOID, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 481",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtDeleteFile(
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 219",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryDefaultUILanguage(
	DefaultUILanguageId: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 68",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwLoadEnclaveData(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	Buffer: PVOID, 
	BufferSize: SIZE_T, 
	LoadInformation: PVOID, 
	InformationLength: ULONG, 
	EnclaveInformation: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 271",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtPlugPlayControl(
	ControlCode: ULONG, 
	Buffer: PVOID, 
	BufferSize: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 317",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetInformationCpuPartition(
	CpuPartitionHandle: HANDLE, 
	CpuPartitionInformationClass: CPU_PARTITION_INFORMATION_CLASS, 
	CpuPartitionInformation: PVOID, 
	CpuPartitionInformationLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 421",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtDelayExecution(
	Alertable: BOOLEAN, 
	DelayInterval: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 52",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwGetNextProcess(
	ProcessHandle: HANDLE, 
	DesiredAccess: ACCESS_MASK, 
	HandleAttributes: ULONG, 
	Flags: ULONG, 
	NextProcessHandle: PHANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 256",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetIoCompletionEx(
	IoCompletionHandle: HANDLE, 
	FileHandle: HANDLE, 
	KeyContext: PVOID, 
	ApcContext: PVOID, 
	IoStatus: NTSTATUS, 
	Information: ULONG_PTR, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 436",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwOpenThreadToken(
	ThreadHandle: HANDLE, 
	DesiredAccess: ACCESS_MASK, 
	OpenAsSelf: BOOLEAN, 
	TokenHandle: PHANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 36",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreatePagingFile(
	PageFileName: PUNICODE_STRING, 
	MinimumSize: PLARGE_INTEGER, 
	MaximumSize: PLARGE_INTEGER, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 188",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwFlushProcessWriteBuffers() -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 243",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwRollbackComplete(
	EnlistmentHandle: HANDLE, 
	TmVirtualClock: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 396",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueueApcThreadEx(
	ThreadHandle: HANDLE, 
	Reserved: HANDLE, 
	ApcRoutine: PKNORMAL_ROUTINE, 
	ApcArgument1: PVOID, 
	ApcArgument2: PVOID, 
	ApcArgument3: PVOID, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 370",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtGetNextProcess(
	ProcessHandle: HANDLE, 
	DesiredAccess: ACCESS_MASK, 
	HandleAttributes: ULONG, 
	Flags: ULONG, 
	NewProcessHandle: PHANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 256",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwFlushBuffersFile(
	FileHandle: HANDLE, 
	IoStatusBlock: PIO_STATUS_BLOCK) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 75",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAlpcCancelMessage(
	PortHandle: HANDLE, 
	Flags: ULONG, 
	PortMessage: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 122",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwOpenProcessTokenEx(
	ProcessHandle: HANDLE, 
	DesiredAccess: ACCESS_MASK, 
	HandleAttributes: ULONG, 
	TokenHandle: PHANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 48",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryInformationCpuPartition(
	CpuPartitionHandle: HANDLE, 
	CpuPartitionInformationClass: u32, 
	CpuPartitionInformation: PVOID, 
	CpuPartitionInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 340",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtOpenProcessToken(
	ProcessHandle: HANDLE, 
	DesiredAccess: ACCESS_MASK, 
	TokenHandle: PHANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 307",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAlpcOpenSenderThread(
	ThreadHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	PortHandle: HANDLE, 
	PortMessage: PVOID, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 138",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryKey(
	KeyHandle: HANDLE, 
	KeyInformationClass: KEY_INFORMATION_CLASS, 
	KeyInformation: PVOID, 
	Length: ULONG, 
	ResultLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 22",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAlpcOpenSenderThread(
	ThreadHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	PortHandle: HANDLE, 
	PortMessage: PVOID, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 138",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateWorkerFactory(
	WorkerFactoryHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	CompletionPortHandle: HANDLE, 
	WorkerProcessHandle: HANDLE, 
	StartRoutine: PVOID, 
	StartParameter: PVOID, 
	MaxThreadCount: ULONG, 
	StackReserve: SIZE_T, 
	StackCommit: SIZE_T) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 213",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwDebugActiveProcess(
	ProcessHandle: HANDLE, 
	DebugObjectHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 214",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetDebugFilterState(
	ComponentId: ULONG, 
	Level: ULONG, 
	State: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 411",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtWaitForAlertByThreadId(
	ThreadId: HANDLE, 
	Timeout: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 483",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwWriteFile(
	FileHandle: HANDLE, 
	Event: HANDLE, 
	ApcRoutine: PIO_APC_ROUTINE, 
	ApcContext: PVOID, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	Buffer: PVOID, 
	Length: ULONG, 
	ByteOffset: PLARGE_INTEGER, 
	Key: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 8",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetCachedSigningLevel(
	FileHandle: HANDLE, 
	Flags: ULONG, 
	SigningLevel: ULONG, 
	Thumbprint: PUCHAR, 
	ThumbprintSize: ULONG, 
	SectionFlags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 408",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwProtectVirtualMemory(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	RegionSize: PSIZE_T, 
	NewProtect: ULONG, 
	OldProtect: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 80",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwReadOnlyEnlistment(
	EnlistmentHandle: HANDLE, 
	TmVirtualClock: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 374",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAlpcCreateResourceReserve(
	PortHandle: HANDLE, 
	Flags: ULONG, 
	MessageSize: ULONG, 
	ActualMessageSize: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 127",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQuerySymbolicLinkObject(
	LinkHandle: HANDLE, 
	LinkTarget: PUNICODE_STRING, 
	ReturnedLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 363",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwRestoreKey(
	KeyHandle: HANDLE, 
	FileHandle: HANDLE, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 393",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwUpdateWnfStateData(
	StateName: PWNF_STATE_NAME, 
	Buffer: PVOID, 
	BufferSize: ULONG, 
	TypeId: PWNF_TYPE_ID, 
	ExplicitScope: PVOID, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 481",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtEnableLastKnownGood() -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 230",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtImpersonateAnonymousToken(
	ThreadHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 261",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtRemoveIoCompletion(
	IoCompletionHandle: HANDLE, 
	KeyContext: PVOID, 
	ApcContext: PVOID, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	Timeout: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 9",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwOpenMutant(
	MutantHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 303",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtMakeTemporaryObject(
	Handle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 281",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtRenameTransactionManager(
	LogFileName: PUNICODE_STRING, 
	TmIdentity: PGUID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 386",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetEventBoostPriority(
	EventHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 45",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwUnlockVirtualMemory(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	RegionSize: PSIZE_T, 
	MapType: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 478",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtReleaseKeyedEvent(
	KeyedEventHandle: HANDLE, 
	ProcessHandle: HANDLE, 
	Key: PVOID, 
	Alertable: BOOLEAN, 
	Timeout: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 381",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetSystemEnvironmentValue(
	VariableName: PUNICODE_STRING, 
	VariableValue: PUNICODE_STRING) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 442",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetSecurityObject(
	Handle: HANDLE, 
	SecurityInformation: SECURITY_INFORMATION, 
	SecurityDescriptor: PSECURITY_DESCRIPTOR) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 441",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQuerySemaphore(
	SemaphoreHandle: HANDLE, 
	SemaphoreInformationClass: u32, 
	SemaphoreInformation: PVOID, 
	SemaphoreInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 362",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtDeleteObjectAuditAlarm(
	SubsystemName: PUNICODE_STRING, 
	HandleId: PVOID, 
	GenerateOnClose: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 221",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwInitializeNlsFiles(
	BaseAddress: PVOID, 
	DefaultLocaleId: PLCID, 
	DefaultCasingTableSize: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 264",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwPrePrepareEnlistment(
	EnlistmentHandle: HANDLE, 
	TmVirtualClock: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 319",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAdjustPrivilegesToken(
	TokenHandle: HANDLE, 
	DisableAllPrivileges: BOOLEAN, 
	NewState: PTOKEN_PRIVILEGES, 
	BufferLength: ULONG, 
	PreviousState: PTOKEN_PRIVILEGES, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 65",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateProfile(
	ProfileHandle: PHANDLE, 
	Process: HANDLE, 
	RangeBase: PVOID, 
	RangeSize: SIZE_T, 
	BucketSize: ULONG, 
	ProfileInfo: PULONG, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 194",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwOpenProcess(
	ProcessHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	ClientId: PCLIENT_ID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 38",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwFreeVirtualMemory(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	RegionSize: PSIZE_T, 
	FreeType: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 30",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwReadFileScatter(
	FileHandle: HANDLE, 
	Event: HANDLE, 
	ApcRoutine: PIO_APC_ROUTINE, 
	ApcContext: PVOID, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	SegmentArray: PFILE_SEGMENT_ELEMENT, 
	Length: ULONG, 
	ByteOffset: PLARGE_INTEGER, 
	Key: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 46",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtManageHotPatch(
	Operation: ULONG, 
	Parameters: PVOID, 
	ParametersSize: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 282",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateIoRing(
	IoRingHandle: PHANDLE, 
	Version: ULONG, 
	SubmissionQueueSize: ULONG, 
	CompletionQueueSize: ULONG, 
	Flags: ULONG, 
	SubmissionQueue: PVOID, 
	CompletionQueue: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 179",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwPulseEvent(
	EventHandle: HANDLE, 
	PreviousState: LPLONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 328",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwFreezeTransactions(
	TmHandle: PHANDLE, 
	Timeout: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 248",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwRemoveProcessDebug(
	ProcessHandle: HANDLE, 
	DebugObjectHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 384",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwFlushVirtualMemory(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	RegionSize: PSIZE_T, 
	IoStatusBlock: PIO_STATUS_BLOCK) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 244",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAddBootEntry(
	BootEntry: PBOOT_ENTRY) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 106",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwResetWriteWatch(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	RegionSize: SIZE_T) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 392",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetTimer2(
	TimerHandle: HANDLE, 
	DueTime: PLARGE_INTEGER, 
	Period: PLARGE_INTEGER, 
	ApcRoutine: PTIMER_APC_ROUTINE, 
	ApcContext: PVOID, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 448",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtGetCurrentProcessorNumberEx(
	ProcNumber: PPROCESSOR_NUMBER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 253",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSubmitIoRing(
	IoRingHandle: HANDLE, 
	SubmissionCount: ULONG, 
	CompletionCount: ULONG, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 460",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwWaitForKeyedEvent(
	KeyedEventHandle: HANDLE, 
	ProcessHandle: HANDLE, 
	Key: PVOID, 
	Alertable: BOOLEAN, 
	Timeout: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 485",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetBootOptions(
	BootOptions: PVOID, 
	Length: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 407",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwOpenIoCompletion(
	IoCompletionHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 297",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCompareSigningLevels(
	LeftSigningLevel: ULONG, 
	RightSigningLevel: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 160",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetQuotaInformationFile(
	FileHandle: HANDLE, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	Buffer: PVOID, 
	Length: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 440",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwMakeTemporaryObject(
	Handle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 281",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateEnlistment(
	EnlistmentHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	ResourceManagerHandle: HANDLE, 
	TransactionHandle: HANDLE, 
	CreateOptions: ULONG, 
	NotificationMask: ULONG, 
	EnlistmentKey: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 175",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
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
	OutputBufferLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 7",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtPropagationFailed(
	ResourceManagerHandle: HANDLE, 
	PropagationId: ULONG, 
	PropagationCount: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 326",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryEaFile(
	FileHandle: HANDLE, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	Buffer: PVOID, 
	Length: ULONG, 
	ReturnSingleEntry: BOOLEAN, 
	EaList: PVOID, 
	EaListLength: ULONG, 
	EaIndex: PULONG, 
	RestartScan: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 336",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateJobObject(
	JobHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 180",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCallEnclave(
	Routine: PVOID, 
	Parameter: PVOID, 
	WaitForThread: BOOLEAN, 
	Result: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 147",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAlertThreadByThreadId(
	ThreadId: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 113",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtRemoveProcessDebug(
	ProcessHandle: HANDLE, 
	DebugObjectHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 384",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtWaitForWorkViaWorkerFactory(
	WorkerFactoryHandle: HANDLE, 
	WorkerCount: PULONG, 
	ProcessHandle: HANDLE, 
	WorkerParameter: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 486",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateEvent(
	EventHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	EventType: EVENT_TYPE, 
	InitialState: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 72",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwImpersonateClientOfPort(
	PortHandle: HANDLE, 
	PortMessage: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 31",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtOpenEventPair(
	EventPairHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 296",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwUnmapViewOfSectionEx(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 479",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwOpenPartition(
	PartitionHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 305",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCompleteConnectPort(
	PortHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 162",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtOpenIoCompletion(
	IoCompletionHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 297",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwPrivilegeObjectAuditAlarm(
	SubsystemName: PUNICODE_STRING, 
	HandleId: PVOID, 
	TokenHandle: HANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectTypeList: POBJECT_TYPE_LIST, 
	ObjectTypeListLength: ULONG, 
	ObjectCreation: BOOLEAN, 
	GenerateOnClose: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 323",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAlpcAcceptConnectPort(
	PortHandle: PHANDLE, 
	ConnectionPortHandle: HANDLE, 
	Flags: ULONG, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	PortAttributes: PALPC_PORT_ATTRIBUTES, 
	PortContext: PVOID, 
	ConnectionContext: PVOID, 
	ServerViewSize: PULONG, 
	ConnectionView: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 121",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtMapViewOfSectionEx(
	SectionHandle: HANDLE, 
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	ZeroBits: ULONG_PTR, 
	CommitSize: SIZE_T, 
	SectionOffset: PLARGE_INTEGER, 
	ViewSize: PSIZE_T, 
	InheritDisposition: ULONG, 
	AllocationType: ULONG, 
	Win32Protect: ULONG, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 286",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtOpenDirectoryObject(
	DirectoryHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 88",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetInformationTransactionManager(
	TransactionManagerHandle: HANDLE, 
	TransactionManagerInformationClass: u32, 
	TransactionManagerInformation: PVOID, 
	TransactionManagerInformationLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 431",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetTimer2(
	TimerHandle: HANDLE, 
	DueTime: PLARGE_INTEGER, 
	Period: PLARGE_INTEGER, 
	ApcRoutine: PTIMER_APC_ROUTINE, 
	ApcContext: PVOID, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 448",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryBootEntryOrder(
	BootEntries: PULONG, 
	BootCount: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 330",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryInformationEnlistment(
	EnlistmentHandle: HANDLE, 
	EnlistmentInformationClass: u32, 
	EnlistmentInformation: PVOID, 
	EnlistmentInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 341",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwFlushInstructionCache(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	RegionSize: SIZE_T) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 241",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwGetWriteWatch(
	ProcessHandle: HANDLE, 
	Flags: ULONG, 
	BaseAddress: PVOID, 
	RegionSize: SIZE_T, 
	UserAddressArray: PVOID, 
	EntriesCount: PULONG, 
	Granularity: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 260",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryPortInformationProcess(
	PortFlags: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 357",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCommitComplete(
	EnlistmentHandle: HANDLE, 
	TmVirtualClock: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 154",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateSection(
	SectionHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	MaximumSize: PLARGE_INTEGER, 
	SectionPageProtection: ULONG, 
	AllocationAttributes: ULONG, 
	FileHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 74",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAllocateUserPhysicalPages(
	ProcessHandle: HANDLE, 
	NumberOfPages: PULONG_PTR, 
	UserPfnArray: PULONG_PTR) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 117",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryBootOptions(
	BootOptionsId: PULONG, 
	Buffer: PVOID, 
	BufferSize: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 331",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAssignProcessToJobObject(
	JobHandle: HANDLE, 
	ProcessHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 145",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtRaiseHardError(
	ErrorStatus: NTSTATUS, 
	NumberOfParameters: ULONG, 
	UnicodeStringParameterMask: ULONG, 
	Parameters: PULONG_PTR, 
	ResponseOption: HARDERROR_RESPONSE_OPTION, 
	Response: PHARDERROR_RESPONSE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 373",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateJobObject(
	JobHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 180",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtIsSystemResumeAutomatic() -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 267",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtFlushInstructionCache(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	RegionSize: SIZE_T) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 241",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAllocateVirtualMemory(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	ZeroBits: ULONG_PTR, 
	RegionSize: PSIZE_T, 
	AllocationType: ULONG, 
	Protect: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 24",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSecureConnectPort(
	PortHandle: PHANDLE, 
	PortName: PUNICODE_STRING, 
	SecurityQos: PSECURITY_QUALITY_OF_SERVICE, 
	ClientView: PVOID, 
	RequiredServerSid: PSID, 
	ConnectData: PVOID, 
	DataLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 404",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwRollforwardTransactionManager(
	TransactionManagerHandle: HANDLE, 
	TmVirtualClock: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 400",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwGetContextThread(
	ThreadHandle: HANDLE, 
	Context: PCONTEXT) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 251",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwOpenDirectoryObject(
	DirectoryHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 88",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtPrePrepareEnlistment(
	EnlistmentHandle: HANDLE, 
	TmVirtualClock: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 319",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtRaiseException(
	ExceptionRecord: PEXCEPTION_RECORD, 
	ContextRecord: PCONTEXT, 
	FirstChance: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 372",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateIRTimer(
	TimerHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 177",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateTokenEx(
	TokenHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	Type: TOKEN_TYPE, 
	AuthenticationId: PLUID, 
	ExpirationTime: PLARGE_INTEGER, 
	User: PTOKEN_USER, 
	Groups: PTOKEN_GROUPS, 
	Privileges: PTOKEN_PRIVILEGES, 
	UserAttributes: PTOKEN_SECURITY_ATTRIBUTES_INFORMATION, 
	DeviceAttributes: PTOKEN_SECURITY_ATTRIBUTES_INFORMATION, 
	DeviceGroups: PTOKEN_GROUPS, 
	MandatoryPolicy: PTOKEN_MANDATORY_POLICY, 
	Owner: PTOKEN_OWNER, 
	PrimaryGroup: PTOKEN_PRIMARY_GROUP, 
	DefaultDacl: PTOKEN_DEFAULT_DACL, 
	Source: PTOKEN_SOURCE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 206",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtDeletePrivateNamespace(
	NamespaceHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 222",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtFlushProcessWriteBuffers() -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 243",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQuerySecurityObject(
	Handle: HANDLE, 
	SecurityInformation: SECURITY_INFORMATION, 
	SecurityDescriptor: PSECURITY_DESCRIPTOR, 
	Length: ULONG, 
	LengthNeeded: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 360",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAreMappedFilesTheSame(
	File1MappedAsAnImage: PVOID, 
	File2MappedAsFile: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 144",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAlpcAcceptConnectPort(
	PortHandle: PHANDLE, 
	ConnectionPortHandle: HANDLE, 
	Flags: ULONG, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	PortAttributes: PALPC_PORT_ATTRIBUTES, 
	PortContext: PVOID, 
	ConnectionContext: PVOID, 
	ServerViewSize: PULONG, 
	ConnectionView: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 121",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQuerySymbolicLinkObject(
	LinkHandle: HANDLE, 
	LinkTarget: PUNICODE_STRING, 
	ReturnedLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 363",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAlertResumeThread(
	ThreadHandle: HANDLE, 
	PreviousSuspendCount: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 111",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwRaiseException(
	ExceptionRecord: PEXCEPTION_RECORD, 
	ContextRecord: PCONTEXT, 
	FirstChance: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 372",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtRegisterProtocolAddressInformation(
	ResourceManagerHandle: HANDLE, 
	ProtocolName: PUNICODE_STRING, 
	CreateOptions: ULONG, 
	Persistence: ULONG, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 379",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAlpcQueryInformationMessage(
	PortHandle: HANDLE, 
	PortMessage: PVOID, 
	MessageInformationClass: ALPC_MESSAGE_INFORMATION_CLASS, 
	MessageInformation: PVOID, 
	MessageInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 140",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
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
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 290",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtInitializeNlsFiles(
	BaseAddress: PVOID, 
	DefaultLocaleId: PLCID, 
	DefaultCasingTableSize: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 264",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateJobSet(
	NumJob: ULONG, 
	UserJobSet: PJOB_SET_ARRAY, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 181",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQuerySecurityAttributesToken(
	TokenHandle: HANDLE, 
	AttributeName: PUNICODE_STRING, 
	Information: PVOID, 
	Length: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 359",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwOpenKey(
	KeyHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 18",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwReplyPort(
	PortHandle: HANDLE, 
	PortMessage: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 12",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtRecoverResourceManager(
	ResourceManagerHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 377",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtWriteFileGather(
	FileHandle: HANDLE, 
	Event: HANDLE, 
	ApcRoutine: PIO_APC_ROUTINE, 
	ApcContext: PVOID, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	SegmentArray: PFILE_SEGMENT_ELEMENT, 
	Length: ULONG, 
	ByteOffset: PLARGE_INTEGER, 
	Key: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 27",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtModifyBootEntry(
	BootEntry: PBOOT_ENTRY) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 287",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAssociateWaitCompletionPacket(
	WaitCompletionPacketHandle: HANDLE, 
	CompletionPacketHandle: HANDLE, 
	WaitObject: HANDLE, 
	Flags: ULONG, 
	KeyContext: ULONG_PTR, 
	ApcContext: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 146",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateNamedPipeFile(
	FileHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
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
	DefaultTimeout: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 187",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwReleaseWorkerFactoryWorker(
	WorkerFactoryHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 382",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetLowEventPair(
	EventPairHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 438",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetInformationKey(
	KeyHandle: HANDLE, 
	KeySetInformationClass: KEY_SET_INFORMATION_CLASS, 
	KeySetInformation: PVOID, 
	Length: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 426",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwGetDevicePowerState(
	DeviceHandle: HANDLE, 
	State: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 254",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtReplyPort(
	PortHandle: HANDLE, 
	PortMessage: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 12",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwVdmControl(
	ControlCode: ULONG, 
	ControlData: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 482",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateTransactionManager(
	TmHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	LogFileName: PUNICODE_STRING, 
	CreateOptions: ULONG, 
	CommitStrength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 208",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtRollbackEnlistment(
	EnlistmentHandle: HANDLE, 
	TmVirtualClock: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 397",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAdjustGroupsToken(
	TokenHandle: HANDLE, 
	ResetToDefault: BOOLEAN, 
	NewState: PTOKEN_GROUPS, 
	BufferLength: ULONG, 
	PreviousState: PTOKEN_GROUPS, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 108",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtWaitLowEventPair(
	EventPairHandle: HANDLE, 
	Alertable: BOOLEAN, 
	Timeout: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 488",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetSecurityObject(
	Handle: HANDLE, 
	SecurityInformation: SECURITY_INFORMATION, 
	SecurityDescriptor: PSECURITY_DESCRIPTOR) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 441",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetHighWaitLowEventPair(
	EventPairHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 419",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateEnclave(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	ZeroBits: SIZE_T, 
	Size: SIZE_T, 
	EnclaveType: ULONG, 
	EnclaveInformation: PVOID, 
	InformationLength: ULONG, 
	EnclaveInformation: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 174",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtPrivilegedServiceAuditAlarm(
	SubsystemName: PUNICODE_STRING, 
	ServiceName: PUNICODE_STRING, 
	TokenHandle: HANDLE, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	GenerateOnClose: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 324",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtFreeUserPhysicalPages(
	ProcessHandle: HANDLE, 
	NumberOfPages: PULONG_PTR, 
	UserPfnArray: PULONG_PTR) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 246",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtVdmControl(
	ControlCode: ULONG, 
	ControlData: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 482",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateSection(
	SectionHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	MaximumSize: PLARGE_INTEGER, 
	SectionPageProtection: ULONG, 
	AllocationAttributes: ULONG, 
	FileHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 74",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAccessCheckByTypeResultList(
	SecurityDescriptor: PSECURITY_DESCRIPTOR, 
	ClientToken: HANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectTypeList: POBJECT_TYPE_LIST, 
	ObjectTypeListLength: ULONG, 
	GenericMapping: PGENERIC_MAPPING, 
	PrivilegeSet: PPRIVILEGE_SET, 
	PrivilegeSetLength: PULONG, 
	GrantedAccessList: PULONG, 
	AccessStatusList: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 100",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryVirtualMemory(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	MemoryInformationClass: MEMORY_INFORMATION_CLASS, 
	MemoryInformation: PVOID, 
	MemoryInformationLength: SIZE_T, 
	ReturnLength: PSIZE_T) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 35",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwFilterToken(
	ExistingTokenHandle: HANDLE, 
	Flags: ULONG, 
	SidsToDisable: PTOKEN_GROUPS, 
	PrivilegesToDelete: PTOKEN_PRIVILEGES, 
	RestrictedSids: PTOKEN_GROUPS, 
	NewTokenHandle: PHANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 237",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtConnectPort(
	PortHandle: PHANDLE, 
	PortName: PUNICODE_STRING, 
	SecurityQos: PSECURITY_QUALITY_OF_SERVICE, 
	ClientView: PVOID, 
	RequiredServerSid: PSID, 
	ConnectData: PVOID, 
	DataLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 164",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateFile(
	FileHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	AllocationSize: PLARGE_INTEGER, 
	FileAttributes: ULONG, 
	ShareAccess: ULONG, 
	CreateDisposition: ULONG, 
	CreateOptions: ULONG, 
	EaBuffer: PVOID, 
	EaLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 85",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryEvent(
	EventHandle: HANDLE, 
	EventInformationClass: EVENT_INFORMATION_CLASS, 
	EventInformation: PVOID, 
	EventInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 86",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwTestAlert() -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 467",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtTerminateThread(
	ThreadHandle: HANDLE, 
	ExitStatus: NTSTATUS) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 83",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtUnlockVirtualMemory(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	RegionSize: PSIZE_T, 
	MapType: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 478",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwModifyDriverEntry(
	DriverEntry: PDRIVER_ENTRY) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 288",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryAuxiliaryCounterFrequency(
	Frequency: PULONGLONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 329",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtResetWriteWatch(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	RegionSize: SIZE_T) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 392",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtFlushKey(
	KeyHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 242",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtReplyWaitReceivePort(
	PortHandle: HANDLE, 
	ReplyMessage: PVOID, 
	ReceiveMessage: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 11",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetInformationProcess(
	ProcessHandle: HANDLE, 
	ProcessInformationClass: PROCESSINFOCLASS, 
	ProcessInformation: PVOID, 
	ProcessInformationLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 28",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateWorkerFactory(
	WorkerFactoryHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	CompletionPortHandle: HANDLE, 
	WorkerProcessHandle: HANDLE, 
	StartRoutine: PVOID, 
	StartParameter: PVOID, 
	MaxThreadCount: ULONG, 
	StackReserve: SIZE_T, 
	StackCommit: SIZE_T) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 213",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwTranslateFilePath(
	FileHandle: HANDLE, 
	Operation: ULONG, 
	Buffer: PVOID, 
	BufferLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 471",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtLoadDriver(
	DriverServiceName: PUNICODE_STRING) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 270",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryBootOptions(
	BootOptionsId: PULONG, 
	Buffer: PVOID, 
	BufferSize: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 331",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwInitiatePowerAction(
	SystemAction: POWER_ACTION, 
	LightestSystemState: SYSTEM_POWER_STATE, 
	Flags: ULONG, 
	Asynchronous: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 266",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAlpcConnectPort(
	PortHandle: PHANDLE, 
	PortName: PUNICODE_STRING, 
	PortAttributes: PALPC_PORT_ATTRIBUTES, 
	ConnectionPort: PALPC_HANDLE, 
	Flags: ULONG, 
	RequiredServerSid: PSID, 
	MessageAttributes: PALPC_MESSAGE_ATTRIBUTES, 
	ConnectData: PVOID, 
	ConnectDataLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 123",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCommitRegistryTransaction(
	TransactionHandle: HANDLE, 
	TmVirtualClock: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 156",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtContinueEx(
	ContextRecord: PCONTEXT, 
	TestAlert: BOOLEAN, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 165",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetLdtEntries(
	Selector1: ULONG, 
	LdtEntry1: LDT_ENTRY, 
	Selector2: ULONG, 
	LdtEntry2: LDT_ENTRY) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 437",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateKey(
	KeyHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	TitleIndex: ULONG, 
	Class: PUNICODE_STRING, 
	CreateOptions: ULONG, 
	Disposition: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 29",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSaveKeyEx(
	KeyHandle: HANDLE, 
	FileHandle: HANDLE, 
	Format: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 402",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAlpcDeletePortSection(
	PortHandle: HANDLE, 
	Flags: ULONG, 
	SectionHandle: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 130",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
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
	OutputBufferLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 57",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryQuotaInformationFile(
	FileHandle: HANDLE, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	Buffer: PVOID, 
	Length: ULONG, 
	ReturnSingleEntry: BOOLEAN, 
	SidList: PVOID, 
	SidListLength: ULONG, 
	StartSidIndex: PULONG, 
	RestartScan: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 358",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCompleteConnectPort(
	PortHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 162",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwYieldExecution() -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 70",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetInformationObject(
	Handle: HANDLE, 
	ObjectInformationClass: OBJECT_INFORMATION_CLASS, 
	ObjectInformation: PVOID, 
	ObjectInformationLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 92",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateLowBoxToken(
	TokenHandle: PHANDLE, 
	ExistingTokenHandle: HANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	PackageSid: PSID, 
	CapabilityCount: ULONG, 
	Capabilities: PSID_AND_ATTRIBUTES, 
	HandleCount: ULONG, 
	Handles: PHANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 184",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwNotifyChangeMultipleKeys(
	MasterKeyHandle: HANDLE, 
	Count: ULONG, 
	KeyObjects: POBJECT_ATTRIBUTES, 
	Event: HANDLE, 
	ApcRoutine: PIO_APC_ROUTINE, 
	ApcContext: PVOID, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	CompletionFilter: ULONG, 
	WatchTree: BOOLEAN, 
	ChangeBuffer: PVOID, 
	BufferSize: ULONG, 
	Asynchronous: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 292",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtOpenPrivateNamespace(
	NamespaceHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 306",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtPropagationComplete(
	ResourceManagerHandle: HANDLE, 
	PropagationId: ULONG, 
	PropagationStatus: NTSTATUS, 
	PropagationCount: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 325",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwNotifyChangeDirectoryFileEx(
	FileHandle: HANDLE, 
	Event: HANDLE, 
	ApcRoutine: PIO_APC_ROUTINE, 
	ApcContext: PVOID, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	Buffer: PVOID, 
	Length: ULONG, 
	CompletionFilter: ULONG, 
	WatchTree: BOOLEAN, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 290",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCancelIoFile(
	FileHandle: HANDLE, 
	IoStatusBlock: PIO_STATUS_BLOCK) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 93",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryMultipleValueKey(
	KeyHandle: HANDLE, 
	ValueEntries: PKEY_VALUE_ENTRY, 
	ValueCount: ULONG, 
	RequiredBufferSize: PULONG, 
	ValueBufferSize: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 353",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwFlushInstallUILanguage(
	InstallUILanguageId: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 240",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtFilterBootOption(
	BootOptionId: ULONG, 
	Buffer: PVOID, 
	BufferSize: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 236",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAdjustTokenClaimsAndDeviceGroups(
	TokenHandle: HANDLE, 
	ResetToDefault: BOOLEAN, 
	NewClaims: PTOKEN_CLAIMS, 
	NewDeviceGroups: PTOKEN_CLAIMS, 
	BufferLength: ULONG, 
	PreviousClaims: PTOKEN_CLAIMS, 
	PreviousDeviceGroups: PTOKEN_CLAIMS, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 109",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSubmitIoRing(
	IoRingHandle: HANDLE, 
	SubmissionCount: ULONG, 
	CompletionCount: ULONG, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 460",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetBootEntryOrder(
	BootEntries: PULONG, 
	BootCount: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 406",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtOpenTransaction(
	TransactionHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	Uow: LUID, 
	TmHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 315",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateKeyTransacted(
	KeyHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	TitleIndex: ULONG, 
	Class: PUNICODE_STRING, 
	CreateOptions: ULONG, 
	TransactionHandle: HANDLE, 
	Disposition: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 182",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtReplyWaitReplyPort(
	PortHandle: HANDLE, 
	ReplyMessage: PVOID, 
	ReceiveMessage: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 389",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtChangeProcessState(
	ProcessHandle: HANDLE, 
	TargetState: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 152",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwRemoveIoCompletionEx(
	IoCompletionHandle: HANDLE, 
	Buffer: PVOID, 
	Count: ULONG, 
	Removed: PULONG, 
	Timeout: PLARGE_INTEGER, 
	Alertable: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 383",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAlpcDeleteResourceReserve(
	PortHandle: HANDLE, 
	Flags: ULONG, 
	ResourceReserve: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 131",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAreMappedFilesTheSame(
	File1MappedAsAnImage: PVOID, 
	File2MappedAsFile: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 144",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwReleaseSemaphore(
	SemaphoreHandle: HANDLE, 
	ReleaseCount: LONG, 
	PreviousCount: LPLONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 10",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateDebugObject(
	DebugObjectHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 171",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateDirectoryObjectEx(
	DirectoryHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	ObjectName: PUNICODE_STRING, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 173",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwOpenProcessToken(
	ProcessHandle: HANDLE, 
	DesiredAccess: ACCESS_MASK, 
	TokenHandle: PHANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 307",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtOpenKeyTransactedEx(
	KeyHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	OpenOptions: ULONG, 
	TransactionHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 301",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreatePrivateNamespace(
	NamespaceHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	CreateParameters: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 191",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetIntervalProfile(
	Interval: ULONG, 
	Source: KPROFILE_SOURCE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 434",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCancelTimer2(
	TimerHandle: HANDLE, 
	CurrentState: PBOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 150",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryInformationAtom(
	AtomHandle: HANDLE, 
	AtomInformationClass: ATOM_INFORMATION_CLASS, 
	AtomInformation: PVOID, 
	AtomInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 338",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryAttributesFile(
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	FileInformation: PFILE_BASIC_INFORMATION) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 61",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwRegisterThreadTerminatePort(
	PortHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 380",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryEaFile(
	FileHandle: HANDLE, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	Buffer: PVOID, 
	Length: ULONG, 
	ReturnSingleEntry: BOOLEAN, 
	EaList: PVOID, 
	EaListLength: ULONG, 
	EaIndex: PULONG, 
	RestartScan: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 336",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtWaitForMultipleObjects32(
	Count: ULONG, 
	Handles: PHANDLE, 
	WaitType: WAIT_TYPE, 
	Alertable: BOOLEAN, 
	Timeout: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 26",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwIsProcessInJob(
	ProcessHandle: HANDLE, 
	JobHandle: HANDLE, 
	Result: PBOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 79",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetIRTimer(
	TimerHandle: HANDLE, 
	DueTime: PLARGE_INTEGER, 
	Period: PLARGE_INTEGER, 
	ApcRoutine: PTIMER_APC_ROUTINE, 
	ApcContext: PVOID, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 420",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwRemoveIoCompletion(
	IoCompletionHandle: HANDLE, 
	KeyContext: PVOID, 
	ApcContext: PVOID, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	Timeout: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 9",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateThreadStateChange(
	StateChangeHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	ThreadHandle: HANDLE, 
	ProcessHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 202",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryInformationJobObject(
	JobHandle: HANDLE, 
	JobObjectInformationClass: JOBOBJECTINFOCLASS, 
	JobObjectInformation: PVOID, 
	JobObjectInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 342",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwOpenTransaction(
	TransactionHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	Uow: LUID, 
	TmHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 315",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwEnumerateTransactionObject(
	EnumerateHandle: HANDLE, 
	TransactionObject: PULONG, 
	ObjectType: ULONG, 
	Count: ULONG, 
	ReturnCount: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 234",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSignalAndWaitForSingleObject(
	SignalHandle: HANDLE, 
	WaitHandle: HANDLE, 
	Alertable: BOOLEAN, 
	Timeout: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 456",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateTransaction(
	TransactionHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	Uow: LUID, 
	TmHandle: HANDLE, 
	CreateOptions: ULONG, 
	IsolationLevel: ULONG, 
	IsolationFlags: ULONG, 
	Timeout: PLARGE_INTEGER, 
	Description: PWSTR) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 207",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryOpenSubKeysEx(
	KeyHandle: HANDLE, 
	Format: ULONG, 
	Info: PVOID, 
	InfoLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 356",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtWaitForMultipleObjects(
	Count: ULONG, 
	Handles: PHANDLE, 
	WaitType: WAIT_TYPE, 
	Alertable: BOOLEAN, 
	Timeout: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 91",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtPrepareEnlistment(
	EnlistmentHandle: HANDLE, 
	TmVirtualClock: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 321",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAlpcCancelMessage(
	PortHandle: HANDLE, 
	Flags: ULONG, 
	PortMessage: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 122",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtWriteVirtualMemory(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	Buffer: PVOID, 
	BufferSize: SIZE_T, 
	NumberOfBytesWritten: PSIZE_T) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 58",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwLoadKey2(
	TargetKey: POBJECT_ATTRIBUTES, 
	SourceFile: POBJECT_ATTRIBUTES, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 273",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQuerySection(
	SectionHandle: HANDLE, 
	SectionInformationClass: SECTION_INFORMATION_CLASS, 
	SectionInformation: PVOID, 
	SectionInformationLength: SIZE_T, 
	ReturnLength: PSIZE_T) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 81",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtPrivilegeObjectAuditAlarm(
	SubsystemName: PUNICODE_STRING, 
	HandleId: PVOID, 
	TokenHandle: HANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectTypeList: POBJECT_TYPE_LIST, 
	ObjectTypeListLength: ULONG, 
	ObjectCreation: BOOLEAN, 
	GenerateOnClose: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 323",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateLowBoxToken(
	TokenHandle: PHANDLE, 
	ExistingTokenHandle: HANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	PackageSid: PSID, 
	CapabilityCount: ULONG, 
	Capabilities: PSID_AND_ATTRIBUTES, 
	HandleCount: ULONG, 
	Handles: PHANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 184",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtIsProcessInJob(
	ProcessHandle: HANDLE, 
	JobHandle: HANDLE, 
	Result: PBOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 79",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwLoadKey(
	TargetKey: POBJECT_ATTRIBUTES, 
	SourceFile: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 272",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAlpcQueryInformation(
	PortHandle: HANDLE, 
	AlpcInformationClass: ALPC_INFORMATION_CLASS, 
	AlpcInformation: PVOID, 
	AlpcInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 139",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtModifyDriverEntry(
	DriverEntry: PDRIVER_ENTRY) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 288",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwManageHotPatch(
	Operation: ULONG, 
	Parameters: PVOID, 
	ParametersSize: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 282",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateProfileEx(
	ProfileHandle: PHANDLE, 
	Process: HANDLE, 
	RangeBase: PVOID, 
	RangeSize: SIZE_T, 
	BucketSize: ULONG, 
	ProfileInfo: PULONG, 
	Flags: ULONG, 
	Buffer: PULONG, 
	BufferSize: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 195",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwChangeProcessState(
	ProcessHandle: HANDLE, 
	TargetState: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 152",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateMutant(
	MutantHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	InitialOwner: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 186",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtTraceControl(
	Operation: ULONG, 
	InputBuffer: PVOID, 
	InputBufferSize: ULONG, 
	OutputBuffer: PVOID, 
	OutputBufferSize: ULONG, 
	OutputBufferUsed: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 470",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwEnumerateSystemEnvironmentValuesEx(
	InformationClass: ULONG, 
	Buffer: PVOID, 
	BufferLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 233",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryAuxiliaryCounterFrequency(
	Frequency: PULONGLONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 329",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwDebugContinue(
	DebugObjectHandle: HANDLE, 
	ClientId: CLIENT_ID, 
	ContinueStatus: NTSTATUS) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 215",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtContinue(
	ContextRecord: PCONTEXT, 
	TestAlert: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 67",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwConvertBetweenAuxiliaryCounterAndPerformanceCounter(
	AuxiliaryCounterValue: PULONGLONG, 
	PerformanceCounterValue: PULONGLONG, 
	ConversionConstant: PULONGLONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 166",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtGetMUIRegistryInfo(
	Flags: ULONG, 
	Data: PVOID, 
	DataSize: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 255",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAlpcDeleteSectionView(
	PortHandle: HANDLE, 
	Flags: ULONG, 
	SectionView: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 132",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwResumeProcess(
	ProcessHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 394",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetInformationEnlistment(
	EnlistmentHandle: HANDLE, 
	EnlistmentInformationClass: ENLISTMENT_INFORMATION_CLASS, 
	EnlistmentInformation: PVOID, 
	EnlistmentInformationLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 423",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwListenPort(
	PortHandle: HANDLE, 
	ConnectionRequest: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 269",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetDefaultUILanguage(
	DefaultUILanguageId: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 414",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtResumeThread(
	ThreadHandle: HANDLE, 
	PreviousSuspendCount: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 82",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwOpenKeyTransacted(
	KeyHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	TransactionHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 300",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateFile(
	FileHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	AllocationSize: PLARGE_INTEGER, 
	FileAttributes: ULONG, 
	ShareAccess: ULONG, 
	CreateDisposition: ULONG, 
	CreateOptions: ULONG, 
	EaBuffer: PVOID, 
	EaLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 85",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetDefaultLocale(
	DefaultLocaleId: ULONG, 
	UserProfile: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 413",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtFreeVirtualMemory(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	RegionSize: PSIZE_T, 
	FreeType: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 30",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateWaitablePort(
	PortHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 211",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAlertMultipleThreadByThreadId(
	ThreadIds: PULONG, 
	Count: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 110",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCancelSynchronousIoFile(
	ThreadHandle: HANDLE, 
	IoStatusBlock: PIO_STATUS_BLOCK) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 149",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryInformationCpuPartition(
	CpuPartitionHandle: HANDLE, 
	CpuPartitionInformationClass: CPU_PARTITION_INFORMATION_CLASS, 
	CpuPartitionInformation: PVOID, 
	CpuPartitionInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 340",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtOpenPartition(
	PartitionHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 305",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwUnmapViewOfSection(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 42",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCommitEnlistment(
	EnlistmentHandle: HANDLE, 
	TmVirtualClock: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 155",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwLockVirtualMemory(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	RegionSize: PSIZE_T, 
	MapType: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 279",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryIoCompletion(
	IoCompletionHandle: HANDLE, 
	IoCompletionInformationClass: IO_COMPLETION_INFORMATION_CLASS, 
	IoCompletionInformation: PVOID, 
	IoCompletionInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 350",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtWaitHighEventPair(
	EventPairHandle: HANDLE, 
	Alertable: BOOLEAN, 
	Timeout: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 487",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCancelWaitCompletionPacket(
	WaitCompletionPacketHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 151",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwGetNextThread(
	ThreadHandle: HANDLE, 
	DesiredAccess: ACCESS_MASK, 
	HandleAttributes: ULONG, 
	Flags: ULONG, 
	NextThreadHandle: PHANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 257",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetDefaultHardErrorPort(
	PortHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 412",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateKeyedEvent(
	KeyedEventHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 183",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAllocateVirtualMemory(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	ZeroBits: ULONG_PTR, 
	RegionSize: PSIZE_T, 
	AllocationType: ULONG, 
	Protect: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 24",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryInformationResourceManager(
	ResourceManagerHandle: HANDLE, 
	ResourceManagerInformationClass: RESOURCEMANAGER_INFORMATION_CLASS, 
	ResourceManagerInformation: PVOID, 
	ResourceManagerInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 344",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAdjustGroupsToken(
	TokenHandle: HANDLE, 
	ResetToDefault: BOOLEAN, 
	NewState: PTOKEN_GROUPS, 
	BufferLength: ULONG, 
	PreviousState: PTOKEN_GROUPS, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 108",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetEaFile(
	FileHandle: HANDLE, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	Buffer: PVOID, 
	Length: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 416",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetTimer(
	TimerHandle: HANDLE, 
	DueTime: PLARGE_INTEGER, 
	TimerApcRoutine: PTIMER_APC_ROUTINE, 
	TimerContext: PVOID, 
	Resume: BOOLEAN, 
	Period: LONG, 
	PreviousState: PBOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 98",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAlertThreadByThreadIdEx(
	ThreadId: HANDLE, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 114",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryWnfStateNameInformation(
	StateName: PWNF_STATE_NAME, 
	InformationClass: WNF_STATE_NAME_INFORMATION_CLASS, 
	Information: PVOID, 
	InformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 369",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryObject(
	Handle: HANDLE, 
	ObjectInformationClass: OBJECT_INFORMATION_CLASS, 
	ObjectInformation: PVOID, 
	ObjectInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 16",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwMapCMFModule(
	CMFModule: PULONG, 
	Buffer: PVOID, 
	BufferSize: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 284",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtNotifyChangeSession(
	SessionHandle: HANDLE, 
	EventHandle: HANDLE, 
	ApcRoutine: PVOID, 
	ApcContext: PVOID, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	CompletionFilter: ULONG, 
	WatchTree: BOOLEAN, 
	ChangeBuffer: PVOID, 
	BufferSize: ULONG, 
	Asynchronous: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 293",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateEventPair(
	EventPairHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 176",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCompactKeys(
	Count: ULONG, 
	KeyHandles: PHANDLE, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 158",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwOpenEnlistment(
	EnlistmentHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	ResourceManagerHandle: HANDLE, 
	EnlistmentId: LUID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 295",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCompareObjects(
	FirstObjectHandle: HANDLE, 
	SecondObjectHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 159",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCommitTransaction(
	TransactionHandle: HANDLE, 
	Wait: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 157",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryAttributesFile(
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	FileInformation: PFILE_BASIC_INFORMATION) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 61",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwOpenSection(
	SectionHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 55",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetBootEntryOrder(
	BootEntries: PULONG, 
	BootCount: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 406",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAccessCheckByTypeResultList(
	SecurityDescriptor: PSECURITY_DESCRIPTOR, 
	ClientToken: HANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectTypeList: POBJECT_TYPE_LIST, 
	ObjectTypeListLength: ULONG, 
	GenericMapping: PGENERIC_MAPPING, 
	PrivilegeSet: PPRIVILEGE_SET, 
	PrivilegeSetLength: PULONG, 
	GrantedAccessList: PULONG, 
	AccessStatusList: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 100",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryDriverEntryOrder(
	LoadOrder: PULONG, 
	Number: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 335",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwWriteVirtualMemory(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	Buffer: PVOID, 
	BufferSize: SIZE_T, 
	NumberOfBytesWritten: PSIZE_T) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 58",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryInformationTransactionManager(
	TransactionManagerHandle: HANDLE, 
	TransactionManagerInformationClass: TRANSACTIONMANAGER_INFORMATION_CLASS, 
	TransactionManagerInformation: PVOID, 
	TransactionManagerInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 346",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtEnumerateSystemEnvironmentValuesEx(
	InformationClass: ULONG, 
	Buffer: PVOID, 
	BufferLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 233",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQuerySystemInformation(
	SystemInformationClass: SYSTEM_INFORMATION_CLASS, 
	SystemInformation: PVOID, 
	SystemInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 54",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryTimerResolution(
	MinimumResolution: PULONG, 
	MaximumResolution: PULONG, 
	CurrentResolution: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 367",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetInformationResourceManager(
	ResourceManagerHandle: HANDLE, 
	ResourceManagerInformationClass: RESOURCEMANAGER_INFORMATION_CLASS, 
	ResourceManagerInformation: PVOID, 
	ResourceManagerInformationLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 427",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtUnloadKeyEx(
	KeyHandle: HANDLE, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 476",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtOpenProcessTokenEx(
	ProcessHandle: HANDLE, 
	DesiredAccess: ACCESS_MASK, 
	HandleAttributes: ULONG, 
	TokenHandle: PHANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 48",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwRenameTransactionManager(
	LogFileName: PUNICODE_STRING, 
	TmIdentity: PGUID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 386",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAlertThreadByThreadId(
	ThreadId: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 113",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetValueKey(
	KeyHandle: HANDLE, 
	ValueName: PUNICODE_STRING, 
	TitleIndex: ULONG, 
	Type: ULONG, 
	Data: PVOID, 
	DataSize: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 96",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAlpcCreateSecurityContext(
	PortHandle: HANDLE, 
	Flags: ULONG, 
	SecurityAttributes: PALPC_SECURITY_ATTRIBUTES, 
	Context: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 129",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwShutdownSystem(
	Action: SHUTDOWN_ACTION) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 454",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetTimer(
	TimerHandle: HANDLE, 
	DueTime: PLARGE_INTEGER, 
	TimerApcRoutine: PTIMER_APC_ROUTINE, 
	TimerContext: PVOID, 
	Resume: BOOLEAN, 
	Period: LONG, 
	PreviousState: PBOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 98",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAlpcDeleteSectionView(
	PortHandle: HANDLE, 
	Flags: ULONG, 
	SectionView: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 132",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwOpenSymbolicLinkObject(
	LinkHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 312",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwDeleteDriverEntry(
	DriverEntry: PDRIVER_ENTRY) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 218",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetInformationJobObject(
	JobHandle: HANDLE, 
	JobObjectInformationClass: JOBOBJECTINFOCLASS, 
	JobObjectInformation: PVOID, 
	JobObjectInformationLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 425",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSerializeBoot(
	SerializeBootValue: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 405",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetThreadExecutionState(
	ExecutionState: ULONG, 
	PreviousExecutionState: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 447",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtOpenSection(
	SectionHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 55",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtMakePermanentObject(
	Handle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 280",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSaveKey(
	KeyHandle: HANDLE, 
	FileHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 401",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateIoCompletion(
	IoCompletionHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	Count: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 178",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetContextThread(
	ThreadHandle: HANDLE, 
	Context: PCONTEXT) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 410",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwWriteRequestData(
	PortHandle: HANDLE, 
	PortMessage: PVOID, 
	DataIndex: ULONG, 
	Buffer: PVOID, 
	BufferSize: ULONG, 
	OutputBufferSize: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 87",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtGetContextThread(
	ThreadHandle: HANDLE, 
	Context: PCONTEXT) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 251",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryValueKey(
	KeyHandle: HANDLE, 
	ValueName: PUNICODE_STRING, 
	KeyValueInformationClass: KEY_VALUE_INFORMATION_CLASS, 
	KeyValueInformation: PVOID, 
	Length: ULONG, 
	ResultLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 23",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAlpcQueryInformationMessage(
	PortHandle: HANDLE, 
	PortMessage: PVOID, 
	MessageInformationClass: ALPC_MESSAGE_INFORMATION_CLASS, 
	MessageInformation: PVOID, 
	MessageInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 140",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetInformationThread(
	ThreadHandle: HANDLE, 
	ThreadInformationClass: THREADINFOCLASS, 
	ThreadInformation: PVOID, 
	ThreadInformationLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 13",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtReplacePartitionUnit(
	PartitionHandle: HANDLE, 
	TargetHandle: HANDLE, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 388",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtApphelpCacheControl(
	ControlCode: ULONG, 
	Data: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 76",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueueApcThreadEx(
	ThreadHandle: HANDLE, 
	Reserved: HANDLE, 
	ApcRoutine: PKNORMAL_ROUTINE, 
	ApcArgument1: PVOID, 
	ApcArgument2: PVOID, 
	ApcArgument3: PVOID, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 370",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreatePrivateNamespace(
	NamespaceHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	CreateParameters: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 191",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetInformationVirtualMemory(
	ProcessHandle: HANDLE, 
	VirtualMemoryInformationClass: VIRTUAL_MEMORY_INFORMATION_CLASS, 
	VirtualMemoryInformation: PVOID, 
	VirtualMemoryInformationLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 432",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryTimer(
	TimerHandle: HANDLE, 
	TimerInformationClass: TIMER_INFORMATION_CLASS, 
	TimerInformation: PVOID, 
	TimerInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 56",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAlpcDeletePortSection(
	PortHandle: HANDLE, 
	Flags: ULONG, 
	SectionHandle: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 130",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwDeleteObjectAuditAlarm(
	SubsystemName: PUNICODE_STRING, 
	HandleId: PVOID, 
	GenerateOnClose: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 221",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAddDriverEntry(
	DriverEntry: PDRIVER_ENTRY) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 107",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetHighEventPair(
	EventPairHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 418",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwOpenRegistryTransaction(
	TransactionHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 308",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAccessCheckAndAuditAlarm(
	SubsystemName: PUNICODE_STRING, 
	HandleId: PVOID, 
	ObjectTypeName: PUNICODE_STRING, 
	ObjectName: PUNICODE_STRING, 
	SecurityDescriptor: PSECURITY_DESCRIPTOR, 
	DesiredAccess: ACCESS_MASK, 
	GenericMapping: PGENERIC_MAPPING, 
	ObjectCreation: BOOLEAN, 
	GrantedAccess: PACCESS_MASK, 
	AccessStatus: PBOOLEAN, 
	GenerateOnClose: PBOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 41",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwUmsThreadYield(
	Reason: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 472",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwContinueEx(
	ContextRecord: PCONTEXT, 
	TestAlert: BOOLEAN, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 165",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAddAtom(
	AtomName: PWSTR, 
	Length: ULONG, 
	Atom: PUSHORT) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 71",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtOpenKeyedEvent(
	KeyedEventHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 302",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQuerySystemEnvironmentValue(
	VariableName: PUNICODE_STRING, 
	VariableValue: PWSTR, 
	VariableValueLength: USHORT, 
	ReturnLength: PUSHORT) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 364",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtDeleteBootEntry(
	BootEntryId: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 217",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateMailslotFile(
	FileHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	CreateOptions: ULONG, 
	MailslotQuota: ULONG, 
	MaximumMessageSize: ULONG, 
	ReadTimeout: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 185",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtTerminateJobObject(
	JobHandle: HANDLE, 
	ExitStatus: NTSTATUS) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 466",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwReleaseKeyedEvent(
	KeyedEventHandle: HANDLE, 
	ProcessHandle: HANDLE, 
	Key: PVOID, 
	Alertable: BOOLEAN, 
	Timeout: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 381",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateCrossVmEvent(
	EventHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	VmHandle: HANDLE, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 169",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwFlushBuffersFileEx(
	FileHandle: HANDLE, 
	Flags: ULONG, 
	Parameters: PVOID, 
	ParametersSize: ULONG, 
	IoStatusBlock: PIO_STATUS_BLOCK) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 239",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryDirectoryFileEx(
	FileHandle: HANDLE, 
	Event: HANDLE, 
	ApcRoutine: PIO_APC_ROUTINE, 
	ApcContext: PVOID, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	FileInformation: PVOID, 
	Length: ULONG, 
	FileInformationClass: FILE_INFORMATION_CLASS, 
	ReturnSingleEntry: BOOLEAN, 
	FileName: PUNICODE_STRING, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 333",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetIntervalProfile(
	Interval: ULONG, 
	Source: KPROFILE_SOURCE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 434",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwImpersonateThread(
	ServerThreadHandle: HANDLE, 
	ClientThreadHandle: HANDLE, 
	SecurityQos: PSECURITY_QUALITY_OF_SERVICE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 262",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryPerformanceCounter(
	PerformanceCounter: PLARGE_INTEGER, 
	PerformanceFrequency: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 49",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtDisplayString(
	String: PUNICODE_STRING) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 228",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtGetCurrentProcessorNumber() -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 252",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAccessCheckByType(
	SecurityDescriptor: PSECURITY_DESCRIPTOR, 
	ClientToken: HANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectTypeList: POBJECT_TYPE_LIST, 
	ObjectTypeListLength: ULONG, 
	GenericMapping: PGENERIC_MAPPING, 
	PrivilegeSet: PPRIVILEGE_SET, 
	PrivilegeSetLength: PULONG, 
	GrantedAccess: PACCESS_MASK, 
	AccessStatus: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 99",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryValueKey(
	KeyHandle: HANDLE, 
	ValueName: PUNICODE_STRING, 
	KeyValueInformationClass: KEY_VALUE_INFORMATION_CLASS, 
	KeyValueInformation: PVOID, 
	Length: ULONG, 
	ResultLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 23",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtMapCMFModule(
	CMFModule: PULONG, 
	Buffer: PVOID, 
	BufferSize: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 284",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwRollbackTransaction(
	TransactionHandle: HANDLE, 
	Wait: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 399",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwMapUserPhysicalPages(
	VirtualAddress: PVOID, 
	NumberOfPages: ULONG_PTR, 
	UserPfnArray: PULONG_PTR) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 285",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateSemaphore(
	SemaphoreHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	InitialCount: LONG, 
	MaximumCount: LONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 199",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryInformationTransactionManager(
	TransactionManagerHandle: HANDLE, 
	TransactionManagerInformationClass: TRANSACTIONMANAGER_INFORMATION_CLASS, 
	TransactionManagerInformation: PVOID, 
	TransactionManagerInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 346",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtRollforwardTransactionManager(
	TransactionManagerHandle: HANDLE, 
	TmVirtualClock: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 400",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAlpcConnectPort(
	PortHandle: PHANDLE, 
	PortName: PUNICODE_STRING, 
	PortAttributes: PALPC_PORT_ATTRIBUTES, 
	ConnectionPort: PALPC_HANDLE, 
	Flags: ULONG, 
	RequiredServerSid: PSID, 
	MessageAttributes: PALPC_MESSAGE_ATTRIBUTES, 
	ConnectData: PVOID, 
	ConnectDataLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 123",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwWaitForSingleObject(
	Handle: HANDLE, 
	Alertable: BOOLEAN, 
	Timeout: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 4",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQuerySecurityPolicy(
	SecurityPolicyId: ULONG, 
	Buffer: PVOID, 
	BufferSize: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 361",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetInformationWorkerFactory(
	WorkerFactoryHandle: HANDLE, 
	WorkerFactoryInformationClass: WORKERFACTORYINFOCLASS, 
	WorkerFactoryInformation: PVOID, 
	WorkerFactoryInformationLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 433",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetSystemPowerState(
	SystemAction: POWER_ACTION, 
	LightestSystemState: SYSTEM_POWER_STATE, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 445",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateTimer2(
	TimerHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	Attributes: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 204",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAddAtomEx(
	AtomName: PWSTR, 
	Length: ULONG, 
	Atom: PUSHORT, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 105",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetInformationSymbolicLink(
	LinkHandle: HANDLE, 
	LinkInformationClass: SYMBOLIC_LINK_INFORMATION_CLASS, 
	LinkInformation: PVOID, 
	LinkInformationLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 428",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAlpcDisconnectPort(
	PortHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 134",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetIRTimer(
	TimerHandle: HANDLE, 
	DueTime: PLARGE_INTEGER, 
	Period: PLARGE_INTEGER, 
	ApcRoutine: PTIMER_APC_ROUTINE, 
	ApcContext: PVOID, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 420",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwRaiseHardError(
	ErrorStatus: NTSTATUS, 
	NumberOfParameters: ULONG, 
	UnicodeStringParameterMask: ULONG, 
	Parameters: PULONG_PTR, 
	ResponseOption: HARDERROR_RESPONSE_OPTION, 
	Response: PHARDERROR_RESPONSE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 373",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateTimer(
	TimerHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	TimerType: TIMER_TYPE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 203",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwOpenObjectAuditAlarm(
	SubsystemName: PUNICODE_STRING, 
	HandleId: PVOID, 
	ObjectTypeName: PUNICODE_STRING, 
	ObjectName: PUNICODE_STRING, 
	SecurityDescriptor: PSECURITY_DESCRIPTOR, 
	ClientToken: HANDLE, 
	DesiredAccess: ACCESS_MASK, 
	GrantedAccess: ACCESS_MASK, 
	Privileges: PPRIVILEGE_SET, 
	ObjectCreation: BOOLEAN, 
	AccessGranted: BOOLEAN, 
	GenerateOnClose: PBOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 304",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtTerminateProcess(
	ProcessHandle: HANDLE, 
	ExitStatus: NTSTATUS) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 44",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAlertMultipleThreadByThreadId(
	ThreadIds: PULONG, 
	Count: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 110",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtUnmapViewOfSection(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 42",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtFreezeRegistry() -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 247",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwPrepareEnlistment(
	EnlistmentHandle: HANDLE, 
	TmVirtualClock: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 321",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateTransactionManager(
	TmHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	LogFileName: PUNICODE_STRING, 
	CreateOptions: ULONG, 
	CommitStrength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 208",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtOpenCpuPartition(
	CpuPartitionHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 294",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCancelTimer2(
	TimerHandle: HANDLE, 
	CurrentState: PBOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 150",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwOpenTimer(
	TimerHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 314",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCommitRegistryTransaction(
	TransactionHandle: HANDLE, 
	TmVirtualClock: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 156",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCommitTransaction(
	TransactionHandle: HANDLE, 
	Wait: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 157",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryInformationToken(
	TokenHandle: HANDLE, 
	TokenInformationClass: TOKEN_INFORMATION_CLASS, 
	TokenInformation: PVOID, 
	TokenInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 33",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwRecoverResourceManager(
	ResourceManagerHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 377",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAdjustTokenClaimsAndDeviceGroups(
	TokenHandle: HANDLE, 
	ResetToDefault: BOOLEAN, 
	NewClaims: PTOKEN_CLAIMS, 
	NewDeviceGroups: PTOKEN_CLAIMS, 
	BufferLength: ULONG, 
	PreviousClaims: PTOKEN_CLAIMS, 
	PreviousDeviceGroups: PTOKEN_CLAIMS, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 109",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtReleaseWorkerFactoryWorker(
	WorkerFactoryHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 382",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtOpenSession(
	SessionHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 311",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetInformationToken(
	TokenHandle: HANDLE, 
	TokenInformationClass: TOKEN_INFORMATION_CLASS, 
	TokenInformation: PVOID, 
	TokenInformationLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 429",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwFilterTokenEx(
	ExistingTokenHandle: HANDLE, 
	Flags: ULONG, 
	SidsToDisable: PTOKEN_GROUPS, 
	PrivilegesToDelete: PTOKEN_PRIVILEGES, 
	RestrictedSids: PTOKEN_GROUPS, 
	SidsToDisable: PTOKEN_GROUPS, 
	NewTokenHandle: PHANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 238",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtDeleteValueKey(
	KeyHandle: HANDLE, 
	ValueName: PUNICODE_STRING) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 223",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryPerformanceCounter(
	PerformanceCounter: PLARGE_INTEGER, 
	PerformanceFrequency: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 49",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwOpenJobObject(
	JobHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 298",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtOpenEnlistment(
	EnlistmentHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	ResourceManagerHandle: HANDLE, 
	EnlistmentId: LUID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 295",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwRecoverTransactionManager(
	TransactionManagerHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 378",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateProcessStateChange(
	StateChangeHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	ProcessHandle: HANDLE, 
	ThreadHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 193",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetIoCompletion(
	IoCompletionHandle: HANDLE, 
	KeyContext: ULONG_PTR, 
	ApcContext: PVOID, 
	IoStatus: NTSTATUS, 
	Information: ULONG_PTR) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 435",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateKeyedEvent(
	KeyedEventHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 183",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAllocateUserPhysicalPagesEx(
	ProcessHandle: HANDLE, 
	NumberOfPages: PULONG_PTR, 
	UserPfnArray: PULONG_PTR, 
	ExtendedParameters: PULONG, 
	ParameterCount: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 118",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryIntervalProfile(
	Source: KPROFILE_SOURCE, 
	Interval: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 349",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwLoadDriver(
	DriverServiceName: PUNICODE_STRING) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 270",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetInformationObject(
	Handle: HANDLE, 
	ObjectInformationClass: OBJECT_INFORMATION_CLASS, 
	ObjectInformation: PVOID, 
	ObjectInformationLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 92",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateSectionEx(
	SectionHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	MaximumSize: PLARGE_INTEGER, 
	SectionPageProtection: ULONG, 
	AllocationAttributes: ULONG, 
	FileHandle: HANDLE, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 198",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtOpenTransactionManager(
	TmHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	LogFileName: PUNICODE_STRING, 
	TmIdentity: LUID, 
	OpenOptions: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 316",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtStopProfile(
	ProfileObject: HANDLE, 
	ProfileSource: KPROFILE_SOURCE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 459",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateUserProcess(
	ProcessHandle: PHANDLE, 
	ThreadHandle: PHANDLE, 
	ProcessDesiredAccess: ACCESS_MASK, 
	ThreadDesiredAccess: ACCESS_MASK, 
	ProcessObjectAttributes: POBJECT_ATTRIBUTES, 
	ThreadObjectAttributes: POBJECT_ATTRIBUTES, 
	ProcessFlags: ULONG, 
	ThreadFlags: ULONG, 
	ProcessParameters: PRTL_USER_PROCESS_PARAMETERS, 
	CreateInfo: PVOID, 
	AttributeList: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 209",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetSystemEnvironmentValueEx(
	VariableName: PUNICODE_STRING, 
	VariableValue: PWSTR, 
	VariableValueLength: USHORT, 
	Attributes: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 443",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtDuplicateObject(
	SourceProcessHandle: HANDLE, 
	SourceHandle: HANDLE, 
	TargetProcessHandle: HANDLE, 
	TargetHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	HandleAttributes: ULONG, 
	Options: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 60",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtNotifyChangeKey(
	KeyHandle: HANDLE, 
	Event: HANDLE, 
	ApcRoutine: PIO_APC_ROUTINE, 
	ApcContext: PVOID, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	CompletionFilter: ULONG, 
	WatchTree: BOOLEAN, 
	ChangeBuffer: PVOID, 
	BufferSize: ULONG, 
	Asynchronous: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 291",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAllocateLocallyUniqueId(
	LuId: PLUID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 115",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateResourceManager(
	ResourceManagerHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	TmHandle: HANDLE, 
	ResourceManagerGuid: PGUID, 
	CreateOptions: ULONG, 
	Description: PUNICODE_STRING) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 197",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryMutant(
	MutantHandle: HANDLE, 
	MutantInformationClass: MUTANT_INFORMATION_CLASS, 
	MutantInformation: PVOID, 
	MutantInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 354",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetVolumeInformationFile(
	FileHandle: HANDLE, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	FsInformation: PVOID, 
	Length: ULONG, 
	FsInformationClass: FSINFOCLASS) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 452",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwConnectPort(
	PortHandle: PHANDLE, 
	PortName: PUNICODE_STRING, 
	SecurityQos: PSECURITY_QUALITY_OF_SERVICE, 
	ClientView: PVOID, 
	RequiredServerSid: PSID, 
	ConnectData: PVOID, 
	DataLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 164",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwWaitLowEventPair(
	EventPairHandle: HANDLE, 
	Alertable: BOOLEAN, 
	Timeout: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 488",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtResetEvent(
	EventHandle: HANDLE, 
	PreviousState: PLONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 391",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryDefaultUILanguage(
	DefaultUILanguageId: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 68",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateSymbolicLinkObject(
	LinkHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	LinkTarget: PUNICODE_STRING) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 200",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetCachedSigningLevel2(
	FileHandle: HANDLE, 
	Flags: ULONG, 
	SigningLevel: ULONG, 
	Thumbprint: PUCHAR, 
	ThumbprintSize: ULONG, 
	SectionFlags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 409",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreatePagingFile(
	PageFileName: PUNICODE_STRING, 
	MinimumSize: PLARGE_INTEGER, 
	MaximumSize: PLARGE_INTEGER, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 188",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtGetDevicePowerState(
	DeviceHandle: HANDLE, 
	State: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 254",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwUnloadKey(
	TargetKey: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 474",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateProfileEx(
	ProfileHandle: PHANDLE, 
	Process: HANDLE, 
	RangeBase: PVOID, 
	RangeSize: SIZE_T, 
	BucketSize: ULONG, 
	ProfileInfo: PULONG, 
	Flags: ULONG, 
	Buffer: PULONG, 
	BufferSize: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 195",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAlpcCreatePort(
	PortHandle: PHANDLE, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	PortAttributes: PALPC_PORT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 125",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetEventEx(
	EventHandle: HANDLE, 
	EventNumber: LONG, 
	PreviousState: PLONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 417",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetIoCompletion(
	IoCompletionHandle: HANDLE, 
	KeyContext: ULONG_PTR, 
	ApcContext: PVOID, 
	IoStatus: NTSTATUS, 
	Information: ULONG_PTR) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 435",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateDirectoryObjectEx(
	DirectoryHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	ObjectName: PUNICODE_STRING, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 173",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryDebugFilterState(
	ComponentId: ULONG, 
	Level: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 332",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwPlugPlayControl(
	ControlCode: ULONG, 
	Buffer: PVOID, 
	BufferSize: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 317",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
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
	FileName: PUNICODE_STRING, 
	RestartScan: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 53",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryDefaultLocale(
	UserProfile: BOOLEAN, 
	DefaultLocaleId: PLCID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 21",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwTerminateJobObject(
	JobHandle: HANDLE, 
	ExitStatus: NTSTATUS) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 466",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryDebugFilterState(
	ComponentId: ULONG, 
	Level: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 332",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryBootEntryOrder(
	BootEntries: PULONG, 
	BootCount: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 330",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateWnfStateName(
	StateName: PWNF_STATE_NAME, 
	NameLifetime: ULONG, 
	DataScope: ULONG, 
	PermanentData: BOOLEAN, 
	TypeId: PWNF_TYPE_ID, 
	MaximumStateSize: ULONG, 
	SecurityDescriptor: PSECURITY_DESCRIPTOR) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 212",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryDirectoryObject(
	DirectoryHandle: HANDLE, 
	Buffer: PVOID, 
	BufferLength: ULONG, 
	ReturnSingleEntry: BOOLEAN, 
	RestartScan: BOOLEAN, 
	Context: PULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 334",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAllocateUuids(
	Time: PULONG, 
	TimeHi: PULONG, 
	Seq: PULONG, 
	NodeId: PUCHAR) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 119",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryKey(
	KeyHandle: HANDLE, 
	KeyInformationClass: KEY_INFORMATION_CLASS, 
	KeyInformation: PVOID, 
	Length: ULONG, 
	ResultLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 22",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAddAtom(
	AtomName: PWSTR, 
	Length: ULONG, 
	Atom: PUSHORT) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 71",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwNotifyChangeDirectoryFile(
	FileHandle: HANDLE, 
	Event: HANDLE, 
	ApcRoutine: PIO_APC_ROUTINE, 
	ApcContext: PVOID, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	Buffer: PVOID, 
	Length: ULONG, 
	CompletionFilter: ULONG, 
	WatchTree: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 289",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryInformationResourceManager(
	ResourceManagerHandle: HANDLE, 
	ResourceManagerInformationClass: RESOURCEMANAGER_INFORMATION_CLASS, 
	ResourceManagerInformation: PVOID, 
	ResourceManagerInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 344",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateKey(
	KeyHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	TitleIndex: ULONG, 
	Class: PUNICODE_STRING, 
	CreateOptions: ULONG, 
	Disposition: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 29",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtOpenFile(
	FileHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	ShareAccess: ULONG, 
	OpenOptions: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 51",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAccessCheck(
	SecurityDescriptor: PSECURITY_DESCRIPTOR, 
	ClientToken: HANDLE, 
	DesiredAccess: ACCESS_MASK, 
	GenericMapping: PGENERIC_MAPPING, 
	PrivilegeSet: PPRIVILEGE_SET, 
	PrivilegeSetLength: PULONG, 
	GrantedAccess: PACCESS_MASK, 
	AccessStatus: PBOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 0",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetQuotaInformationFile(
	FileHandle: HANDLE, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	Buffer: PVOID, 
	Length: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 440",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetInformationFile(
	FileHandle: HANDLE, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	FileInformation: PVOID, 
	Length: ULONG, 
	FileInformationClass: FILE_INFORMATION_CLASS) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 39",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwWaitForMultipleObjects(
	Count: ULONG, 
	Handles: PHANDLE, 
	WaitType: WAIT_TYPE, 
	Alertable: BOOLEAN, 
	Timeout: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 91",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtUnloadKey2(
	TargetKey: POBJECT_ATTRIBUTES, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 475",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwGetCurrentProcessorNumberEx(
	ProcNumber: PPROCESSOR_NUMBER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 253",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtRequestWaitReplyPort(
	PortHandle: HANDLE, 
	RequestMessage: PVOID, 
	ReplyMessage: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 34",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwOpenSemaphore(
	SemaphoreHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 310",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateEnclave(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	ZeroBits: SIZE_T, 
	Size: SIZE_T, 
	EnclaveType: ULONG, 
	EnclaveInformation: PVOID, 
	InformationLength: ULONG, 
	EnclaveInformation: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 174",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAccessCheckByTypeAndAuditAlarm(
	SubsystemName: PUNICODE_STRING, 
	HandleId: PVOID, 
	ObjectTypeName: PUNICODE_STRING, 
	ObjectName: PUNICODE_STRING, 
	SecurityDescriptor: PSECURITY_DESCRIPTOR, 
	ClientToken: HANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectTypeList: POBJECT_TYPE_LIST, 
	ObjectTypeListLength: ULONG, 
	GenericMapping: PGENERIC_MAPPING, 
	ObjectCreation: BOOLEAN, 
	GrantedAccess: PACCESS_MASK, 
	AccessStatus: PBOOLEAN, 
	GenerateOnClose: PBOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 89",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAlpcRevokeSecurityContext(
	PortHandle: HANDLE, 
	Flags: ULONG, 
	Context: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 141",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCompressKey(
	KeyHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 163",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtOpenEvent(
	EventHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 64",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryVolumeInformationFile(
	FileHandle: HANDLE, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	FsInformation: PVOID, 
	Length: ULONG, 
	FsInformationClass: FSINFOCLASS) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 73",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtRollbackTransaction(
	TransactionHandle: HANDLE, 
	Wait: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 399",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtOpenRegistryTransaction(
	TransactionHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 308",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwPowerInformation(
	InformationLevel: POWER_INFORMATION_LEVEL, 
	InputBuffer: PVOID, 
	InputBufferLength: ULONG, 
	OutputBuffer: PVOID, 
	OutputBufferLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 95",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtLoadKey2(
	TargetKey: POBJECT_ATTRIBUTES, 
	SourceFile: POBJECT_ATTRIBUTES, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 273",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueueApcThreadEx2(
	ThreadHandle: HANDLE, 
	Reserved: HANDLE, 
	ApcRoutine: PKNORMAL_ROUTINE, 
	ApcArgument1: PVOID, 
	ApcArgument2: PVOID, 
	ApcArgument3: PVOID, 
	Flags: ULONG, 
	Reserved2: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 371",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetInformationFile(
	FileHandle: HANDLE, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	FileInformation: PVOID, 
	Length: ULONG, 
	FileInformationClass: FILE_INFORMATION_CLASS) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 39",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwPrivilegeCheck(
	ClientToken: HANDLE, 
	RequiredPrivileges: PPRIVILEGE_SET, 
	SubjectContextLocked: PBOOLEAN, 
	Result: PBOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 322",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwResetEvent(
	EventHandle: HANDLE, 
	PreviousState: PLONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 391",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwMapViewOfSectionEx(
	SectionHandle: HANDLE, 
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	ZeroBits: ULONG_PTR, 
	CommitSize: SIZE_T, 
	SectionOffset: PLARGE_INTEGER, 
	ViewSize: PSIZE_T, 
	InheritDisposition: ULONG, 
	AllocationType: ULONG, 
	Win32Protect: ULONG, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 286",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwReplyWaitReplyPort(
	PortHandle: HANDLE, 
	ReplyMessage: PVOID, 
	ReceiveMessage: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 389",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAlpcCreateSecurityContext(
	PortHandle: HANDLE, 
	Flags: ULONG, 
	SecurityAttributes: PALPC_SECURITY_ATTRIBUTES, 
	Context: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 129",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwGetMUIRegistryInfo(
	Flags: ULONG, 
	Data: PVOID, 
	DataSize: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 255",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateSemaphore(
	SemaphoreHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	InitialCount: LONG, 
	MaximumCount: LONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 199",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwRegisterProtocolAddressInformation(
	ResourceManagerHandle: HANDLE, 
	ProtocolName: PUNICODE_STRING, 
	CreateOptions: ULONG, 
	Persistence: ULONG, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 379",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQuerySystemInformationEx(
	SystemInformationClass: SYSTEM_INFORMATION_CLASS, 
	InputBuffer: PVOID, 
	InputBufferLength: ULONG, 
	SystemInformation: PVOID, 
	SystemInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 366",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtUnsubscribeWnfStateChange(
	ChangeStamp: PWNF_CHANGE_STAMP) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 480",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreatePartition(
	PartitionHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 189",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwLockFile(
	FileHandle: HANDLE, 
	Event: HANDLE, 
	ApcRoutine: PIO_APC_ROUTINE, 
	ApcContext: PVOID, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	ByteOffset: PLARGE_INTEGER, 
	Length: PLARGE_INTEGER, 
	Key: ULONG, 
	FailImmediately: BOOLEAN, 
	ExclusiveLock: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 276",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAllocateLocallyUniqueId(
	LuId: PLUID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 115",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAlpcSetInformation(
	PortHandle: HANDLE, 
	AlpcInformationClass: ALPC_INFORMATION_CLASS, 
	AlpcInformation: PVOID, 
	AlpcInformationLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 143",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtThawRegistry() -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 468",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetDriverEntryOrder(
	LoadOrder: PULONG, 
	Number: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 415",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtDeleteKey(
	KeyHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 220",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateResourceManager(
	ResourceManagerHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	TmHandle: HANDLE, 
	ResourceManagerGuid: PGUID, 
	CreateOptions: ULONG, 
	Description: PUNICODE_STRING) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 197",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAlpcCreatePortSection(
	PortHandle: HANDLE, 
	Flags: ULONG, 
	SectionHandle: HANDLE, 
	SectionSize: SIZE_T, 
	AlpcSectionHandle: PALPC_HANDLE, 
	SectionView: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 126",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtFlushInstallUILanguage(
	InstallUILanguageId: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 240",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSaveMergedKeys(
	HighPreparedKeyHandle: HANDLE, 
	LowPreparedKeyHandle: HANDLE, 
	FileHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 403",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwOpenPrivateNamespace(
	NamespaceHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 306",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetHighEventPair(
	EventPairHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 418",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtLoadKey(
	TargetKey: POBJECT_ATTRIBUTES, 
	SourceFile: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 272",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryInformationJobObject(
	JobHandle: HANDLE, 
	JobObjectInformationClass: JOBOBJECTINFOCLASS, 
	JobObjectInformation: PVOID, 
	JobObjectInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 342",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtWorkerFactoryWorkerReady(
	WorkerFactoryHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 1",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetSystemEnvironmentValueEx(
	VariableName: PUNICODE_STRING, 
	VariableValue: PWSTR, 
	VariableValueLength: USHORT, 
	Attributes: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 443",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtDeleteWnfStateName(
	StateName: PWNF_STATE_NAME) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 225",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtShutdownWorkerFactory(
	WorkerFactoryHandle: HANDLE, 
	Timeout: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 455",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwReplyWaitReceivePort(
	PortHandle: HANDLE, 
	ReplyMessage: PVOID, 
	ReceiveMessage: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 11",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtOpenThreadToken(
	ThreadHandle: HANDLE, 
	DesiredAccess: ACCESS_MASK, 
	OpenAsSelf: BOOLEAN, 
	TokenHandle: PHANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 36",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtResumeProcess(
	ProcessHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 394",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwWaitForDebugEvent(
	DebugObjectHandle: HANDLE, 
	Alertable: BOOLEAN, 
	Timeout: PLARGE_INTEGER, 
	WaitStateChange: PDBGUI_WAIT_STATE_CHANGE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 484",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtRenameKey(
	KeyHandle: HANDLE, 
	NewName: PUNICODE_STRING) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 385",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateCrossVmEvent(
	EventHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	VmHandle: HANDLE, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 169",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryInformationAtom(
	AtomHandle: HANDLE, 
	AtomInformationClass: ATOM_INFORMATION_CLASS, 
	AtomInformation: PVOID, 
	AtomInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 338",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAllocateUuids(
	Time: PULONG, 
	TimeHi: PULONG, 
	Seq: PULONG, 
	NodeId: PUCHAR) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 119",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAlpcCreateSectionView(
	PortHandle: HANDLE, 
	Flags: ULONG, 
	SectionView: PALPC_SECTION_VIEW) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 128",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetDefaultUILanguage(
	DefaultUILanguageId: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 414",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwOpenFile(
	FileHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	ShareAccess: ULONG, 
	OpenOptions: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 51",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSystemDebugControl(
	ControlCode: ULONG, 
	InputBuffer: PVOID, 
	InputBufferLength: ULONG, 
	OutputBuffer: PVOID, 
	OutputBufferLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 464",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCloseObjectAuditAlarm(
	SubsystemName: PUNICODE_STRING, 
	HandleId: PVOID, 
	GenerateOnClose: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 59",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwDeleteFile(
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 219",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAllocateUserPhysicalPages(
	ProcessHandle: HANDLE, 
	NumberOfPages: PULONG_PTR, 
	UserPfnArray: PULONG_PTR) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 117",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAccessCheckByTypeResultListAndAuditAlarmByHandle(
	SubsystemName: PUNICODE_STRING, 
	HandleId: PVOID, 
	ClientToken: HANDLE, 
	ObjectTypeName: PUNICODE_STRING, 
	ObjectName: PUNICODE_STRING, 
	SecurityDescriptor: PSECURITY_DESCRIPTOR, 
	DesiredAccess: ACCESS_MASK, 
	ObjectTypeList: POBJECT_TYPE_LIST, 
	ObjectTypeListLength: ULONG, 
	GenericMapping: PGENERIC_MAPPING, 
	ObjectCreation: BOOLEAN, 
	GrantedAccessList: PULONG, 
	AccessStatusList: PULONG, 
	GenerateOnClose: PBOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 102",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetInformationIoRing(
	IoRingHandle: HANDLE, 
	InformationClass: ULONG, 
	Information: PVOID, 
	InformationLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 424",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwStopProfile(
	ProfileObject: HANDLE, 
	ProfileSource: KPROFILE_SOURCE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 459",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtLoadKey3(
	TargetKey: POBJECT_ATTRIBUTES, 
	SourceFile: POBJECT_ATTRIBUTES, 
	Flags: ULONG, 
	Reserved: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 274",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetUuidSeed(
	Seed: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 451",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCopyFileChunk(
	SourceFileHandle: HANDLE, 
	TargetFileHandle: HANDLE, 
	SourceOffset: PLARGE_INTEGER, 
	TargetOffset: PLARGE_INTEGER, 
	Length: SIZE_T, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 167",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwReadRequestData(
	PortHandle: HANDLE, 
	PortMessage: PVOID, 
	DataIndex: ULONG, 
	Buffer: PVOID, 
	BufferSize: ULONG, 
	OutputBufferSize: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 84",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQuerySection(
	SectionHandle: HANDLE, 
	SectionInformationClass: SECTION_INFORMATION_CLASS, 
	SectionInformation: PVOID, 
	SectionInformationLength: SIZE_T, 
	ReturnLength: PSIZE_T) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 81",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwChangeThreadState(
	ThreadHandle: HANDLE, 
	TargetState: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 153",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtReadRequestData(
	PortHandle: HANDLE, 
	PortMessage: PVOID, 
	DataIndex: ULONG, 
	Buffer: PVOID, 
	BufferSize: ULONG, 
	OutputBufferSize: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 84",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwDeleteValueKey(
	KeyHandle: HANDLE, 
	ValueName: PUNICODE_STRING) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 223",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAccessCheckByTypeAndAuditAlarm(
	SubsystemName: PUNICODE_STRING, 
	HandleId: PVOID, 
	ObjectTypeName: PUNICODE_STRING, 
	ObjectName: PUNICODE_STRING, 
	SecurityDescriptor: PSECURITY_DESCRIPTOR, 
	ClientToken: HANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectTypeList: POBJECT_TYPE_LIST, 
	ObjectTypeListLength: ULONG, 
	GenericMapping: PGENERIC_MAPPING, 
	ObjectCreation: BOOLEAN, 
	GrantedAccess: PACCESS_MASK, 
	AccessStatus: PBOOLEAN, 
	GenerateOnClose: PBOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 89",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwReplyWaitReceivePortEx(
	PortHandle: HANDLE, 
	ReplyMessage: PVOID, 
	ReceiveMessage: PVOID, 
	Timeout: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 43",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAlpcImpersonateClientOfPort(
	PortHandle: HANDLE, 
	PortMessage: PVOID, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 136",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQuerySystemEnvironmentValueEx(
	VariableName: PUNICODE_STRING, 
	VariableValue: PWSTR, 
	VariableValueLength: PUSHORT, 
	Attributes: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 365",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCancelWaitCompletionPacket(
	WaitCompletionPacketHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 151",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAcquireCrossVmMutant(
	MutantHandle: HANDLE, 
	VmHandle: HANDLE, 
	Timeout: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 103",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtUmsThreadYield(
	Reason: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 472",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetInformationIoRing(
	IoRingHandle: HANDLE, 
	InformationClass: ULONG, 
	Information: PVOID, 
	InformationLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 424",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtGetCompleteWnfStateSubscription(
	StateName: PWNF_STATE_NAME, 
	ChangeStamp: PWNF_CHANGE_STAMP, 
	Subscription: PWNF_USER_SUBSCRIPTION) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 250",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAcquireCrossVmMutant(
	MutantHandle: HANDLE, 
	VmHandle: HANDLE, 
	Timeout: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 103",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwShutdownWorkerFactory(
	WorkerFactoryHandle: HANDLE, 
	Timeout: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 455",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSystemDebugControl(
	ControlCode: ULONG, 
	InputBuffer: PVOID, 
	InputBufferLength: ULONG, 
	OutputBuffer: PVOID, 
	OutputBufferLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 464",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAccessCheck(
	SecurityDescriptor: PSECURITY_DESCRIPTOR, 
	ClientToken: HANDLE, 
	DesiredAccess: ACCESS_MASK, 
	GenericMapping: PGENERIC_MAPPING, 
	PrivilegeSet: PPRIVILEGE_SET, 
	PrivilegeSetLength: PULONG, 
	GrantedAccess: PACCESS_MASK, 
	AccessStatus: PBOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 0",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtProtectVirtualMemory(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	RegionSize: PSIZE_T, 
	NewProtect: ULONG, 
	OldProtect: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 80",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwGetNlsSectionPtr(
	SectionType: ULONG, 
	SectionId: ULONG, 
	SectionData: PVOID, 
	SectionSize: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 258",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwTraceEvent(
	TraceHandle: HANDLE, 
	Flags: ULONG, 
	FieldSize: ULONG, 
	Fields: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 94",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAlpcSendWaitReceivePort(
	PortHandle: HANDLE, 
	Flags: ULONG, 
	SendMessage: PVOID, 
	SendAttributes: PALPC_MESSAGE_ATTRIBUTES, 
	ReceiveMessage: PVOID, 
	ReceiveMessageLength: PULONG, 
	ReceiveAttributes: PALPC_MESSAGE_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 142",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtMapViewOfSection(
	SectionHandle: HANDLE, 
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	ZeroBits: ULONG_PTR, 
	CommitSize: SIZE_T, 
	SectionOffset: PLARGE_INTEGER, 
	ViewSize: PSIZE_T, 
	InheritDisposition: ULONG, 
	AllocationType: ULONG, 
	Win32Protect: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 40",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtReplyWaitReceivePortEx(
	PortHandle: HANDLE, 
	ReplyMessage: PVOID, 
	ReceiveMessage: PVOID, 
	Timeout: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 43",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtFilterTokenEx(
	ExistingTokenHandle: HANDLE, 
	Flags: ULONG, 
	SidsToDisable: PTOKEN_GROUPS, 
	PrivilegesToDelete: PTOKEN_PRIVILEGES, 
	RestrictedSids: PTOKEN_GROUPS, 
	SidsToDisable: PTOKEN_GROUPS, 
	NewTokenHandle: PHANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 238",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwOpenEvent(
	EventHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 64",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtReadOnlyEnlistment(
	EnlistmentHandle: HANDLE, 
	TmVirtualClock: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 374",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetSystemTime(
	SystemTime: PLARGE_INTEGER, 
	PreviousTime: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 446",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAlpcDisconnectPort(
	PortHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 134",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtOpenKey(
	KeyHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 18",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwFlushWriteBuffer() -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 245",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetEventBoostPriority(
	EventHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 45",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtRecoverEnlistment(
	EnlistmentHandle: HANDLE, 
	EnlistmentKey: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 376",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwUnloadKey2(
	TargetKey: POBJECT_ATTRIBUTES, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 475",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateProcessStateChange(
	StateChangeHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	ProcessHandle: HANDLE, 
	ThreadHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 193",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryObject(
	Handle: HANDLE, 
	ObjectInformationClass: OBJECT_INFORMATION_CLASS, 
	ObjectInformation: PVOID, 
	ObjectInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 16",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCallbackReturn(
	Result: PVOID, 
	ResultLength: ULONG, 
	Status: NTSTATUS) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 5",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwTerminateThread(
	ThreadHandle: HANDLE, 
	ExitStatus: NTSTATUS) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 83",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAlertThread(
	ThreadHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 112",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwClose(
	Handle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 15",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwUnsubscribeWnfStateChange(
	ChangeStamp: PWNF_CHANGE_STAMP) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 480",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwIsUILanguageComitted() -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 268",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwLoadKey3(
	TargetKey: POBJECT_ATTRIBUTES, 
	SourceFile: POBJECT_ATTRIBUTES, 
	Flags: ULONG, 
	Reserved: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 274",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateThread(
	ThreadHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	ProcessHandle: HANDLE, 
	ClientId: PCLIENT_ID, 
	ThreadContext: PCONTEXT, 
	InitialTeb: PUSER_STACK, 
	CreateSuspended: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 78",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryMutant(
	MutantHandle: HANDLE, 
	MutantInformationClass: MUTANT_INFORMATION_CLASS, 
	MutantInformation: PVOID, 
	MutantInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 354",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtEnumerateTransactionObject(
	EnumerateHandle: HANDLE, 
	TransactionObject: PULONG, 
	ObjectType: ULONG, 
	Count: ULONG, 
	ReturnCount: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 234",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtEnumerateBootEntries(
	Buffer: PVOID, 
	BufferLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 231",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateUserProcess(
	ProcessHandle: PHANDLE, 
	ThreadHandle: PHANDLE, 
	ProcessDesiredAccess: ACCESS_MASK, 
	ThreadDesiredAccess: ACCESS_MASK, 
	ProcessObjectAttributes: POBJECT_ATTRIBUTES, 
	ThreadObjectAttributes: POBJECT_ATTRIBUTES, 
	ProcessFlags: ULONG, 
	ThreadFlags: ULONG, 
	ProcessParameters: PRTL_USER_PROCESS_PARAMETERS, 
	CreateInfo: PVOID, 
	AttributeList: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 209",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtGetNlsSectionPtr(
	SectionType: ULONG, 
	SectionId: ULONG, 
	SectionData: PVOID, 
	SectionSize: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 258",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryMultipleValueKey(
	KeyHandle: HANDLE, 
	ValueEntries: PKEY_VALUE_ENTRY, 
	ValueCount: ULONG, 
	RequiredBufferSize: PULONG, 
	ValueBufferSize: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 353",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwDeviceIoControlFile(
	FileHandle: HANDLE, 
	Event: HANDLE, 
	ApcRoutine: PIO_APC_ROUTINE, 
	ApcContext: PVOID, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	IoControlCode: ULONG, 
	InputBuffer: PVOID, 
	InputBufferLength: ULONG, 
	OutputBuffer: PVOID, 
	OutputBufferLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 7",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQuerySystemEnvironmentValue(
	VariableName: PUNICODE_STRING, 
	VariableValue: PWSTR, 
	VariableValueLength: USHORT, 
	ReturnLength: PUSHORT) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 364",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCommitEnlistment(
	EnlistmentHandle: HANDLE, 
	TmVirtualClock: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 155",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateTimer(
	TimerHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	TimerType: TIMER_TYPE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 203",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCloseObjectAuditAlarm(
	SubsystemName: PUNICODE_STRING, 
	HandleId: PVOID, 
	GenerateOnClose: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 59",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateEvent(
	EventHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	EventType: EVENT_TYPE, 
	InitialState: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 72",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetInformationDebugObject(
	DebugObjectHandle: HANDLE, 
	DebugObjectInformationClass: DEBUGOBJECTINFOCLASS, 
	DebugObjectInformation: PVOID, 
	DebugObjectInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 422",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwEnumerateDriverEntries(
	LoadOrder: PULONG, 
	Number: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 232",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCancelTimer(
	TimerHandle: HANDLE, 
	CurrentState: PBOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 97",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwFreezeRegistry() -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 247",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwDirectGraphicsCall(
	Command: ULONG, 
	InputBuffer: PVOID, 
	InputBufferSize: ULONG, 
	OutputBuffer: PVOID, 
	OutputBufferSize: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 226",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtReadVirtualMemoryEx(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	Buffer: PVOID, 
	BufferSize: SIZE_T, 
	NumberOfBytesRead: PSIZE_T, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 375",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtOpenSymbolicLinkObject(
	LinkHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 312",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryInformationTransaction(
	TransactionHandle: HANDLE, 
	TransactionInformationClass: TRANSACTION_INFORMATION_CLASS, 
	TransactionInformation: PVOID, 
	TransactionInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 345",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetEvent(
	EventHandle: HANDLE, 
	PreviousState: PLONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 14",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtDrawText(
	WindowHandle: HANDLE, 
	Text: PUNICODE_STRING, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 229",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetTimerResolution(
	DesiredResolution: ULONG, 
	SetResolution: BOOLEAN, 
	ActualResolution: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 450",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwRequestPort(
	PortHandle: HANDLE, 
	PortMessage: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 390",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCompressKey(
	KeyHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 163",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryInformationWorkerFactory(
	WorkerFactoryHandle: HANDLE, 
	WorkerFactoryInformationClass: WORKERFACTORYINFOCLASS, 
	WorkerFactoryInformation: PVOID, 
	WorkerFactoryInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 347",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateMailslotFile(
	FileHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	CreateOptions: ULONG, 
	MailslotQuota: ULONG, 
	MaximumMessageSize: ULONG, 
	ReadTimeout: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 185",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtOpenSemaphore(
	SemaphoreHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 310",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwReadFile(
	FileHandle: HANDLE, 
	Event: HANDLE, 
	ApcRoutine: PIO_APC_ROUTINE, 
	ApcContext: PVOID, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	Buffer: PVOID, 
	Length: ULONG, 
	ByteOffset: PLARGE_INTEGER, 
	Key: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 6",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateCrossVmMutant(
	MutantHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	VmHandle: HANDLE, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 170",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateProcessEx(
	ProcessHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	ParentProcess: HANDLE, 
	Flags: ULONG, 
	SectionHandle: HANDLE, 
	DebugPort: HANDLE, 
	ExceptionPort: HANDLE, 
	JobMemberLevel: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 77",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwPrePrepareComplete(
	EnlistmentHandle: HANDLE, 
	TmVirtualClock: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 318",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwGetCurrentProcessorNumber() -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 252",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtPowerInformation(
	InformationLevel: POWER_INFORMATION_LEVEL, 
	InputBuffer: PVOID, 
	InputBufferLength: ULONG, 
	OutputBuffer: PVOID, 
	OutputBufferLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 95",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAlpcOpenSenderProcess(
	ProcessHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	PortHandle: HANDLE, 
	PortMessage: PVOID, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 137",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAlpcConnectPortEx(
	PortHandle: PHANDLE, 
	PortName: PUNICODE_STRING, 
	PortAttributes: PALPC_PORT_ATTRIBUTES, 
	ConnectionPort: PALPC_HANDLE, 
	Flags: ULONG, 
	RequiredServerSid: PSID, 
	MessageAttributes: PALPC_MESSAGE_ATTRIBUTES, 
	ConnectData: PVOID, 
	ConnectDataLength: PULONG, 
	ReceiveMessage: PVOID, 
	ReceiveMessageLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 124",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetHighWaitLowEventPair(
	EventPairHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 419",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetInformationVirtualMemory(
	ProcessHandle: HANDLE, 
	VirtualMemoryInformationClass: VIRTUAL_MEMORY_INFORMATION_CLASS, 
	VirtualMemoryInformation: PVOID, 
	VirtualMemoryInformationLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 432",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCompactKeys(
	Count: ULONG, 
	KeyHandles: PHANDLE, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 158",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtListenPort(
	PortHandle: HANDLE, 
	ConnectionRequest: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 269",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtPulseEvent(
	EventHandle: HANDLE, 
	PreviousState: PLONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 328",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetLowEventPair(
	EventPairHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 438",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSaveMergedKeys(
	HighPreparedKeyHandle: HANDLE, 
	LowPreparedKeyHandle: HANDLE, 
	FileHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 403",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetTimerResolution(
	DesiredResolution: ULONG, 
	SetResolution: BOOLEAN, 
	ActualResolution: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 450",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetLowWaitHighEventPair(
	EventPairHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 439",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtTestAlert() -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 467",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSuspendProcess(
	ProcessHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 462",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtOpenThreadTokenEx(
	ThreadHandle: HANDLE, 
	DesiredAccess: ACCESS_MASK, 
	OpenAsSelf: BOOLEAN, 
	HandleAttributes: ULONG, 
	TokenHandle: PHANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 47",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCompareTokens(
	FirstTokenHandle: HANDLE, 
	SecondTokenHandle: HANDLE, 
	Equal: PBOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 161",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtEnumerateKey(
	KeyHandle: HANDLE, 
	Index: ULONG, 
	KeyInformationClass: KEY_INFORMATION_CLASS, 
	KeyInformation: PVOID, 
	Length: ULONG, 
	ResultLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 50",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetInformationEnlistment(
	EnlistmentHandle: HANDLE, 
	EnlistmentInformationClass: ENLISTMENT_INFORMATION_CLASS, 
	EnlistmentInformation: PVOID, 
	EnlistmentInformationLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 423",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryIoRingCapabilities(
	Capabilities: PIORING_CAPABILITIES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 351",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetDebugFilterState(
	ComponentId: ULONG, 
	Level: ULONG, 
	State: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 411",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAlpcCreatePort(
	PortHandle: PHANDLE, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	PortAttributes: PALPC_PORT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 125",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueueApcThread(
	ThreadHandle: HANDLE, 
	ApcRoutine: PIO_APC_ROUTINE, 
	ApcArgument1: PVOID, 
	ApcArgument2: PVOID, 
	ApcArgument3: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 69",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryQuotaInformationFile(
	FileHandle: HANDLE, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	Buffer: PVOID, 
	Length: ULONG, 
	ReturnSingleEntry: BOOLEAN, 
	SidList: PVOID, 
	SidListLength: ULONG, 
	StartSidIndex: PULONG, 
	RestartScan: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 358",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCopyFileChunk(
	SourceFileHandle: HANDLE, 
	TargetFileHandle: HANDLE, 
	SourceOffset: PLARGE_INTEGER, 
	TargetOffset: PLARGE_INTEGER, 
	Length: SIZE_T, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 167",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwManagePartition(
	PartitionHandle: HANDLE, 
	Operation: ULONG, 
	Parameters: PVOID, 
	ParametersSize: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 283",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwExtendSection(
	SectionHandle: HANDLE, 
	SectionSize: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 235",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtOpenKeyEx(
	KeyHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	OpenOptions: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 299",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtTraceEvent(
	TraceHandle: HANDLE, 
	Flags: ULONG, 
	FieldSize: ULONG, 
	Fields: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 94",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAddAtomEx(
	AtomName: PWSTR, 
	Length: ULONG, 
	Atom: PUSHORT, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 105",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwPropagationFailed(
	ResourceManagerHandle: HANDLE, 
	PropagationId: ULONG, 
	PropagationStatus: NTSTATUS, 
	PropagationCount: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 326",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwGetCompleteWnfStateSubscription(
	StateName: PWNF_STATE_NAME, 
	ChangeStamp: PWNF_CHANGE_STAMP, 
	Subscription: PWNF_USER_SUBSCRIPTION) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 250",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetInformationCpuPartition(
	CpuPartitionHandle: HANDLE, 
	CpuPartitionInformationClass: CPU_PARTITION_INFORMATION_CLASS, 
	CpuPartitionInformation: PVOID, 
	CpuPartitionInformationLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 421",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAllocateReserveObject(
	ReserveObject: PHANDLE, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	Type: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 116",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtUnlockFile(
	FileHandle: HANDLE, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	ByteOffset: PLARGE_INTEGER, 
	Length: PLARGE_INTEGER, 
	Key: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 477",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetTimerEx(
	TimerHandle: HANDLE, 
	DueTime: PLARGE_INTEGER, 
	Period: PLARGE_INTEGER, 
	ApcRoutine: PTIMER_APC_ROUTINE, 
	ApcContext: PVOID, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 449",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateWaitCompletionPacket(
	WaitCompletionPacketHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 210",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAlpcCreateResourceReserve(
	PortHandle: HANDLE, 
	Flags: ULONG, 
	MessageSize: ULONG, 
	ActualMessageSize: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 127",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateCpuPartition(
	CpuPartitionHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 168",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAlpcImpersonateClientContainerOfPort(
	PortHandle: HANDLE, 
	PortMessage: PVOID, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 135",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtReadFileScatter(
	FileHandle: HANDLE, 
	Event: HANDLE, 
	ApcRoutine: PIO_APC_ROUTINE, 
	ApcContext: PVOID, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	SegmentArray: PFILE_SEGMENT_ELEMENT, 
	Length: ULONG, 
	ByteOffset: PLARGE_INTEGER, 
	Key: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 46",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryInformationProcess(
	ProcessHandle: HANDLE, 
	ProcessInformationClass: PROCESSINFOCLASS, 
	ProcessInformation: PVOID, 
	ProcessInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 25",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtThawTransactions(
	TmHandle: PHANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 469",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtYieldExecution() -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 70",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetTimerEx(
	TimerHandle: HANDLE, 
	DueTime: PLARGE_INTEGER, 
	Period: PLARGE_INTEGER, 
	ApcRoutine: PTIMER_APC_ROUTINE, 
	ApcContext: PVOID, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 449",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryInformationPort(
	PortHandle: HANDLE, 
	PortInformationClass: PORT_INFORMATION_CLASS, 
	PortInformation: PVOID, 
	Length: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 343",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateNamedPipeFile(
	FileHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
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
	DefaultTimeout: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 187",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwRenameKey(
	KeyHandle: HANDLE, 
	NewName: PUNICODE_STRING) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 385",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAlpcQueryInformation(
	PortHandle: HANDLE, 
	AlpcInformationClass: ALPC_INFORMATION_CLASS, 
	AlpcInformation: PVOID, 
	AlpcInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 139",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateToken(
	TokenHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	Type: TOKEN_TYPE, 
	AuthenticationId: PLUID, 
	ExpirationTime: PLARGE_INTEGER, 
	User: PTOKEN_USER, 
	Groups: PTOKEN_GROUPS, 
	Privileges: PTOKEN_PRIVILEGES, 
	Owner: PTOKEN_OWNER, 
	PrimaryGroup: PTOKEN_PRIMARY_GROUP, 
	DefaultDacl: PTOKEN_DEFAULT_DACL, 
	Source: PTOKEN_SOURCE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 205",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtLockRegistryKey(
	KeyHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 278",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtImpersonateThread(
	ServerThreadHandle: HANDLE, 
	ClientThreadHandle: HANDLE, 
	SecurityQos: PSECURITY_QUALITY_OF_SERVICE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 262",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwWorkerFactoryWorkerReady(
	WorkerFactoryHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 1",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtConvertBetweenAuxiliaryCounterAndPerformanceCounter(
	AuxiliaryCounterValue: PULONGLONG, 
	PerformanceCounterValue: PULONGLONG, 
	ConversionConstant: PULONGLONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 166",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetInformationToken(
	TokenHandle: HANDLE, 
	TokenInformationClass: TOKEN_INFORMATION_CLASS, 
	TokenInformation: PVOID, 
	TokenInformationLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 429",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCancelSynchronousIoFile(
	ThreadHandle: HANDLE, 
	IoStatusBlock: PIO_STATUS_BLOCK) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 149",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCallEnclave(
	Routine: PVOID, 
	Parameter: PVOID, 
	WaitForThread: BOOLEAN, 
	Result: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 147",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwEnumerateBootEntries(
	Buffer: PVOID, 
	BufferLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 231",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryInformationToken(
	TokenHandle: HANDLE, 
	TokenInformationClass: TOKEN_INFORMATION_CLASS, 
	TokenInformation: PVOID, 
	TokenInformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 33",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwDelayExecution(
	Alertable: BOOLEAN, 
	DelayInterval: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 52",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryDirectoryFileEx(
	FileHandle: HANDLE, 
	Event: HANDLE, 
	ApcRoutine: PIO_APC_ROUTINE, 
	ApcContext: PVOID, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	FileInformation: PVOID, 
	Length: ULONG, 
	FileInformationClass: FILE_INFORMATION_CLASS, 
	ReturnSingleEntry: BOOLEAN, 
	FileName: PUNICODE_STRING, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 333",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetEaFile(
	FileHandle: HANDLE, 
	IoStatusBlock: PIO_STATUS_BLOCK, 
	Buffer: PVOID, 
	Length: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 416",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateDebugObject(
	DebugObjectHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 171",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSinglePhaseReject(
	EnlistmentHandle: HANDLE, 
	TmVirtualClock: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 457",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtDeleteAtom(
	Atom: USHORT) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 216",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtRevertContainerImpersonation() -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 395",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateKeyTransacted(
	KeyHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	TitleIndex: ULONG, 
	Class: PUNICODE_STRING, 
	CreateOptions: ULONG, 
	TransactionHandle: HANDLE, 
	Disposition: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 182",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwCreateCrossVmMutant(
	MutantHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	VmHandle: HANDLE, 
	Flags: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 170",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwDuplicateObject(
	SourceProcessHandle: HANDLE, 
	SourceHandle: HANDLE, 
	TargetProcessHandle: HANDLE, 
	TargetHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	HandleAttributes: ULONG, 
	Options: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 60",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCancelTimer(
	TimerHandle: HANDLE, 
	CurrentState: PBOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 97",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtRegisterThreadTerminatePort(
	PortHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 380",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwAlertResumeThread(
	ThreadHandle: HANDLE, 
	PreviousSuspendCount: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 111",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtImpersonateClientOfPort(
	PortHandle: HANDLE, 
	PortMessage: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 31",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtPrePrepareComplete(
	EnlistmentHandle: HANDLE, 
	TmVirtualClock: PVOID) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 318",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueueApcThreadEx2(
	ThreadHandle: HANDLE, 
	Reserved: HANDLE, 
	ApcRoutine: PKNORMAL_ROUTINE, 
	ApcArgument1: PVOID, 
	ApcArgument2: PVOID, 
	ApcArgument3: PVOID, 
	Flags: ULONG, 
	Reserved2: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 371",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetInformationTransaction(
	TransactionHandle: HANDLE, 
	TransactionInformationClass: TRANSACTION_INFORMATION_CLASS, 
	TransactionInformation: PVOID, 
	TransactionInformationLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 430",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwWaitHighEventPair(
	EventPairHandle: HANDLE, 
	Alertable: BOOLEAN, 
	Timeout: PLARGE_INTEGER) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 487",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAlpcConnectPortEx(
	PortHandle: PHANDLE, 
	PortName: PUNICODE_STRING, 
	PortAttributes: PALPC_PORT_ATTRIBUTES, 
	ConnectionPort: PALPC_HANDLE, 
	Flags: ULONG, 
	RequiredServerSid: PSID, 
	MessageAttributes: PALPC_MESSAGE_ATTRIBUTES, 
	ConnectData: PVOID, 
	ConnectDataLength: PULONG, 
	ReceiveMessage: PVOID, 
	ReceiveMessageLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 124",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwGetCachedSigningLevel(
	FileHandle: HANDLE, 
	Flags: PULONG, 
	SigningLevel: PULONG, 
	Thumbprint: PUCHAR, 
	ThumbprintSize: PULONG, 
	SectionFlags: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 249",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtAcceptConnectPort(
	PortHandle: PHANDLE, 
	PortContext: PVOID, 
	ConnectionPortHandle: HANDLE, 
	SecurityQos: PSECURITY_QUALITY_OF_SERVICE, 
	ServerView: PVOID, 
	ServerViewSize: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 2",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwOpenTransactionManager(
	TmHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES, 
	LogFileName: PUNICODE_STRING, 
	TmIdentity: LUID, 
	OpenOptions: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 316",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetDefaultHardErrorPort(
	PortHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 412",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtQueryWnfStateNameInformation(
	StateName: PWNF_STATE_NAME, 
	InformationClass: WNF_STATE_NAME_INFORMATION_CLASS, 
	Information: PVOID, 
	InformationLength: ULONG, 
	ReturnLength: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 369",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwQueryInstallUILanguage(
	InstallUILanguageId: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 348",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetThreadExecutionState(
	ExecutionState: ULONG, 
	PreviousExecutionState: PULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 447",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwSetLdtEntries(
	Selector1: ULONG, 
	LdtEntry1: LDT_ENTRY, 
	Selector2: ULONG, 
	LdtEntry2: LDT_ENTRY) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 437",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtClearEvent(
	EventHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 62",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwInitializeEnclave(
	ProcessHandle: HANDLE, 
	BaseAddress: PVOID, 
	ZeroBits: SIZE_T, 
	EnclaveInformation: PVOID, 
	InformationLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 263",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwDeletePrivateNamespace(
	NamespaceHandle: HANDLE) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 222",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetDefaultLocale(
	DefaultLocaleId: ULONG, 
	UserProfile: BOOLEAN) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 413",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn ZwDeleteAtom(
	Atom: USHORT) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 216",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtSetInformationJobObject(
	JobHandle: HANDLE, 
	JobObjectInformationClass: JOBOBJECTINFOCLASS, 
	JobObjectInformation: PVOID, 
	JobObjectInformationLength: ULONG) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 425",
            "syscall",
            "ret"
        );
}

#[unsafe(naked)]
pub fn NtCreateDirectoryObject(
	DirectoryHandle: PHANDLE, 
	DesiredAccess: ACCESS_MASK, 
	ObjectAttributes: POBJECT_ATTRIBUTES) -> NTSTATUS {
    core::arch::naked_asm!(
            "mov r10, rcx",
            "mov eax, 172",
            "syscall",
            "ret"
        );
}