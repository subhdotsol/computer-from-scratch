use bits::{add_u8, binary_to_decimal, decimal_to_binary, sub_u8, twos_complement};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "to-binary" => {
            if args.len() < 3 {
                return print_usage();
            }
            let x = parse_u8(&args[2]);
            println!("{}", decimal_to_binary(x));
        }
        "to-decimal" => {
            if args.len() < 3 {
                return print_usage();
            }
            println!("{}", binary_to_decimal(&args[2]));
        }
        "add" => {
            if args.len() < 4 {
                return print_usage();
            }
            let a = parse_u8(&args[2]);
            let b = parse_u8(&args[3]);
            let sum = add_u8(a, b);
            print_addition(a, b, sum);
        }
        "sub" => {
            if args.len() < 4 {
                return print_usage();
            }
            let a = parse_u8(&args[2]);
            let b = parse_u8(&args[3]);
            let diff = sub_u8(a, b);
            print_subtraction(a, b, diff);
        }
        "twos-complement" => {
            if args.len() < 3 {
                return print_usage();
            }
            let x = parse_u8(&args[2]);
            print_twos_complement(x);
        }
        _ => print_usage(),
    }
}

fn parse_u8(s: &str) -> u8 {
    s.parse().expect("expected a number between 0 and 255")
}

fn print_addition(a: u8, b: u8, sum: u8) {
    println!("  {}", decimal_to_binary(a));
    println!("+ {}", decimal_to_binary(b));
    println!("-----------");
    println!("  {}", decimal_to_binary(sum));
    println!();
    println!("{} + {} = {}", a, b, sum);
}

fn print_subtraction(a: u8, b: u8, diff: u8) {
    println!("  {}", decimal_to_binary(a));
    println!("- {}", decimal_to_binary(b));
    println!("-----------");
    println!("  {}", decimal_to_binary(diff));
    println!();
    println!("{} - {} = {}", a, b, diff as i8);
}

fn print_twos_complement(x: u8) {
    println!("{}", decimal_to_binary(x));
    println!("{}", decimal_to_binary(twos_complement(x)));
}

fn print_usage() {
    println!("bits-playground <command> [args]");
    println!();
    println!("commands:");
    println!("  to-binary <n>            decimal -> binary");
    println!("  to-decimal <bits>        binary -> decimal");
    println!("  add <a> <b>              add two numbers bit by bit");
    println!("  sub <a> <b>              subtract two numbers");
    println!("  twos-complement <n>      show a number and its two's complement");
}