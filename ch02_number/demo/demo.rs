fn main(){
    // int
    println!("The sum is {}.", 80 + 34);
    println!("{} + {} = {}", 80, 34, 80+34);
    // + - * / %
    println!("{}", (23-6)%5 + 20*30 / (3+4));

    // float
    println!("{} + {} = {}", 80.3, 34.8, 80.3+34.8);
    println!("{} + {} = {}", 80.3, 34.9, 80.3+34.9);
    // + - * / %
    println!("{}", (23.-6.)%5. + 20.*30. / (3.+4.));

    // convert data type
    println!("{}+{}={}", 2.7, 1., 2.7+1.);

    // %
    println!("{} % {} = {}", -12, 10, -12%10);
    println!("{} % {} = {}", -1.2, 1., -1.2%1.);

    // strings
    println!("These
        are
        three lines!");
        
    println!("{}", "These\n\
    are\n\
    three lines!");
    println!("{}", "This \
    is \
    just one line");
}