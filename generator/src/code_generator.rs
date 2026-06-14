use codegen::Scope;
use std::fs;
use anyhow::Result;
use std::collections::HashMap;

use crate::types::{Function, FunctionCategory};

pub fn generate_code(functions: Vec<Function>) -> Result<()> {
    let mut scope = Scope::new();
    let mut categorized: HashMap<&'static str, Vec<Function>> = HashMap::new();
    
    for func in functions {
        let category_name = match func.category {
            FunctionCategory::Syscall(_) => "syscalls",
            FunctionCategory::Rtl => "rtl",
            FunctionCategory::Rtlp => "rtlp",
            FunctionCategory::Ldr => "ldr",
            FunctionCategory::Tpp => "tpp",
            FunctionCategory::Etw => "etw",
            FunctionCategory::Avrfp => "avrfp",
            FunctionCategory::String => "string",
            FunctionCategory::Feature => "feature",
            FunctionCategory::Resource => "resource",
            FunctionCategory::Other => "other",
        };
        categorized.entry(category_name).or_default().push(func);
    }
    
    for (module_name, module_functions) in categorized {
        let module = scope.new_module(module_name);
        
        for func in module_functions {
            match func.category {
                FunctionCategory::Syscall(num) => {
                    let fn_name = format!("nt_{}", func.name.trim_start_matches("Nt"));
                    let mut f = module.new_fn(&fn_name);
                    f.vis("pub");
                    f.line(&format!("// Syscall number: 0x{:X} ({})", num, num));
                    f.line("unimplemented!()");
                }
                _ => {
                    let mut f = module.new_fn(&func.name);
                    f.vis("pub");
                    f.line("unimplemented!()");
                }
            }
        }
    }
    
    let output = scope.to_string();
    fs::write("generated.rs", output)?;
    Ok(())
}