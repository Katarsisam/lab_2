use std::io::stdin; 




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
if a.contains(&format!("R{}", i)) && a.contains(&format!("W{}", i)) && a.contains(&format!("R{}", i)) { c += 1; } } 
return c; }



fn task_3(a: &mut i32) { 
let mut len = 0; 
let mut input = String::new(); 
stdin().read_line(&mut input).unwrap(); 
*a = input.trim().parse().unwrap(); 
let mut x = *a; 
let mut step = 1; 
let mut numb = 0; 
while x != 0 { 
x /= 10; len += 1; 
} 
x = *a; 
print!("Number:");
for _i in 0..len { 
    if step == 1 && *a / 10 != 0 { 
        numb += 1; 
        print!("\n{}: ", numb); 
    } 
    if *a % 10 == (*a / 10) % 10 || *a % 10 == x % 10 { 
        print!("{}", *a % 10); 
        step = 0; 
    } else { 
        step += 1; 
    } 
    x = *a; 
    *a /= 10; 
} 
print!("\n");
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
let mut k: i32 = 0 ;
let mut input = String::new(); 
stdin().read_line(&mut input).unwrap(); 
let num = input.trim().parse().unwrap(); 
match num { 
    1 => println!("{}", task_1(&mut k)), 
    2 => println!("{}", task_2()), 
    3 => task_3(&mut k), 
    _ => println!("Try again"), } 
}