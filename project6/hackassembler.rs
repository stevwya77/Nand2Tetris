/*
*   hackassembler.rs:
*   drives the process; opens,reads,create,and writes text files
*/

mod parser;
mod code;
mod symboltable;

use std::fs::File;
use std::io::{self, BufRead, BufReader, Seek, SeekFrom, Write};
use symboltable::TableManager;
use parser::{Parser, A_INSTRUCTION, C_INSTRUCTION, L_INSTRUCTION};

fn main() -> io::Result<()> {
    let mut line_parser = Parser::new();
    let mut manager = TableManager::new();
    let program = File::open("Pong.asm")?;
    let mut reader = BufReader::new(program);
    let mut pass_line_num = 0;

    // first pass, find labels and their line nums
    loop {
        if line_parser.has_more_lines(&mut reader)? { break; }
        let mut line = String::new();
        reader.read_line(&mut line)?;
        line_parser.advance(&line);
        let instruction = line_parser.instruction_type();

        match instruction {
            L_INSTRUCTION => {
                let symbol = line_parser.symbol().unwrap_or_default();
                manager.add_entry(symbol, Some(pass_line_num));
            }
            A_INSTRUCTION | C_INSTRUCTION => {
                pass_line_num += 1;
            }
           _ => {}
        }
    }

    // first pass complete, reset line
    reader.seek(SeekFrom::Start(0))?;

    let mut bin_output: Vec<String> = Vec::new();

    loop {
        if line_parser.has_more_lines(&mut reader)? { break; }
        let mut line = String::new();
        reader.read_line(&mut line)?;
        line_parser.advance(&line);
        let instruction = line_parser.instruction_type();

        match instruction {
            A_INSTRUCTION => {
                let symbol = line_parser.symbol().unwrap_or_default();
                let address: i32;

                if symbol.chars().next().unwrap_or(' ').is_ascii_digit() {
                    address = symbol.parse::<i32>().unwrap_or(0);
                } else {
                    if !manager.contains(&symbol) {
                        manager.add_entry(symbol.clone(), None);
                    }
                    address = *manager.get_address(&symbol).unwrap_or(&0);
                }
                bin_output.push(format!("0{:015b}", address));
            }

            C_INSTRUCTION => {
                let dest = line_parser.dest().unwrap_or_default();
                let d_code = code::bin_dest(&dest);
                println!("Destination:{} Bin:{}", dest, d_code);
                let comp = line_parser.comp().unwrap_or_default();
                let c_code = code::bin_comp(&comp);
                println!("Comp:{} Bin:{}", comp, c_code);
                let jump = line_parser.jump().unwrap_or_default();
                let j_code = code::bin_jump(&jump);
                println!("Jump:{} Bin:{}", jump, j_code);
                bin_output.push(format!("111{}{}{}", c_code, d_code, j_code));
            }
            _ => {}
        
        }
    }

    let join_and_sep = bin_output.join("\n");
    let mut output_file = File::create("Pong.hack")?;
    write!(output_file, "{}", join_and_sep)?;

    println!("assembly write to hack file completed");
    Ok(())
}