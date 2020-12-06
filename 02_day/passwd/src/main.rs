use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::Peekable;

#[derive(Debug, Clone)]
pub enum Tokens {
    Passwd(String),
    Target(String),
    Num(usize),
}

fn get_number<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> usize {
    let mut number = c.to_string().parse::<usize>().expect("The caller should have passed a digit.");
    while let Some(Ok(digit)) = iter.peek().map(|c| c.to_string().parse::<usize>()) {
        number = number * 10 + digit;
        iter.next();
    }
    number
}

fn get_word<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> String {
    let mut word = c.to_string().parse::<String>().expect("The caller should have passed a character.");
    while let Some(Ok(character)) = iter.peek().map(|c| c.to_string().parse::<char>()) {
        match character {
            'a'..='z' => {
                word.push(character);
                iter.next();
            },
            _ => {
                break;
            }
        };
    }
    word
}

fn lex_entry(input: &String) -> Result<Vec<Tokens>, String> {
    let mut result = Vec::new();
    let mut it = input.chars().peekable();
   
    while let Some(&c) = it.peek() {
        match c {
            '0'..='9' => {
                it.next();
                let n = get_number(c, &mut it);
                result.push(Tokens::Num(n));
            }
            'a'..='z' => {
                it.next();
                let n = get_word(c, &mut it);
                if n.len() > 1 {
                    result.push(Tokens::Passwd(n));
                } else {
                    result.push(Tokens::Target(n));
                }
            }
            ' ' | '-' | ':' | '\n' => {
                it.next();
            }
            _ => {
                return Err(format!("unexpected character {}", c));
            }
        }
    }
   
    Ok(result)
}

fn parse_entry(tokens: &Vec<Tokens>) -> Result<(usize, usize, &String, &String), &str> {
    if tokens.len() != 4 { 
        return Err("Expression needs four tokens.");
    }

    let min = match tokens.get(0).unwrap() { 
        Tokens::Num(n) => n, 
        _ => {
            return Err("Expected number.")
        }
    };

    let max = match tokens.get(1).unwrap() { 
        Tokens::Num(n) => n, 
        _ => {
            return Err("Expected number.")
        }
    };
    
    let target = match tokens.get(2).unwrap() { 
        Tokens::Target(t) => t, 
        _ => {
            return Err("Expected token.")
        }
    };
    
    let passwd = match tokens.get(3).unwrap() { 
        Tokens::Passwd(p) => p, 
        _ => {
            return Err("Expected passwd.")
        }
    };
    
    Ok((*min, *max, target, passwd))
}

fn evaluate_expression_one(min: &usize, max: &usize, target: &String, passwd: &String) -> bool {
    let v: Vec<&str> = passwd.matches(target).collect();

    if *min <= v.len() && v.len() <= *max {
        return true;
    } 
    
    return false;
}

fn evaluate_expression_two(min: &usize, max: &usize, target: &String, passwd: &String) -> bool {
    let char_at_pos_one = passwd.get(*min-1..*min).unwrap_or("1");
    let char_at_pos_two = passwd.get(*max-1..*max).unwrap_or("1");

    if (char_at_pos_one == target) ^ (char_at_pos_two == target) {
        return true;
    } 

    return false;
}

fn is_valid(entry: &String) -> bool {        
    let tokens = lex_entry(&entry).unwrap();
    let (min, max, target, passwd) = parse_entry(&tokens).unwrap();
    let result = evaluate_expression_one(&min, &max, &target, &passwd);
    //let result = evaluate_expression_two(&min, &max, target, passwd);
    return result;
}

fn main() -> std::io::Result<()> {
    let file_name = std::env::args().nth(1).expect("no file given");
    let file = File::open(&file_name)?;
    let reader = BufReader::new(file);
    let mut count = 0; 

    for line in reader.lines() {
        if is_valid(&line.unwrap()) {
            count += 1;
        }
    }

    println!("Valid Passwords: {}", count);
    Ok(())
}