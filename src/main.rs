use std::io;
use std::cmp::Ordering;
use rand::Rng;
use brainfuck;
use std::convert::{TryFrom,TryInto};


const FIB: u128 = 99999999;
const ELEGANCY: bool = true;

fn fib_art(i: usize) -> usize {
    //const BF: [char; 7] = ['', 'a'];

 
    let mut res: usize;

    if i <= 1 {
        //println!("fib({}) = {};", i, 1);
        res = 1;
    } else {
        res = fib_art(i-2) + fib_art(i-1);
    }

    //print!("{}", v[i%v.len()]);
        //std::iter::repeat("=>").take(i).collect::<String>())
    //print!("fib({}) = {};", i, res);
    return res;
}

/// Iterative fibonacci.
///
/// https://github.com/rust-lang/rust-by-example
pub struct Fibonacci {
	curr: u128,
	next: u128,
}

impl Iterator for Fibonacci {
	type Item = u128;
	fn next(&mut self) -> Option<u128> {
		let new_next = self.curr + self.next;

		self.curr = self.next;
		self.next = new_next;

		Some(self.curr)
	}
}


pub fn fib() -> Fibonacci {
	Fibonacci { curr: 1, next: 1 }
}

fn main() {

    //let v : Vec<char> = vec!['+', '+', '>', '<', '-', '-', '[', ']', '.'];
    let v : Vec<char> = vec!['+', '+', '>', '<', '-', '-', '<', '>', '.', '.', '.', '.'];
    let mut code = String::from("");
    let mut block = String::from("");

    const BLOCK_NUM:u128 = 40;
    const BLOCK_LEN:usize = 320;

    let cap = 40;
    let mut seed:u128 = 2 + rand::thread_rng().gen_range(1..128);
    let mut last:u128= seed;
    let mut iter:u128 = 0;
    let mut num_of_blocks:u128 = 0;
    let mut prev_char: char = ' ';

    loop {
        iter += 1;

        let c: char= {
            last += fib().take((5+(seed+last)%42).try_into().unwrap()).last().unwrap();
            last += rand::thread_rng().gen_range(1..65);
            let picks: Vec<char> = match prev_char {
                '+' => vec!['+', '+', '+', '+', '+', '+', '+', '+', '>', '<', '[', ']', '.', '.'],
                '-' => vec!['-', '-', '+', '+', '+', '>', '<', '+', '[', ']', '.', '.', '.', '.'],
                '.' => vec!['+', '+', '>', '>','>','>','>','<','<','<','<','<', '<', '-', '-', '[', ']'],
                '>' => vec!['+', '+', '>', '>','>','>','>', '-', '-', '[', ']', '.'],
                '<' => vec!['+', '+', '<', '<','<','<','<', '-', '-', '[', ']', '.'],
                '[' => vec!['+', '+', '>', '>','>','>','>','<','<','<','<','<', '<', '-', '-', '['],
                ']' => vec!['+', '+', '>', '>','>','>','>','<','<','<','<','<', '<', '-', '-', ']'],
                _ => vec!['+', '>', '<', '-', '[', ']', '.']
            };
            //println!("{}", last);
            let i: usize = last.try_into().unwrap();
            //println!("{}", i%8);
            prev_char = picks[i%picks.len()];
            prev_char
        };

        if (block.len() >= BLOCK_LEN) {
            let result = match brainfuck::eval_string(&block) {
                Ok(n) => {
                    println!("{}", block);
                    code.push_str(&block);
                    num_of_blocks += 1;
                },
                _ => {
                    //println!("> {}", iter);
                    let new_seed = rand::thread_rng().gen_range(1..65);
                    seed = new_seed;
                    block = "".to_string();
                }
            };
            //break;
        } else {
            block.push(c);
        }

        if (num_of_blocks >= BLOCK_NUM) {
            println!("{}", code);
            break;
        }

        //print!("{}", c);
    }


    //loop {
        //println!("Guess the number!");

        //let secret_number = rand::thread_rng().gen_range(1..101);
        //println!("hint: {}", secret_number);

        //let mut  guess = String::new();

        //io::stdin()
        //.read_line(&mut guess)
        //.expect("Failed to read line");

        //let guess: u32 = match guess.trim().parse() {
            //Ok(num) => num,
            //Err(_) => continue,
        //};



        //match guess.cmp(&secret_number) {
            //Ordering::Less => println!("Too small"),    
            //Ordering::Greater => println!("Too big"),    
            //Ordering::Equal => {
                //let mut _g = String::new();
                //println!("You got it! Wanna experience the true power of Rust?");

                //io::stdin()
                    //.read_line(&mut _g)
                    //.expect("failed to read line");

                //match _g.trim() {
                    //"yes please" => println!("{}", fib(42)),
                    //_ => break,
                //}
              
            //},    
        //}
        
        //println!("You guess {}", guess);
    //}
}
