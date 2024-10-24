/**
 * take 4 parameters - operator, two integers and a mod integer and returns the quotient and remainder
 */

fn return_mod(num: i32, mod_num: i32) -> u32 {
    if num >= mod_num {
        (num % mod_num) as u32
    } else {
        num as u32
    }
}

fn mod_calculator(num_1: i32, num_2: i32, op: char, mod_num: i32) -> u32 {
    match op {
        '+' => {
            let result = num_1 + num_2;
            return_mod(result, mod_num)
        }
        '-' => {
            let result = num_1 - num_2;
            if result < 0 {
                let result_2 = result % mod_num;
                (mod_num + result_2) as u32
            } else {
                return_mod(result, mod_num)
            }
        }
        '*' => {
            let result = num_1 * num_2;
            return_mod(result, mod_num)
        }
        _ => 0,
    }
}

fn main() {
    let first = mod_calculator(10, 15, '+', 12);
    let second = mod_calculator(10, 15, '-', 12);
    let third = mod_calculator(10, 15, '*', 12);

    println!("10 + 15 mod 12 is {first}");
    println!("10 - 15 mod 12 is {second}");
    println!("10 * 15 mod 12 is {third}");
}
