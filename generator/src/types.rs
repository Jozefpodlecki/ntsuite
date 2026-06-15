pub struct Function {
    pub name: String,
    pub rva: u32,
    pub size: u32,
    pub category: FunctionCategory,
    pub return_type: Option<String>,
    pub parameter_types: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionCategory {
    Syscall(u32),
    NtZw,
    Rtl,
    Rtlp,
    Ldr,
    Tpp,
    Etw,
    Avrfp,
    Sym,
    Crt,
    Feature,
    Resource,
    Other,
}