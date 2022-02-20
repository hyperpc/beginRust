fn main(){
    println!("##### 1 tuple #####");
    tuple_demo();
    
    println!("##### 2 struct #####");
    struct_demo();
    
    println!("##### 3 tuple and struct #####");
    tuple_struct();
}

fn tuple_struct(){
    struct SomeData (
        i32,
        f32,
        char,
        [u8; 5]
    );

    let data = SomeData(
        10_000_000,
        183.19,
        'Q',
        [9, 0, 250, 60, 200]
    );
    println!("{}, {}, {}, {}", data.2, data.0, data.1, data.3[2]);
}

fn struct_demo(){
    struct SomeData {
        integer: i32,
        fractional: f32,
        character: char,
        five_bytes: [u8;5]
    }

    let mut data = SomeData {
        integer: 10_000_000,
        fractional: 183.19,
        character: 'Q',
        five_bytes: [9, 0, 250, 60, 200]
    };

    data.five_bytes[3] = data.five_bytes[3] * 2;

    println!("{}, {}, {}, {}", data.five_bytes[3], data.integer, data.fractional, data.character);
}

fn tuple_demo(){
    let mut data = (10000000, 183.19, 'Q');
    let data2: (i32, f64, char) = data;
    println!("{}, {}, {}", data.0, data2.1, data.2);

    data.0 = -5;
    data.2='X';
    println!("{}, {}, {}", data.0, data.1, data.2);

    let data3:(u16, char, i16, f64, char, bool, i16) = (10, 'x', 12, 183.19, 'Q', false, -9);
    println!("{}", data3.2 + data3.6);
}