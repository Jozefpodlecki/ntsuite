mod v10_0_26100_8521;

pub struct FunctionSignature {
    pub name: &'static str,
    pub return_type: &'static str,
    pub parameters: Vec<(&'static str, &'static str)>,
}

pub use v10_0_26100_8521::*;