
# 第1章 终端打印

## 安装

官网地址: [https://www.rust-lang.org/zh-CN/tools/install](https://www.rust-lang.org/zh-CN/tools/install)  

检验安装结果：
> rustc -V  
> rustc --version  
> mkdir demo  
> cd demo  
> 编写rust代码demo.rs  
> rustc demo.rs  
> demo  

## 命令行脚本

Windows / rs.bat:

```bat
@echo off
cls
rustc %* --color always 2>&1 | more
```

> rs demo.rs  

Linux / rs.sh:

```shell
clear
rustc $* --color always 2>&1 | more
```

## 注释

单行注释： //  
多行注释: /*(支持匹配的嵌套)*/
