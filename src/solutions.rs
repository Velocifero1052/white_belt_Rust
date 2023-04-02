
pub fn square_root_resolver(){
    let (a, b, c): (f32, f32, f32);
    scan!("{} {} {}\n", a, b, c);

    if a != 0.0 && b != 0.0 {
        let mut under_root_expression = b * b - 4.0 * a * c;
        if under_root_expression < 0.0 {
            return;
        } else if under_root_expression == 0.0 {
            println!("{}", -b / (2.0 * a));
        } else {
            under_root_expression = f32::sqrt(under_root_expression);
            let x1 = (-b + under_root_expression) / (2.0 * a);
            let x2 = (-b - under_root_expression) / (2.0 * a);
            println!("{} {}", x1, x2)
        }
    }else if a == 0.0 && b != 0.0 && c != 0.0{
        println!("{}", -c / b)
    }else if a != 0.0 && b == 0.0 && c == 0.0 || a == 0.0 && b != 0.0 && c == 0.0 {
        println!("{}", 0.0)
    }else if a != 0.0 && b == 0.0 && c != 0.0 && -c / a >= 0.0{
        let res = f32::sqrt(-c / a);
        println!("{}", res)
    }
}

pub fn so_called_impossible(){
    let (a, b):(i32, i32);
    scan!("{} {}", a, b);
    if b != 0 {
        println!("{}", a / b)
    }else{
        println!("Impossible")
    }
}

pub fn price_calculations(){
    let (n, a, b, x, y, mut final_price): (f32, f32, f32, f32, f32, f32);
    scan!("{} {} {} {} {}", n, a, b, x, y);
    final_price = n;
    if n > a && n > b {
        final_price = n - n * (y / 100.0)
    }else if n > a && n <= b {
        final_price = n - n * (x / 100.0)
    }
    println!("{}", final_price)
}

pub fn print_even_numbers(){
    let (x, y, mut i): (i32, i32, i32);
    let mut first = true;
    scan!("{} {}", x, y);

    if x == y && x % 2 == 0 {
        println!("{}", x);
        return
    } else if x % 2 == 0 {
        i = x;
    } else {
        i = x + 1;
    }
    while i <= y {
        if first {
            print!("{}", i);
            first = false;
        } else {
            print!(" {}", i);
        }
        i += 2;
    }
}

pub fn second_occurrence_of_f(){
    let mut word = String::new();
    let mut found_index = -2;
    let mut first = true;
    scan!("{}", word);
    let mut i:i32 = 0;
    while i < word.len() as i32{
        let char = word.as_bytes()[i as usize] as char;
        if char == 'f' && first {
            found_index = -1;
            first = false;
        }else if char == 'f' && !first {
            found_index = i;
            break;
        }
        i += 1;
    }
    print!("{}", found_index)
}

pub fn gcd(a: i32, b: i32) -> i32{
    if b == 0{
        return a;
    }
    return gcd(b, a % b)
}

pub fn convert_to_binary_string(){
    let mut n:i32;
    scan!("{}", n);
    print!("{}", format!("{:b}", n));
}