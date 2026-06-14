
pub struct Function {
    pub name: String,
    pub rva: u32,
    pub size: u32,
    pub category: FunctionCategory,
}

#[derive(Debug)]
pub enum FunctionCategory {
    Syscall(u32),
    Rtl,
    Rtlp,
    Ldr,
    Tpp,
    Etw,
    Avrfp,
    String,
    Feature,
    Resource,
    Other,
}