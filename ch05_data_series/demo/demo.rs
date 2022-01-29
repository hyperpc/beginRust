fn main(){
    // 1. array
    array_demo();

    // 2. vector
    vector_demo();

    // 3. print array, not via for loop
    print_array();
}

fn array_demo(){
    // default array
    let x = ["English", "This", "sentence", "a", "in", "is"];
    println!("{} {} {} {} {} {}", x[1], x[5], x[3], x[2], x[4], x[0]);

    let a = [true, false];
    let b = [1,2,3,4,5];
    println!("{}, {}", a.len(), b.len());

    //let x = ["This", 4]; // Error: type error
    //let x = [4, 5.]; // Error: type error

    let mut _x = ["a"]; // array of strings
    //x[0] = 3; // Error: type error
    //x[-1] = "b"; // Error: index error
    //x[0.] = "b"; // Error: index error
    //x[false] = "b"; // Error: index error
    //x["0"] = "b"; // Error: index error

    let x = ["a"];
    let _y = x[0];

    // mut array
    let mut x = ["This", "is", "a", "sentence"];
    x[2] = "a nice";
    println!("{} {} {} {}.", x[0], x[1], x[2], x[3]);

    let mut x = ["a", "b", "c"];
    println!("{} {} {}", x[0], x[1], x[2]);
    x = ["X", "Y", "Z"];
    println!("{} {} {}", x[0], x[1], x[2]);
    x = ["1", "2", "3"];
    println!("{} {} {}", x[0], x[1], x[2]);

    // array size
    let mut x = [4.; 5000];
    x[2000] = 3.14;
    println!("{} {}", x[1000], x[2000]);

    let mut fib = [1; 15];
    for i in 2..fib.len(){
        fib[i] = fib[i-2]+fib[i-1];
    }
    println!("");
    for i in 0..fib.len(){
        print!("{}, ", fib[i]);
    }

    // multi-dim array
    let mut x = [[[[23; 4]; 6]; 8]; 15];
    x[14][7][5][3]=56;
    println!("{}, {}", x[0][0][0][0], x[14][7][5][3]);
    println!("{}, {}, {}, {}", x.len(), x[0].len(), x[0][0].len(), x[0][0][0].len());

    // empty array
    let _a = [""; 0];

    // copy
    println!("");
    let mut a1 = [4,56, -2];
    let a2 = [7, 81, 12500];
    print!("{:?}", a1);
    a1=a2;
    print!("{:?}", a1);

}

fn vector_demo(){
    println!("");
    let x = vec!["This", "is"];
    print!("{} {}. Length is {}.", x[0], x[1], x.len());
    let mut x = vec!["This", "is"];
    x.push("a"); 
    print!(" {}.", x.len());
    x.push("sentence"); 
    print!(" {}.", x.len());
    x[0] = "That";
    println!("");
    for i in 0..x.len(){
        print!(" {}", x[i]);
    }
    
    println!("");
    let length = 5000;
    let mut y = vec![4.; length];
    y[6] = 3.14;
    y.push(4.86);
    println!("{}, {}, {}", y[6], y[4999], y[5000]);

    // same data type, diff length
    let mut _x = vec!["a", "b", "c"];
    _x=vec!["X", "Y"];

    // others
    let mut x = vec!["This", "is", "a", "sentence"];
    x.insert(1, "line");
    x.insert(2, "contains");
    x.remove(3);
    x.push("about Rust");
    x.pop();
    for i in 0..x.len(){
        print!("{} ", x[i]);
    }

    // empty vector
    let _a = vec![true; 0];
    let _b = vec![false; 0];
    
    // copy vector
    println!("");
    let mut a1 = vec![4,56, -2];
    let a2 = vec![7, 81, 12500];
    print!("{:?}", a1);
    a1=a2;
    print!("{:?}", a1);
    let a2 = vec![7, 81];
    a1=a2;
    print!("{:?}", a1);
}

fn print_array(){
    println!("");
    print!("{:?}, {:?}", [1,2,3], vec![4,5]);
}
