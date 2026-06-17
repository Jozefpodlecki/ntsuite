use codegen::Scope;
use log::info;
use std::fs;
use anyhow::Result;
use std::collections::HashMap;

use crate::{data::V10_0_26100_8521_SIGNATURES, types::{Function, FunctionCategory}};

pub fn generate_code(version: String, functions: Vec<Function>) -> Result<()> {
    let mut scope = Scope::new();
    
    let version_sanitized = version.replace('.', "_");
    let file_name = format!("v{version_sanitized}");

    for func in functions
        .iter()
        .filter(|f| matches!(f.category, FunctionCategory::Syscall(_)))
    {
    

        let number = match func.category {
            FunctionCategory::Syscall(number) => number,
            _ => continue,
        };
        
        let signature = match V10_0_26100_8521_SIGNATURES.get(func.name.as_str()) {
            Some(value) => value,
            None => {
                info!("Skipping {}", func.name);
                continue;
            },
        };

        let mut f = scope.new_fn(&func.name);
            f.vis("pub");
            f.ret(signature.return_type);
            f.attr("unsafe(naked)");

            for (kind, name) in &signature.parameters {
                f.arg(format!("\n\t{name}"), kind);
            }

            let asm = format!(
    r#"core::arch::naked_asm!(
        "mov r10, rcx",
        "mov eax, {}",
        "syscall",
        "ret"
    );"#,
                number
            );
            f.line(&asm);
    }
    
    let mut output = scope.to_string();
output = r#"use ntapi::{ntdef::*, ntdbg::*, ntobapi::*, ntexapi::*, ntioapi::*, ntkeapi::*, ntmmapi::*, ntseapi::*, ntregapi::*, ntpsapi::*, ntpoapi::*, ntlpcapi::*, ntrtl::*, nttmapi::*};

"#.to_string() + &output;
    let output_path = format!(r#"C:\repos\ntsuite\ntsyscalls\src\{file_name}.rs"#);
    fs::write(output_path, output)?;
    
    Ok(())
}