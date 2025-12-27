/*
*   symboltable.rs:
*   handles symbols
*/
use std::collections::HashMap;

pub struct TableManager {
    table: HashMap<String, i32>,
    next_address: i32,
}

impl TableManager {
    pub fn new() -> Self {
        let mut table = HashMap::new();
        // predefined addresses
        table.insert("R0".to_string(), 0);
        table.insert("R1".to_string(), 1);
        table.insert("R2".to_string(), 2);
        table.insert("R3".to_string(), 3);
        table.insert("R4".to_string(), 4);
        table.insert("R5".to_string(), 5);
        table.insert("R6".to_string(), 6);
        table.insert("R7".to_string(), 7);
        table.insert("R8".to_string(), 8);
        table.insert("R9".to_string(), 9);
        table.insert("R10".to_string(), 10);
        table.insert("R11".to_string(), 11);
        table.insert("R12".to_string(), 12);
        table.insert("R13".to_string(), 13);
        table.insert("R14".to_string(), 14);
        table.insert("R15".to_string(), 15);
        table.insert("SP".to_string(), 0);
        table.insert("LCL".to_string(), 1);
        table.insert("ARG".to_string(), 2);
        table.insert("THIS".to_string(), 3);
        table.insert("THAT".to_string(), 4);
        table.insert("SCREEN".to_string(), 16384);
        table.insert("KBD".to_string(), 24576);

        Self {
            table,
            next_address: 16,
        }
    }
    // checks if symbol exists in table
    pub fn contains(&self, symbol: &str) -> bool {
        self.table.contains_key(symbol)
    }

    // gets address if exists in table
    pub fn get_address(&self, symbol: &str) -> Option<&i32> {
        self.table.get(symbol)
    }

    // adds <symbol, address> to the table
    pub fn add_entry(&mut self, symbol: String, address: Option<i32>) {
        if !self.table.contains_key(&symbol) {
            match address {
                Some(addr) => {
                    self.table.insert(symbol, addr);
                }
                None => {
                    self.table.insert(symbol, self.next_address);
                    self.next_address += 1;
                }
            }
        }
    }
}

