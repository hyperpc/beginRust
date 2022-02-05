fn main(){
    println!("##### int_spell #####");
    int_spell();

    println!("##### int_type #####");
    int_type();

    println!("##### index_usize #####");
    index_usize();

    println!("##### infer_type #####");
    infer_type();

    println!("##### float_type #####");
    float_type();

    println!("##### as_convert #####");
    as_convert();

    println!("##### empty_tuple #####");
    empty_tuple();

    println!("##### bool_char #####");
    bool_char();

    println!("##### array_vector_type #####");
    array_vector_type();

    println!("##### const_def #####");
    const_def();

    println!("##### find_data_type #####");
    //find_data_type();
}

/*
fn find_data_type(){
    let a:bool = 4u32/3u32;  // expected `bool`, found `u32`
    let b:bool = 4/3;        // expected `bool`, found integer
}
*/

fn const_def(){
    //
    //let n = 20;
    //let _ = [0;n]; // error. the length of array defined when compiled; but n is assigned value when running
    const N:usize = 20;
    let arr = [0; N];
    println!("arr[1]={}", arr[1]);
}

fn array_vector_type(){
    let array1: [char;3] = ['x', 'y', 'z'];
    print!("array1[1]={}, ", array1[1]);
    let array2: [f32;200] = [0f32; 200];
    print!("array2[1]={}, ", array2[1]);
    let vec1: Vec<char> = vec!['x', 'y', 'z'];
    print!("vec1[1]={}, ", vec1[1]);
    let vec2: Vec<i32> = vec![0; 5000];
    print!("vec2[1]={}", vec2[1]);
    println!("");
}

fn empty_tuple(){
    //
    let a:()=();           // ()
    let b={12;87;283};     // 283  //return of block is the value of the last expression, "238"
    let c={12;87;283;};    // ()   //return of block is the value of the last expression: ";" end of expression
    let d={};              // ()
    let e= if false{};     // ()   // else {}
    let f = while false{}; // ()   // else {}
    println!("{:?} {:?} {:?} {:?} {:?} {:?}", a, b, c, d, e, f);
}

fn bool_char(){
    // 
    let a:bool = true;
    let b:char = 'a';
    println!("[{}], [{}]", a, b);

    // 
    let e_grave = 'è';
    let janpanese_character = 'さ';
    println!("[{}], [{}]", e_grave, janpanese_character);

    // 
    //let _a = 'a' + 'b';    // error
    //let _b = true + false; // error
    println!("{} {} {} {} {}", true as u8, false as u8, 'A' as u32, 'à' as u32, '€' as u32);
    let truthy = 1;
    let falsy = 0;
    println!("{} {} {} {}", truthy!=0, falsy!=0, 65 as char, 224 as char);
    //println!("{} {}", truthy as bool, falsy as bool); //error: compare with zero instead: `truthy != 0`
    //let 3 as bool; // error, only 0 or 1 as bool will work
}

fn as_convert(){
    let a:i16 = 12;
    let b:u32 = 4;
    let c:f32 = 3.14;
    println!("a+b+c={}", a as i8 + b as i8 + c as i8);

    // error: literal out of range for `i8/u16`
    //let e = 500 as i8;
    //let f = 100_000 as u16;
    //let g = 10_000_000_000 as u32;
    //println!("{}, {}, {}", e, f, g);

    let m = 1;
    let n = 0;
    println!("{} {} {} {}", m, n, m!=0, n!=0);
    let truthy = true;
    let falsy = false;
    println!("{} {} {} {}", truthy, falsy, truthy as u8, falsy as u8);

    for i in 0..256{
        print!("{}:[{}], ", i, i as u8 as char);
    }
}

fn float_type(){
    let a:f64=4.6;
    let b:f32=3.91;
    println!("a={}, b={}", a, b);

    let c = 4.6;
    let mut d:f32 = 3.91e5;
    print!("d={}, ", d);
    d=c; // if remove this line code, type of c will be f64 in default
    println!("c={}, d={}", c, d);
}

fn infer_type(){
    // 
    let a = [0];
    let i = 0;
    println!("a[i]={}", a[i]); // a: array; i: usize

    let i_=1;
    let j_:u16=i_;   // i_ inferred as u16
    //let k_:i16=i_; // error: type not matched
    let k_=i_;       // k_ inferred as u16
    println!("j_={}, k_={}", j_, k_);

    let m = 8;
    //let n = 8_000_000_000; // error: literal out of range for `i32`
    let n = 8_000;
    println!("m={}, n={}", m,n);
    // literal out of range for i32
}

fn index_usize(){
    let arr = [11, 22, 33];
    let i:usize = 2;
    println!("arr[i]={}", arr[i]);
    //let i1:isize = 2;
    //println!("arr[i]={}", arr[i1]); // slice indices are of type `usize` or ranges of `usize`
    //let i2:u32 = 2; // only usize allowed
    //println!("arr[i]={}", arr[i2]);
    //let i3:u64 = 2; // only usize allowed
    //println!("arr[i]={}", arr[i3]);
}

fn int_type(){
    // integer
    let a:i8=5;
    let b:i16=5;
    let c:i32=5;
    let d:i64=5;
    println!("{} {} {} {}", a, b, c, d);
    //println!("a+b={}", a+b); // type not matched
    let e:i8=3;
    println!("a+e={}", a+e);

    // unsigned
    let a_:u8 = 5;
    let b_ = 5 as u16;
    let c_ = 5 + b_ - b_;
    let d_=5u64;
    let _e_=5_u64;
    println!("{} {} {} {} {}", a_, b_, c_, d_, _e_);
    //println!("a_+b_={}", a_+b_); // type not matched
    let _f_:u8=3;
    println!("a_+_f_={}", a_+_f_);
}

fn int_spell(){
    // base-number:数字基数
    let hexadecimal = 0x10; // 16, base-number is x
    let octal = 0o10;       // 8, base-number is o
    let binary = 0b10;      // 2, base-number is b
    let mut decimal = 10;   // 10
    println!("hexadecimal={}, octal={}, binary={}, decimal={}", hexadecimal, decimal, octal, binary);

    decimal = hexadecimal;
    println!("decimal(hexadecimal)={}", decimal);
    decimal = octal;
    println!("decimal(octal)={}", decimal);
    decimal = binary;
    println!("decimal(binary)={}", decimal);

    println!("0xA={}, 0b100000000={}", 0xA, 0b100000000);

    // underline
    let hexadecimal_ = 0x_00FF_f7a3; // 16, base-number is x
    let octal_ = 0o_777_205_162;       // 8, base-number is o
    let binary_ = 0b_0110_1001_1111_0001;      // 2, base-number is b
    let decimal_ = 1_234_567;   // 10
    println!("hexadecimal_={}, octal_={}, binary_={}, decimal_={}", hexadecimal_, decimal_, octal_, binary_);

    // e: exponential
    let one_thousand = 1e3;
    let one_million = 1e6;
    let thirteen_billions_and_half = 13.5e9;
    let twelve_millions = 12e-6;
    println!("one_thousand={}, one_million={}, thirteen_billions_and_half={}, twelve_millions={}", one_thousand, one_million, thirteen_billions_and_half, twelve_millions)
}
