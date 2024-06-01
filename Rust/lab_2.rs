use std::io::stdin; 
use std::io;

fn task_1(n: &mut i32) -> i32 { 
let mut ak = 0; 
let mut input = String::new(); 
stdin().read_line(&mut input).unwrap(); 
*n = input.trim().parse().unwrap();  
for i in 0..=*n { 
    if simpl_numb(i) 
    { ak = i; } 
}
//print!("{}: ", ak); 
let mut al=*n;
while  !simpl_numb(al) == true{
    al+=1; }
//print!("\n{}:\n ", al); 
return if *n - ak < al - *n { *n - ak } else { al-*n }; 
}

fn task_2() -> i32 { 
let mut a = String::new(); 
stdin().read_line(&mut a).unwrap(); 
let mut c = 0; 
for i in 0..=9 { 
if a.contains(&format!("R{}", i)) && a.contains(&format!("W{}", i)) && a.contains(&format!("B{}", i)) { c += 1; } } 
return c; }

fn task_3(a: &mut i32, len: i32) -> i32 {
    let mut step = 1;
    let mut x = 0;
    let  mx = *a;
    for _ in 0..len {
        if *a % 10 == (*a / 10) % 10 || *a % 10 == x % 10 {
            step = 0;
        } else {
            step += 1;
        }
        x = *a;
        *a /= 10;
    }
    return if step == 0 { mx } else { 0 };
}

fn length(a: i32) -> i32 {
    let mut x = a;
    let mut len = 0;
    while x > 0 {
        x /= 10;
        len += 1;
    }
    return len;
}

fn simpl_numb(a: i32) -> bool { 
    if a > 2 { 
        for i in 2..a { 
            if a % i == 0 { 
                return false; } 
            } return true; } 
            else { 
                return false; } 
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let numb: i32 = input.trim().parse().unwrap();
    let mut k = 0;
    let mut count = 0;
    let mut numb_x = 0;
    let mut len = 0;
    match numb {
        1 => println!("{}", task_1(&mut k)),
        2 => println!("{}", task_2()),
        3 => {
            input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            numb_x = input.trim().parse().unwrap();
           
            while count < numb_x { 
            input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let mut k : i32 = input.trim().parse().expect("Invalid input");
                len = length(k);
                println!("\n{}", task_3(&mut k, len));
                count += 1;
            }
            println!();
        }
        _ => println!("Try again"),
    }
}