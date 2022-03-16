fn main(){
    println!("##### 1. generic func  #####");
    let a:i16 = f_func::<i16>('a', 37, 41);
    let b:f64 = f_func::<f64>('b', 37.2, 41.1);
    println!("a={}, b={}", a, b);
    let c:i16 = f_func('a', 37, 41);
    let d:f64 = f_func('b', 37.2, 41.1);
    println!("c={}, d={}", c, d);
    
    println!("##### 2. generic struct/tuple  #####");
    f_struct_tuple();

    println!("##### 3. generic enum_None  #####");
    f_vec();

    println!("##### 4. generic enum_Error  #####");
    println!("Not recommend: {:?}, {:?}", f_divide(8., 2.), f_divide(8., 0.));
    f_show_divide(8., 2.);
    f_show_divide(8., 0.);

    println!("##### 5. generic enum_Error_stdlib  #####");
    f_divide_stdlib();
    f_vec_stdlib();
}

/*
enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/

fn f_vec_stdlib(){
    let mut v = vec![11, 22, 33];
    for _ in 0..v.len(){
        print!("{}, ", v.pop().unwrap());
    }
    println!("");
    for _ in 0..5{
        print!("{}, ", v.pop().unwrap());
        // generate below error info when loop index > 2
        // thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', demo.rs:40:32
    }
    println!("");
}

fn f_divide_stdlib(){
    let r1 = f_divide(8., 2.);
    let r2 = f_divide(8., 0.);
    println!("r1.is_ok()={}, r1.is_err()={}", r1.is_ok(), r1.is_err());
    println!("r2.is_ok()={}, r2.is_err()={}", r2.is_ok(), r2.is_err());
    println!("r1.unwrap()={}", r1.unwrap());
    //println!("r2.unwrap()={}", r2.unwrap()); //thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: "Divided by zero"', demo.rs:51:35
}

fn f_show_divide(num:f64, den:f64) {
    match f_divide(num, den) {
        Ok(val) => println!("{} / {} = {}", num, den, val),
        Err(msg) => println!("Cannot divide {} by {}: {}", num, den, msg),
    }
}

fn f_divide(numerator:f64, denominator:f64) -> Result<f64, String> {
    if denominator == 0. {
        Err(format!("Divided by zero"))
    } else {
        Ok(numerator / denominator)
    }
}

/*
enum MyOption<T> {
    Some(T),
    None,
}
*/

fn f_vec(){
    let mut v = vec![11, 22, 33];
    for _ in 0..5{
        let item: Option<i32> = v.pop();
        match item {
            Some(number) => print!("{}, ", number),
            None => print!("#, "),
        }
    }
    println!("");
}

struct S<T1, T2> {
    c:char,
    n1:T1,
    n2:T1,
    n3:T2,
}
struct SE<T1, T2> (char, T1, T1, T2);
fn f_struct_tuple(){
    let _s1 = S{c:'a', n1:34, n2:782, n3:0.02};
    let _s2 = S::<u16, f32>{c:'a', n1:34, n2:782, n3:0.02};
    println!("_s1={{ {}, {}, {}, {} }}", _s1.c, _s1.n1, _s1.n2, _s1.n3);
    println!("_s2={{ {}, {}, {}, {} }}", _s2.c, _s2.n1, _s2.n2, _s2.n3);

    let _se1 = SE ('a', 34, 782, 0.02);
    let _se2 = SE::<u16, f32> ('a', 34, 782, 0.02);
    println!("_se1=({}, {}, {}, {})", _se1.0, _se1.1, _se1.2, _se1.3);
    println!("_se2=({}, {}, {}, {})", _se2.0, _se2.1, _se2.2, _se2.3);
}

fn f_func<T>(ch:char, num1:T, num2:T) -> T {
    if ch == 'a' {num1}
    else {num2}
}