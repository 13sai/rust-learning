use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("!!!guess!!!");
    println!("please input");

    let num = rand::thread_rng().gen_range(1..100);

    println!("rand num is {}", num);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("fail to read line");

        let guess: u32 = guess.trim().parse().expect("input number");

        println!("your input {}", guess);

        match guess.cmp(&num) {
            Ordering::Less => println!("too small"),
            Ordering::Equal => println!("wonderful!!!"),
            Ordering::Greater => println!("too large"),
        }
    }
}
