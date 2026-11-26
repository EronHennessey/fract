///
/// Fract(ulator) - an RPN fraction calculator for makers.
///
/// by Eron Hennessey

use std::io::{self, Write};
use std::vec::Vec;
use std::str::FromStr;
use fraction::{Fraction, Decimal};

const SEEHELP: &str = "Enter 'h' for help.";

const OPERATIONS: [&str; 11] = [
    "+", "-", "*", "/", "q", "s", "h", "?", "o", "d", ""
];

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

/// Format a Fraction as a string with a whole number + fractional part.
fn format_fraction(f: &Fraction) -> String {
    let mut v:String;
    if f.trunc() == Fraction::from(0) {
        // Value has no whole number part.
        v = format!("{}", f.fract());
    } else if f.fract() == Fraction::from(0) {
        // Value has no fractional part.
        v = format!("{}", f.trunc());
        return v; // No need to go further here.
    } else {
        // Value has both a whole and fractional part.
        v = format!("{} {}", f.trunc(), f.fract());
    }
    // if the denominator is > 16, then show an approximate value, as well.
    let approx_denom = 16u64;
    let d = f.denom().unwrap();
    let target_ds = [2, 4, 8, 16];
    if !(target_ds.contains(d)) {
        let actual_denom = f.denom().unwrap().clone() as f64;
        let factor = actual_denom / (approx_denom as f64);
        let approx_frac = Fraction::from_str(format!("{}/{}", ((f.numer().unwrap().clone() as f64) / factor) as u64, approx_denom).as_str()).unwrap();
        if approx_frac == Fraction::from(0) {
            // approximated value is zero!
            v = format!("{} (approx: 0)", v);
        } else {
            let approx_str = format_fraction(&approx_frac);
            v = format!("{} (approx: {})", v, approx_str);
        }
    }
    v
}

fn show_banner() {
    println!("fract(ulator)");
    println!("=============\n");
    println!("Enter numbers to add to the stack, or operations to apply to numbers on the stack.\n");
    println!("Enter 'o' to list available operations; 's' to show the stack; 'h' for help.\n");
}

fn show_operations() {
    println!("\nOperations are:");
    println!(" +      - Add the last two numbers on the stack.");
    println!(" -      - Subtract the last number on the stack from the second-to-last one.");
    println!(" *      - Multiply the last two numbers on the stack.");
    println!(" /      - Divide the second-to-last number on the stack by the last one.");
    println!(" d      - Approximate the last number on the stack as a decimal value.");
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

    show_banner();

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
                "d" => {
                    // Show the last value on the stack as a decimal value.
                    if stack.is_empty() {
                        println!("No values on the stack! {SEEHELP}");
                    } else {
                        let fv = stack.last().unwrap().clone();
                        // Convert fraction to decimal with automatic precision.
                        let dv = Decimal::from_fraction(fv);
                        println!("{}", dv);
                    }
                },
                "s" => {
                    // Show the current stack.
                    println!("\nStack");
                    println!("-----");
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
                    // Display the full help text.
                    show_help();

                },
                "q" => {
                    // Quit the program.
                    quit = true;
                    println!("Quitting...");
                    continue;
                },
                "" => {
                    // Duplicate the last value on the stack.
                    if stack.is_empty() {
                        println!("The stack is empty! No value duplicated. {SEEHELP}");
                    } else {
                        let new_val = stack.last().unwrap().clone();
                        stack.push(new_val);
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
                        println!("Enter 'o' for a list of operations. {SEEHELP}");
                    }
                }
            }
        }
    }
}

