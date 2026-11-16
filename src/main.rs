///
/// Fract(ulator) - an RPN fraction calculator for makers.
///
/// by Eron Hennessey

use std::io::{self, Write};
use std::vec::Vec;
use std::str::FromStr;
use fraction::{Fraction};

/// Prompt the user for input.
fn prompt_user(prompt: String) -> String {
    // Prompt user with a String, return the result as a String.
    print!("{prompt}> ");
    // Must flush output because there was no newline.
    io::stdout().flush().unwrap();
    // Get the answer and return it...
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("Failed to read line");
    answer = String::from(answer.trim_end());
    return answer;
} 

const OPERATIONS: [&str; 10] = [
    "+", "-", "*", "/", "q", "s", "h", "?", "o", ""
];

/// Format a Fraction as a string with a whole number + fractional part.
fn format_fraction(f: &Fraction) -> String {
    let mut v:String;
    if f.trunc() == Fraction::from(0) {
        v = format!("{}", f.fract());
    } else if f.fract() == Fraction::from(0) {
        v = format!("{}", f.trunc());
    } else {
        v = format!("{} {}", f.trunc(), f.fract());
    }
    // if the denominator is > 16, then show an approximate value, as well.
    let approx_denom = 16u64;
    if f.denom().unwrap() > &approx_denom {
        let actual_denom = f.denom().unwrap().clone() as f64;
        let factor = actual_denom / (approx_denom as f64);
        let approx_numer = (f.numer().unwrap().clone() as f64) / factor;
        v = format!("{} (approx: {}/{})", v, approx_numer as u64, approx_denom);
    }
    v
}

fn show_operations() {
    println!("\nOperations are:");
    println!(" +      - Add the last two numbers on the stack.");
    println!(" -      - Subtract the last number on the stack from the second-to-last one.");
    println!(" *      - Multiply the last two numbers on the stack.");
    println!(" /      - Divide the second-to-last number on the stack by the last one.");
    println!(" s      - Show the current stack.");
    println!(" o      - Show all available operations.");
    println!(" h, ?   - Show this help.");
    println!(" q      - Quit the program.\n");
    println!("Pressing Enter at an empty prompt will duplicate the last value on the stack.\n");
}

fn show_help() {
    println!("\nfract(ulator) help");
    println!("==================\n");
    println!("This calculator uses reverse Polish notation (also known as RPN, or 'postfix' notation).\n");
    println!("Type either a *number* or *operation* at the prompt:");
    println!("* Numbers are saved on the *stack*.");
    println!("* Operations are performed on the values in the stack.\n");
    println!("Enter numbers first, then the operation to perform on them.");
    println!("Example: Entering '2', then '5', then '/' yields '2/5'.\n");
    println!("Numbers can be entered as decimals or fractions.");
    println!("Examples: '1.05', '3/4', '1+2/5', '1 1/4'.");
    println!("All numbers, whether decimal or fractional, are converted to reduced, regular fractions.");
    show_operations();
    println!("The result of any operation that yields a value will be printed as a reduced, regular fraction.");
    println!("If the result is a fraction with a denominator > 16, an approximate result is given in 16ths.");
    println!("Any values used are removed from the stack, and then the result is pushed back onto the stack.\n");
}

fn main() {
    let mut stack = Vec::new();
    let mut quit = false;

    while !quit {
        let input: String = prompt_user(String::from("")).to_lowercase();
        let i = input.as_str();
        if OPERATIONS.contains(&i) {
            match i {
                "+" => {
                    if stack.len() > 1 {
                        let x = stack.pop().unwrap();
                        let y = stack.pop().unwrap();
                        let result = y+x;
                        println!("\n{}\n", format_fraction(&result));
                        stack.push(result);
                    }
                    else {
                        println!("Not enough items for '+'!");
                    }
                },
                "-" => {
                    if stack.len() > 1 {
                        let x = stack.pop().unwrap();
                        let y = stack.pop().unwrap();
                        let result = y-x;
                        println!("\n{}\n", format_fraction(&result));
                        stack.push(result);
                    }
                    else {
                        println!("Not enough items for '-'!");
                    }
                },
                "*" => {
                    if stack.len() > 1 {
                        let x = stack.pop().unwrap();
                        let y = stack.pop().unwrap();
                        let result = y*x;
                        println!("\n{}\n", format_fraction(&result));
                        stack.push(result);
                    }
                    else {
                        println!("Not enough items for '*'!");
                    }
                },
                "/" => {
                    if stack.len() > 1 {
                        let x = stack.pop().unwrap();
                        let y = stack.pop().unwrap();
                        let result = y/x;
                        println!("\n{}\n", format_fraction(&result));
                        stack.push(result);
                    }
                    else {
                        println!("Not enough items for '/'!");
                    }
                },
                "s" => {
                    println!("\nCurrent stack");
                    println!("-------------");
                    let mut n = stack.len();
                    if n > 0 {
                        for v in stack.iter() {
                            n -= 1;
                            println!("{:2}: {}", n, format_fraction(v));
                        }
                        println!("");
                    } else {
                        println!("[empty]\n");
                    }
                },
                "o" => {
                    show_operations();

                },
                "h" | "?" => {
                    show_help();

                },
                "q" => {
                    quit = true;
                    println!("Quitting...");
                    continue;
                },
                "" => {
                    if stack.len() > 0 {
                        let last_val = stack.pop().unwrap();
                        let new_val = last_val.clone();
                        stack.push(last_val);
                        stack.push(new_val);
                    }
                    else {
                        println!("The stack is empty! No value duplicated. Enter 'h' for help.");
                    }
                }
                &_ => {
                    println!("What's this?!");
                }
            }
        } else {
            if i.contains(" ") || i.contains("+") {
                let num_parts: Vec<&str>;
                if i.contains(" ") {
                    num_parts = i.splitn(2, " ").collect();
                } else {
                    num_parts = i.splitn(2, "+").collect();
                }
                let whole_result = Fraction::from_str(num_parts[0]);
                let whole_part: Fraction;
                match whole_result {
                    Ok(v) => {
                        whole_part = v;
                    },
                    Err(e) => {
                        println!("Error: {} '{}'", e, num_parts[0]);
                        continue;
                    }
                }
                let frac_result = Fraction::from_str(num_parts[1]);
                let frac_part: Fraction;
                match frac_result {
                    Ok(v) => {
                        frac_part = v;
                    },
                    Err(e) => {
                        println!("Error: {} '{}'", e, num_parts[0]);
                        continue;
                    }
                }
                stack.push(whole_part + frac_part);
            } else {
                let frac_result = Fraction::from_str(i);
                match frac_result {
                    Ok(v) => {
                        stack.push(v);
                    },
                    Err(_e) => {
                        println!("Error: '{}' is not a known operation or number!", i);
                        println!("Enter 'o' for a list of operations. Enter 'h' for help.");
                    }
                }
            }
        }
    }
}

