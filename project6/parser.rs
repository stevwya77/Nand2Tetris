/*
*   parser.rs:
*   Reads and parses an instruction
*/
use std::io::{self, BufRead};

pub const A_INSTRUCTION: &str = "A_INSTRUCTION";
pub const C_INSTRUCTION: &str = "C_INSTRUCTION";
pub const L_INSTRUCTION: &str = "L_INSTRUCTION";
pub const INVALID_INSTRUCTION: &str = "INVALID_INSTRUCTION";

pub struct Parser {
    pub curr: String,
}

impl Parser {
    // Constructor/Initializer
    pub fn new() -> Self {
        Self { curr: String::new() }
    }
    // checks if there is more work to do
    pub fn has_more_lines<R: BufRead>(&self, reader: &mut R) -> io::Result<bool> {
        Ok(reader.fill_buf()?.is_empty())
    }

    // gets the next instruction and makes it the current instruction
    pub fn advance(&mut self, line: &str) {
        self.curr = line.trim().to_string();
    }

    // returns type of current instruction, as a constant (a,c,l)
    pub fn instruction_type(&self) -> &'static str {
        let trimmed = self.curr.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") {
            return INVALID_INSTRUCTION;
        }

        match self.curr.chars().next() {
            Some('@') => A_INSTRUCTION,
            Some('(') => L_INSTRUCTION,
            _ => C_INSTRUCTION,
        }
    }

    // returns instructions symbol (only if @ or '()')
    pub fn symbol(&self) -> Option<String> {
        let inst_type = self.instruction_type();
        let line = &self.curr;

        match inst_type {
            A_INSTRUCTION => {
                if line.len() > 1 {
                    Some(line[1..].to_string())
                } else {
                    Some("".to_string())
                }
            }

            L_INSTRUCTION => {
                let start = line.find('(')?;
                let end = line.rfind(')')?;

                if end > start {
                    Some(line[start + 1..end].to_string())
                } else {
                    None
                }
            }

            _ => None,
        }
    }

    // returns instructions dest
    pub fn dest(&self) -> Option<String> {
        if self.instruction_type() == C_INSTRUCTION {
            if let Some((left, _right)) = self.curr.split_once('=') {
                return Some(left.trim().to_string());
            }
        }
        None
    }

    // returns instructions comp
    pub fn comp(&self) -> Option<String> {
        if self.instruction_type() != C_INSTRUCTION { return None; }

        let tmp = if let Some((_before_dest, right_comp)) = self.curr.split_once('=') {
            right_comp
        } else {
            &self.curr
        };

        let comp = if let Some((left_comp, _right_jump)) = tmp.split_once(';') {
            left_comp
        } else {
            tmp
        };

        Some(comp.trim().to_string())
    }

    // returns instructions jump
    pub fn jump(&self) -> Option<String> {
        if self.instruction_type() != C_INSTRUCTION { return None; }

        if let Some((_dest_comp, right_jump)) = self.curr.split_once(';') {
            let jump = right_jump.trim();
            let jump = jump.split("//").next().unwrap_or("").trim();
            if !jump.is_empty() {
                return Some(jump.to_string());
            }
        }
        None
    }
}