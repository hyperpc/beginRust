# 第8章 使用异构数据结构

## 元组

```rust
//数组：
let array1: [char;3] = ['x', 'y', 'z'];
//向量：
let vec1: Vec<char> = vec!['x', 'y', 'z'];
//元组：
let data = (10000000, 183.19, 'Q');
let data2: (i32, f64, char) = data;
print!("{}, {}, {}", data.0, data2.1, data.2);
int i = 0;
print!("{}", data.i); // error
```

元组，是不允许通过变量索引访问其中的元素的；  
只允许通过字面量访问元组的元素  
空元组为()

## 结构

元组虽然可以靠顺序来确定元素类型，但是元素过多，就会难以理解  

```rust
    let data3:(u16, char, i16, f64, char, bool, i16) = (10, 'x', 12, 183.19, 'Q', false, -9);
    print!("{}", data3.2 + data3.6);
```

结构可以避免这种状况  
使用结构的一般顺序：

- 声明结构类型  
- 构建结构对象  
- 通过点语法使用结构对象  

```rust
    struct SomeData{
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

    print!("{}, {}, {}, {}", data.five_bytes[3], data.integer, data.fractional, data.character);
```

空结构为SomeData{}, SomeData为预先定义的结构的名字  

## 元组结构

元组：其类型没有名称且不需要事先声明，并且其字段没有名称  
结构：其类型具有名称且必须要事先声明，并且其字段具有名称  
元组结构混合体：通过struct关键字声明，但是使用元组的小括号包裹元素，且无需指定元素名称，但是元素有类型和声明顺序  
元组结构混合类型不允许为空，而且这种类型并不常用  

## 一般命名约定

- 常量名仅包含大写字母，单词之间用下划线分隔，如MAX_POWER  
- 应用程序或标准库定义的类型名称和枚举名，由首字母大写的单词组成，不需要下划线分隔，如VehicleData  
- 其他名称都用小写字母，且使用下划线分隔单词，如registration_year  
