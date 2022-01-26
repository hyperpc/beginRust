
# 第3章 命名对象

## workspace

> mkdir demo  
> cd demo  
> cd.>Cargo.toml  

```toml
[workspace]
members = [
    "adder",
]
```

> cargo new adder  
> cd adder/src  
> 编写rust代码main.rs  
> rustc main.rs  
> main  

## 变量

> mkdir demo2  
> cd demo2  
> cd.>demo.rs  
> 编写rust代码demo.rs  
> rustc demo.rs  
> main  

### 变量声明

变量声明的一般过程：  

- 保留一块足够大的对象(即allocation分配一块内存区域，deallocation取消分配)，以包含一个整数  
- 以二进制格式将值存储在该对象中  
- 将变量名与对象关联，以便后续编码的引用  

变量类型和赋值

```rust
    // 声明静态变量，一旦赋值，一般不可更改；除非重新再声明一次
    let number1 = 12; 
    // 声明可变变量，
    let mut number3 = 12;
    let mut _number = 12;

    // 变量类型必须严格一致
    _numer = 12.;// 此行代码将报类型报错

    // 变量类型
    println!("12-(7%5)={}",12-(7%5));
    println!("12.-(7.%5.)={}",12.-(7.%5.));
    println!("12.-(7%5)={}",12.-(7%5)); // data type error
    
    let x = 120;
    print!("x={} ", x);
    let x = "abcd";
    print!("x={} ", x);
    let mut x = true;
    print!("x={} ", x);
    x = !x;
    print!("x={} ", x);

    // 赋值算术运算符
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
```

### 布尔

布尔运算符，与C语言的布尔运算符相同：
> ==, !=, <, >, <=, >=  

两个变量使用比较运算符时，变量类型必须严格一致  
数值类型，比较的时大小；字符串；欸行，比较的是字典顺序  

逻辑连接词: !(非), %%(且), ||(或)  
优先级: ! > && > ||，但是括号()可以改变优先级顺序  


```rust
    let mut truth = true;
    let mut falsity = false;
    println!("{} {} {}", truth, falsity, -50<6);

    println!("{} {}", truth || truth && !truth, (truth || truth) && !truth);
```
