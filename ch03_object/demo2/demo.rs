fn main(){
    println!("##### variables #####");

    // declare const var
    let number1 = 12;
    let number2;
    number2 = 53;
    println!("{} + {} = {}", 12, 53, number1 + number2); //12 and 53 are literals, number1 and number2 are varables

    // declare mutable var
    let mut number3 = 12;
    println!("number3 = {}", number3);
    number3 = 10;
    println!("number3 = {}", number3);

    // warning if found an mut var, but never re-assigned
    let mut number4 = 12;
    println!("number4 = {}", number4);
    // to avoid waring
    let mut _number = 12;
    println!("_number = {}", _number);

    // data type
    //_numer = 12.; // data type error
    println!("12-(7%5)={}",12-(7%5));
    println!("12.-(7.%5.)={}",12.-(7.%5.));
    //println!("12.-(7%5)={}",12.-(7%5)); // data type error

    let number4 = 3.14; // re-declare number4, which pre-declared
    println!("number4 = {}", number4);

    let x = 120;
    print!("x={} ", x);
    let x = "abcd";
    print!("x={} ", x);
    let mut x = true;
    print!("x={} ", x);
    x = !x;
    print!("x={} ", x);

    // assign values
    let mut a = 12;
    a = a + 1;
    a = a - 4;
    a = a * 7;
    a = a / 6;
    print!("a={} ", a);
    a = 12;
    a += 1;
    a -= 4;
    a *= 7;
    a /= 6;
    println!("a={} ", a);

    println!("##### boolean #####");

    // boolean expression
    let mut truth = true;
    let mut falsity = false;
    println!("{} {}", truth, falsity);
    truth = 5>2;
    falsity = -12.3>10.;
    println!("{} {} {}", truth, falsity, -50<6);

    // string in dict order
    println!("{} {} {} {}", "abc"<"abcd", "ab"<"ac", "a"<"ab", "A"<"a");

    /*
    !, &&, ||
    */
    println!("{} {}", !truth, !falsity);
    println!("{} {} {} {}", falsity && falsity, falsity && truth, truth && falsity, truth && truth);
    println!("{} {} {} {}", falsity || falsity, falsity || truth, truth || falsity, truth || truth);
    println!("{} {}", truth || truth && !truth, (truth || truth) && !truth);

}