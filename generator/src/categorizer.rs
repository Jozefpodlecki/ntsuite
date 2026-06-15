use iced_x86::{Code, Decoder, DecoderOptions, Register};
use anyhow::{Result, bail};

use crate::{pe_parser::extract_text_section, types::{Function, FunctionCategory}};

pub fn categorize_functions(pe_data: &[u8], mut functions: Vec<Function>) -> Result<Vec<Function>> {
    let (text_bytes, text_rva) = extract_text_section(pe_data)?;
    
    for func in functions.iter_mut() {
        let rva = func.rva;
        if rva < text_rva || rva >= text_rva + text_bytes.len() as u32 {
            continue;
        }
        
        let offset = (rva - text_rva) as usize;
        func.size = get_function_size(text_bytes, offset).unwrap_or(0);
        func.category = categorize_function(text_bytes, offset, &func.name).unwrap_or(FunctionCategory::Other);
    }
    
    Ok(functions)
}

pub fn categorize_function(bytes: &[u8], offset: usize, name: &str) -> Result<FunctionCategory> {
    if name.starts_with("Nt") || name.starts_with("Zw") {
        let (is_syscall, syscall_num) = analyze_syscall(bytes, offset)?;
        
        if is_syscall {
            return Ok(FunctionCategory::Syscall(syscall_num));
        }

        return Ok(FunctionCategory::NtZw);
    }
    
    if name.starts_with("Rtlp") {
        return Ok(FunctionCategory::Rtlp);
    }

    if name.starts_with("Rtl") {
        return Ok(FunctionCategory::Rtl);
    }

    if name.starts_with("Ldrp") || name.starts_with("Ldr") {
        return Ok(FunctionCategory::Ldr);
    }

    if name.starts_with("Tpp") || name.starts_with("Tp") {
        return Ok(FunctionCategory::Tpp);
    }

    if name.starts_with("Etw") || name.starts_with("Etwp") {
        return Ok(FunctionCategory::Etw);
    }

    if name.starts_with("Avrf") || name.starts_with("AVrfp") {
        return Ok(FunctionCategory::Avrfp);
    }

    if name.starts_with("Sym") || name.starts_with("SymCrypt") {
        return Ok(FunctionCategory::Sym);
    }

    if name.starts_with("Feature") {
        return Ok(FunctionCategory::Feature);
    }

    if name.starts_with('_') && (name.contains("toa") || name.contains("tow") || name.contains("toi")) {
        return Ok(FunctionCategory::Crt);
    }

    if name.starts_with('_') && (name.contains("fil") || name.contains("fget") || name.contains("fput") || name.contains("input") || name.contains("output")) {
        return Ok(FunctionCategory::Crt);
    }

    if name.starts_with('_') && (name.contains("wcs") || name.contains("str") || name.contains("mem")) {
        return Ok(FunctionCategory::Crt);
    }

    if name.starts_with("wc") || name.starts_with("str") || name.starts_with("mem") {
        return Ok(FunctionCategory::Crt);
    }

    if name.starts_with("_RtlMuiReg") || name.contains("MuiReg") {
        return Ok(FunctionCategory::Resource);
    }
    
    Ok(FunctionCategory::Other)
}

pub fn get_function_size(bytes: &[u8], offset: usize) -> Result<u32> {
    let mut decoder = Decoder::new(64, &bytes[offset..], DecoderOptions::NONE);
    let mut size = 0u32;

    while decoder.can_decode() {
        let instr = decoder.decode();
        
        if instr.is_invalid() {
            break;
        }

        if is_return_instruction(instr.code()) {
            size += instr.len() as u32;
            break;
        }

        size += instr.len() as u32;
    }

    Ok(size)
}

fn analyze_syscall(bytes: &[u8], offset: usize) -> Result<(bool, u32)> {
    let mut decoder = Decoder::new(64, &bytes[offset..], DecoderOptions::NONE);
    let mut syscall_num = None;
    let mut is_syscall = false;
    let mut instruction_count = 0;
    const MAX_INSTRUCTIONS: usize = 10;

    while decoder.can_decode() && instruction_count < MAX_INSTRUCTIONS {
        let instr = decoder.decode();
        instruction_count += 1;
        
        if instr.is_invalid() {
            break;
        }

        if instr.is_jmp_near() || instr.is_jmp_far() {
            break;
        }

        if let Some(num) = extract_mov_eax_immediate(&instr) {
            syscall_num = Some(num);
        }

        if instr.code() == Code::Syscall {
            is_syscall = true;
            break;
        }

        if is_return_instruction(instr.code()) {
            break;
        }
    }

    Ok((is_syscall, syscall_num.unwrap_or(0xFFFFFFFF)))
}

fn extract_mov_eax_immediate(instr: &iced_x86::Instruction) -> Option<u32> {
    if instr.code() == Code::Mov_r32_imm32 && instr.op0_register() == Register::EAX {
        Some(instr.immediate32())
    } else {
        None
    }
}

fn is_return_instruction(code: Code) -> bool {
    matches!(code, Code::Retnw | Code::Retnq | Code::Retnd)
}