use std::{io, cmp::Ordering};


const SECRET_NUMBER: i32 = 32;

fn main() {
    println!("Guess a number between 1 and 100! You have 3 attempts.");

    let mut attempts = 3;
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        if guess < 1 || guess > 100 {
            println!("The secret number has to be between 1 and 100");
            continue
        }

        match guess.cmp(&SECRET_NUMBER) {
            Ordering::Less => {
                println!("too small");
                attempts -= 1;
            },
            Ordering::Greater => {
                println!("too large");
                attempts -= 1;
            },
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }

        if attempts <= 0 {
            println!("You lose");
            break;
        }
    }

    // let mut hm = HashMap::new();
    // hm.insert("one", Todo { name: String::from("Hello world")});
    // hm.insert("two", Todo { name: String::from("My Name")});

    // println!("{:#?}", serde_json::to_string(&hm));

    // let t1: &Todo = hm.get("two").unwrap();
    

    // println!("{}", t1.name);
    // println!("Hello, world!");

    // let numbers = (2, 4, 8, 16, 32);

    // let origin = Point { x: 0, y: 12, z: 0 };

    // match origin {
    //     Point {x: 0, y, z} => println!("{}", y),
    //     _ => ()
    // }

    // let ds = Some(12);

    // match numbers {
    //     (a,b,c,d,e) => {
    //         println!("{}", e)
    //     },
    //     (first, .., last) => {
    //         println!("Some numbers: {}, {}", first, last);
    //     }
    // }
}

// struct Point {
//     x: i32,
//     y: i32,
//     z: i32,
// }
