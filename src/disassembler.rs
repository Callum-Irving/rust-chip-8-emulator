pub fn disassemble_opcode(pc: usize, opcode: u16) -> String {
    let first_nibble = (opcode >> 12 & 0x000F) as u8;

    let assembly: String = match first_nibble {
        0x00 => match (opcode & 0x00FF) as u8 {
            0xe0 => {
                format!("{: <10}", "CLS")
            }
            0xEE => {
                format!("{: <10}", "RET")
            }
            _ => {
                let addr = opcode & 0x0FFF;
                format!("{: <10} #${:03x}", "SYS", addr)
            }
        },
        0x01 => {
            let addr = opcode & 0x0FFF;
            format!("{: <10} #${:03x}", "JP", addr)
        }
        0x02 => {
            let addr = opcode & 0x0FFF;
            format!("{: <10} #${:03x}", "CALL", addr)
        }
        0x03 => {
            let reg = opcode >> 8 & 0x000F;
            let val = opcode & 0x00FF;
            format!("{: <10} V{:01x}, #${:02x}", "SE", reg, val)
        }
        0x04 => {
            let reg = opcode >> 8 & 0x000F;
            let val = opcode & 0x00FF;
            format!("{: <10} V{:01x}, #${:02x}", "SNE", reg, val)
        }
        0x05 => {
            let regx = opcode >> 8 & 0x000F;
            let regy = opcode >> 4 & 0x000F;
            format!("{: <10} V{:01x}, V{:01x}", "SE", regx, regy)
        }
        0x06 => {
            let reg = opcode >> 8 & 0x000F;
            let val = opcode & 0x00FF;
            format!("{: <10} V{:01x}, #${:02x}", "LD", reg, val)
        }
        0x07 => {
            let reg = opcode >> 8 & 0x000F;
            let val = opcode & 0x00FF;
            format!("{: <10} V{:01x}, #${:02x}", "ADD", reg, val)
        }
        0x08 => match (opcode & 0x000F) as u8 {
            0x00 => {
                let regx = opcode >> 8 & 0x000F;
                let regy = opcode >> 4 & 0x000F;
                format!("{: <10} V{:01x}, V{:01x}", "LD", regx, regy)
            }
            0x01 => {
                let regx = opcode >> 8 & 0x000F;
                let regy = opcode >> 4 & 0x000F;
                format!("{: <10} V{:01x}, V{:01x}", "OR", regx, regy)
            }
            0x02 => {
                let regx = opcode >> 8 & 0x000F;
                let regy = opcode >> 4 & 0x000F;
                format!("{: <10} V{:01x}, V{:01x}", "AND", regx, regy)
            }
            0x03 => {
                let regx = opcode >> 8 & 0x000F;
                let regy = opcode >> 4 & 0x000F;
                format!("{: <10} V{:01x}, V{:01x}", "XOR", regx, regy)
            }
            0x04 => {
                let regx = opcode >> 8 & 0x000F;
                let regy = opcode >> 4 & 0x000F;
                format!("{: <10} V{:01x}, V{:01x}", "ADD", regx, regy)
            }
            0x05 => {
                let regx = opcode >> 8 & 0x000F;
                let regy = opcode >> 4 & 0x000F;
                format!("{: <10} V{:01x}, V{:01x}", "SUB", regx, regy)
            }
            0x06 => {
                let regx = opcode >> 8 & 0x000F;
                let regy = opcode >> 4 & 0x000F;
                format!("{: <10} V{:01x}, V{:01x}", "SHR", regx, regy)
            }
            0x07 => {
                let regx = opcode >> 8 & 0x000F;
                let regy = opcode >> 4 & 0x000F;
                format!("{: <10} V{:01x}, V{:01x}", "SUBN", regx, regy)
            }
            0x0E => {
                let regx = opcode >> 8 & 0x000F;
                let regy = opcode >> 4 & 0x000F;
                format!("{: <10} V{:01x} {{,V{:01x}}}", "SHL", regx, regy)
            }
            _ => "UNKNOWN".to_string(),
        },
        0x09 => {
            let regx = opcode >> 8 & 0x000F;
            let regy = opcode >> 4 & 0x000F;
            format!("{: <10} V{:01x}, V{:01x}", "SNE", regx, regy)
        }
        0x0a => {
            let val = opcode & 0x0FFF;
            format!("{: <10} I, #${:03x}", "LD", val)
        }
        0x0b => {
            let addr = opcode & 0x0FFF;
            format!("{: <10} V0, #${:03x}", "JP", addr)
        }
        0x0c => {
            let reg = opcode >> 8 & 0x000F;
            let val = opcode & 0x00FF;
            format!("{: <10} V{:01x}, #${:02x}", "RND", reg, val)
        }
        0x0d => {
            let regx = opcode >> 8 & 0x000F;
            let regy = opcode >> 4 & 0x000F;
            let nibble = opcode & 0x000F;
            format!(
                "{: <10} V{:01x}, V{:01x} ,#${:01x}",
                "DRW", regx, regy, nibble
            )
        }
        0x0e => {
            let reg = opcode >> 8 & 0x000F;
            format!("{: <10} V{:01x}", "SKP", reg)
        }
        0x0f => match (opcode & 0x00FF) as u8 {
            0x07 => {
                let reg = opcode >> 8 & 0x000F;
                format!("{: <10} V{:01x}, DT", "LD", reg)
            }
            0x0a => {
                let reg = opcode >> 8 & 0x000F;
                format!("{: <10} V{:01x}, K", "LD", reg)
            }
            0x15 => {
                let reg = opcode >> 8 & 0x000F;
                format!("{: <10} DT, V{:01x}", "LD", reg)
            }
            0x18 => {
                let reg = opcode >> 8 & 0x000F;
                format!("{: <10} ST, V{:01x}", "LD", reg)
            }
            0x1e => {
                let reg = opcode >> 8 & 0x000F;
                format!("{: <10} I, V{:01x}", "ADD", reg)
            }
            0x29 => {
                let reg = opcode >> 8 & 0x000F;
                format!("{: <10} F, V{:01x}", "LD", reg)
            }
            0x33 => {
                let reg = opcode >> 8 & 0x000F;
                format!("{: <10} B, V{:01x}", "LD", reg)
            }
            0x55 => {
                let reg = opcode >> 8 & 0x000F;
                format!("{: <10} [I], V{:01x}", "LD", reg)
            }
            0x65 => {
                let reg = opcode >> 8 & 0x000F;
                format!("{: <10} V{:01x}, [I]", "LD", reg)
            }
            _ => "UNKNOWN".to_string(),
        },
        _ => "UNKNOWN".to_string(),
    };

    format!(
        "{:04x} {:02x} {:02x} {}",
        pc,
        opcode >> 8,
        opcode & 0xFF,
        assembly
    )
}
