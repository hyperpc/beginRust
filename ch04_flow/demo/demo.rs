fn main(){
    // condition flow
    condition_demo();

    // loop
    loop_demo();

    // scope
    scope_demo();
}

fn condition_demo() {
    println!("##### condition_demo #####");

    // 1. if...else...
    let n=4;
    if n>0 {
        println!("number is positive");
    } else {
        println!("non positive");
    }
    
    // 2. if...else if...else...
    let m = 4;
    let mut text="";
    if m > 1000 {
        text = "big";
    } else if m > 0 {
        text = "small";
    } else if m < 0 {
        text = "negative";
    } else {
        text = "neither positive nor negative";
    }
    println!("{} ", text);

    // 3. ?: similiar, but inner expression have no tailor ";"
    println!("{}",
        if m>1000 {"big"}
        else if m>0 {"small"}
        else if m<0 {"negative"}
        else {"neither positive nor negative"}
    );

    // 4. return same data type in each branch
    let _a = if true {"abc"} else {"xyz"};
    let _b = if true {3456} else {12};
    let _c = if true {3.14} else {12.};
    println!("{} {} {}", _a, _b, _c);

}

fn loop_demo() {
    println!("##### loop_demo #####");

    // 1. while
    let mut i=1;
    while i<= 10 {
        print!("{} ", i*i);
        i += 1;
    }
    println!("");

    // 2. continue, break
    i=0;
    while i<50 {
        i += 1;
        if i%3==0 {continue;}
        if i*i>400 {break;}
        println!("{} ", i*i);
    }

    // 3. loop - infinity
    i=1;
    loop {
        let ii = i*i;
        if ii>=200 {break;}
        print!("{} ", ii);
        i += 1;
    }
    println!("");

    // 4. for-in..
    let mut limit=4;
    for i in 1..limit {
        limit -= 1;
        print!("{} ", i);
    }
    println!("");
    println!(":{} ", limit);
}

fn scope_demo() {
    println!("##### scope_demo #####");

    // 1. destroy inner variable
    let mut  _i=1;
    if true {
        let _i=2;
    }
    print!("{} ", _i);

    // 2. outter variable
    while _i>0 {
        _i -= 1;
        let _i=5;
        print!(":{} ", _i);
    }
    print!("{} ", _i);
}
