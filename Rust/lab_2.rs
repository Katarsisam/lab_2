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
    let mut step = 0;
    let mut x = *a;
    let mut mx = *a;
    for _ in 0..len {
        if *a % 10 == (*a / 10) % 10 || *a % 10 == x % 10 {
            step = 0;
        } else {
            step += 1;
        }
        x = *a;
        *a /= 10;
    }
    println!("{}", step);
    return if step == 0 { mx } else { 0 };
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

fn length(a: &mut i32) -> i32 {
    
    let mut len = 0;
    let mut x = *a;
    while x > 0 {
        x /= 10;
        len += 1;
    }
    return len;
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut numb: i32 = input.trim().parse().expect("Invalid input");
    match numb {
        1 => println!("{}", task_1(&mut numb)),
        2 => println!("{}", task_2()),
        3 => {
            let mut numb_x = String::new();
            io::stdin().read_line(&mut numb_x).expect("Failed to read input");
            let numb_x: i32 = numb_x.trim().parse().expect("Invalid input");
            println!("Numbers:");
            let mut count = 0;
            let mut len = 0;
            while count < numb_x {
            let mut inputX = String::new();
    io::stdin().read_line(&mut inputX).expect("Failed to read input");
    let mut k : i32 = input.trim().parse().expect("Invalid input");
                len = length(&mut k);
                println!("{}", task_3(&mut k, len));
                count += 1;
            }
        }
        _ => println!("Try again"),
    }
}