use iced_x86::{Decoder, DecoderOptions, Code};
use anyhow::{Result, bail};

use crate::{pe_parser::extract_text_section, types::{Function, FunctionCategory}};

pub fn categorize_functions(pe_data: &[u8], functions: Vec<(u32, String)>) -> Result<Vec<Function>> {
    let text_bytes = extract_text_section(pe_data)?;
    
    functions.into_iter()
        .map(|(rva, name)| {
            let size = get_function_size(text_bytes, rva).unwrap_or(0);
            let category = categorize_function(text_bytes, rva, &name).unwrap_or(FunctionCategory::Other);
            
            Ok(Function {
                name,
                rva,
                size,
                category,
            })
        })
        .collect()
}

pub fn categorize_function(bytes: &[u8], rva: u32, name: &str) -> Result<FunctionCategory> {
    if !name.contains("Nt") && !name.contains("Zw") {
        return Ok(FunctionCategory::Other);
    }

    let (is_syscall, syscall_num) = analyze_syscall(bytes, rva)?;
    
    if is_syscall {
        Ok(FunctionCategory::Syscall(syscall_num))
    } else {
        Ok(FunctionCategory::Other)
    }
}

pub fn get_function_size(bytes: &[u8], rva: u32) -> Result<u32> {
    let mut decoder = Decoder::new(64, &bytes[rva as usize..], DecoderOptions::NONE);
    let mut size = 0u32;

    while !decoder.can_decode() {
        let instr = decoder.decode();
        
        if instr.is_invalid() {
            bail!("Invalid instruction at RVA 0x{:X}", rva);
        }

        if is_return_instruction(instr.code()) {
            break;
        }

        size += instr.len() as u32;
    }

    Ok(size)
}

fn analyze_syscall(bytes: &[u8], rva: u32) -> Result<(bool, u32)> {
    let mut decoder = Decoder::new(64, &bytes[rva as usize..], DecoderOptions::NONE);
    let mut syscall_num = None;
    let mut is_syscall = false;

    while !decoder.can_decode() {
        let instr = decoder.decode();
        
        if instr.is_invalid() {
            bail!("Invalid instruction");
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
    if instr.code() == Code::Mov_r32_imm32 && instr.op0_register() == iced_x86::Register::EAX {
        Some(instr.immediate32())
    } else {
        None
    }
}

fn is_return_instruction(code: Code) -> bool {
    matches!(code, Code::Retnw | Code::Retnq | Code::Retnd)
}