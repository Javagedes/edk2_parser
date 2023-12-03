use std::collections::HashMap;
use regex::Regex;

use rusqlite::*;

#[derive(Debug)]
enum Block {
    If(String, Vec<Block>),
    ElseIf(String, Vec<Block>),
    Else(Vec<Block>),
    Statement(String),
}
impl Block {
    fn eval(&self) -> bool {
        match self {
            Block::If(condition, _) => {
                self._eval(condition)
            },
            Block::ElseIf(condition, _) => {
                self._eval(condition)
            },
            _ => return true
        }
    }

    fn _eval(&self, condition: &String) -> bool {
        let not_pattern = Regex::new(r"(!)([^=])").unwrap();
        let condition = condition
            .replace(" || ", " OR ") // OR Operator
            .replace(" && ", " AND ") // AND Operator
            .replace(" EQ ", " == ") // Equal Operator
            .replace(" NE ", " !=") // Not Equal Operator
            .replace(" GE ", " >= ") // Greater than or equal to Operator
            .replace(" LE ", " <= ") // Less than or equal to Operator
            .replace(" GT ", " > ") // Greater than Operator
            .replace(" LT ", " < "); // Less than Operator
        
        let condition = not_pattern.replace_all(&condition, |captures: &regex::Captures| {
            let excl = captures.get(1).unwrap().as_str();
            let after_excl = captures.get(2).unwrap().as_str();
            // Conditionally replace based on the character after '!'
            if after_excl == "=" {
                "!=".to_string()
            } else {
                "NOT ".to_string() + after_excl  
            }
        });
        
        let conn = Connection::open_in_memory().unwrap();
        let mut stmt = conn.prepare(format!("SELECT {} AS result", condition).as_str()).unwrap();
        let mut my_iter = stmt.query_map([], |row| {
            Ok(row.get::<usize, bool>(0).unwrap())
        }).unwrap();

        let result = my_iter.next().unwrap().unwrap();
        return result;
    }
}


fn parse_blocks(lines: &mut std::iter::Peekable<impl Iterator<Item = String>>) -> Vec<Block> {
    let mut blocks = Vec::new();

    while let Some(line) = lines.next() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        else if line.starts_with("!if") {
            let condition = line.trim_start_matches("!if");
            let block = Block::If(condition.to_string(), parse_blocks(lines));
            blocks.push(block);
            continue
        }
        else if line.starts_with("!else if") {
            let condition = line.trim_start_matches("!else if");
            let block = Block::ElseIf(condition.to_string(), parse_blocks(lines));
            blocks.push(block);
            continue
        }
        else if line.starts_with("!else") {
            let block = Block::Else(parse_blocks(lines));
            blocks.push(block);
            continue
        }
        else if line.starts_with("!endif") {
            break;
        }
        else {
            let block = Block::Statement(line.to_string());
            blocks.push(block);
        }

        match lines.peek() {
            Some(line) => {
                if line.trim().starts_with("!else if") {
                    break;
                }
        
                if line.trim().starts_with("!else") {
                    break;
                }
            },
            None => {break;}
        } 
    }
    blocks
}
fn parse_blocks2(lines: &mut std::iter::Peekable<impl Iterator<Item = String>>) -> Vec<Block> {
    let mut blocks = Vec::new();

    while let Some(line) = lines.next() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with("!if") {
            let condition = line.trim_start_matches("!if").trim();
            let if_block = Block::If(condition.to_string(), parse_blocks(lines));
            blocks.push(if_block);
        } 
        else if line.starts_with("!else if") {
            let condition = line.trim_start_matches("!else if").trim();
            let elseif_block = Block::ElseIf(condition.to_string(), parse_blocks(lines));
            blocks.push(elseif_block);
            break;
        } else if line.starts_with("!else") {
            let else_block = Block::Else(parse_blocks(lines));
            blocks.push(else_block);
            break;
        } else if line.starts_with("!endif") {
            break;
        } else {
            let statement_block = Block::Statement(line.to_string());
            blocks.push(statement_block);
        }
    }

    blocks
}
#[cfg(test)]
mod test_condition {
    use super::*;

    #[test]
    fn test_parse_blocks() {
        let data = r#"
        !if $(X) = "TRUE"
          DEFINE Y = 3
        !else
          !if $(Z) = "TRUE"
            DEFINE Y = 4
          !else if $(Z) = "FALSE"
            DEFINE Y = 5
          !endif
        !endif
        DEFINE Z = 6
        "#;
        let lines = data.lines().map(|line| line.to_string());
        let v = parse_blocks(&mut lines.peekable());

        
    }
    #[test]
    fn test_eval() {
        let tester = Block::Else(Vec::new());
        let tests = vec![
            // OR Operator
            (r#"(5 > 6) OR (5 > 4)"#, true),
            (r#"(5 > 6) || (5 > 4)"#, true),
            // AND Operator
            (r#"(5 > 6) AND (5 > 4)"#, false),
            (r#"(5 > 6) && (5 > 4)"#, false),
            // Bitwise OR operator
            (r#"(5 | 3) == 7"#, true),
            // Bitwise AND operator
            (r#"(5 & 3) == 1"#, true),
            // TODO: Bitwise xor operator
            // EQ Operator
            (r#"5 == 5"#, true),
            (r#""A" == "A""#, true),
            (r#"5 EQ 5"#, true),
            (r#""A" EQ "A""#, true),
            // NE Operator
            (r#"5 != 5"#, false),
            (r#""A" != "A""#, false),
            (r#"5 NE 5"#, false),
            (r#""A" NE "A""#, false),
            // // GE Operator
            (r#"5 >= 5"#, true),
            (r#"5 GE 5"#, true),
            // // LE Operator
            (r#"5 <= 5"#, true),
            (r#"5 LE 5"#, true),
            // // GT Operator
            (r#"5 > 5"#, false),
            (r#"5 GT 5"#, false),
            // // LT Operator
            (r#"5 < 5"#, false),
            (r#"5 LT 5"#, false),
            // // NOT Operator
            (r#"NOT (5 == 5)"#, false),
            (r#"!(5 == 5)"#, false),




        ]; 
        for (test, result) in tests {
            assert_eq!(tester._eval(&test.to_string()), result);
        }
    }
}