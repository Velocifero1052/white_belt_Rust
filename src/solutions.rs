
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