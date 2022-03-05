
fn main(){
    println!("##### 1. scope #####");
    f();
    fn_scope();

    println!("##### 2. param #####");
    print_sum(3., 5.);
    print_sum(3.2, 5.1);
    let x = 4.;
    print_double(x);
    println!("x={}", x);
    
    println!("##### 3. return #####");
    println!("f1()={}", f1());
    println!("f2()={}", f2());
    println!("f3()={}", f3());
    println!("{}", double(17.3));
    println!("f_good(1.)={}, f_good(-1.)={}", f_good(1.),f_good(-1.));
    println!("f_normal(1.)={}, f_normal(-1.)={}", f_normal(1.),f_normal(-1.));
    println!("f_bad(1.)={}, f_bad(-1.)={}", f_bad(1.),f_bad(-1.));
    f_none(5);
    println!("divide(50, 11)={:?}", divide(50, 11));
    //println!("{}", match f_multi1(){E::E1 =>1, E::E2=>0, _=>-1});
    println!("{}", match f_multi1_e1(){E::E1 =>1, E::E2=>0});
    println!("{}", match f_multi1_e2(){E::E1 =>1, E::E2=>0});
    println!("a={}, b={}", f_multi2().a, f_multi2().b);
    println!("{}", f_multi3().0);
    println!("{}", f_multi4()[0]);
    println!("{}", f_multi5()[0]);
    
    println!("##### 4. mut return #####");
    let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
    println!("arr={:?}", arr);
    arr = double_negatives_basic(arr);
    println!("double_negatives_basic(arr)={:?}", arr);

    println!("##### 5. mut ref #####");
    println!("arr={:?}", arr);
    double_negatives_adv_1(&mut arr);
    println!("arr={:?}", arr);
    double_negatives_adv_2(&mut arr);
    println!("arr={:?}", arr);
    f_ref();
    f_ref_mut();    
}

fn f(){
    println!("1");
}
fn fn_scope(){
    f(); // 2
    {
        f(); // 3
        fn f(){
            println!("3");
        }
    }
    f(); // 2
    fn f(){
            println!("2");
    }
}

fn print_sum(addend1:f64, addend2:f64){
    println!("{} + {} = {}", addend1, addend2, addend1+addend2);
}

fn print_double(mut x:f64){
    x*=2.;
    println!("x * 2. = {}", x);
}

fn double(x:f64) -> f64{ x*2. }

fn f1() -> i32 {4.5; "abc"; 73i32}
fn f2() -> i32 {4.5; "abc"; 73}
fn f3() -> i32 {4.5; "abc"; 73+100}

/*
fn f1_1() -> i32 {4.5; "abc"; false}
fn f1_2() -> i32 {4.5; "abc"; ()}
fn f1_3() -> i32 {4.5; "abc"; {}}
fn f1_4() -> i32 {4.5; "abc"; }
*/

fn f_good(x:f64) ->f64 {
    if x<=0. { 0. }
    else { x+3. }
}

fn f_normal(x:f64) ->f64 {
    if x<=0. {return 0.;}
    x+3.
}

fn f_bad(x:f64) ->f64 {
    if x<=0. {return 0.;}
    return x+3.
}

fn f_none(x:i32){
    if x<=0 {return;}
    if x == 4 {return ();}
    if x == 7 {return {};}
    println!("{}", x);
}

fn divide(dividend:i32, divisor:i32) -> (i32, i32){
    (dividend/divisor, dividend%divisor)
}

enum E{E1, E2}
struct S {a:i32, b:bool}
struct TS(f64, char);
fn f_multi1_e1() -> E {E::E1}
fn f_multi1_e2() -> E {E::E2}
fn f_multi2() -> S{ S{a:49, b:true}}
fn f_multi3() -> TS{TS(4.7, 'w')}
fn f_multi4() -> [i64; 4]{[7, -2, 0, 19]}
fn f_multi5() -> Vec<i64>{vec![12000]}

fn double_negatives_basic(mut a:[i32;10]) -> [i32;10]{
    for i in 0..10{
        if a[i] < 0{a[i]*=2;}
    }
    a
}

fn double_negatives_adv_1(a: &mut [i32; 10]){
    for i in 0..10{
        if (*a)[i] < 0{(*a)[i]*=2;}
    }
}

fn double_negatives_adv_2(a: &mut [i32; 10]){
    for i in 0..10{
        if a[i] < 0{a[i]*=2;}
    }
}

fn f_ref(){
    let a = 15;
    let ref_a = &a;
    println!("{} {} {}", a, *ref_a, ref_a); // 15 15 15

    let b = &&&7;
    println!("{} {} {} {}", ***b, **b, *b, b); // 7 7 7 7
}

fn f_ref_mut(){
    let mut a:i32 = 10;
    let mut b:i32 = 20;
    let mut p:&mut i32 = &mut a;
    println!("{}", *p); // 10
    *p +=1;
    println!("{}", *p); // 11
    p=&mut b;
    println!("{}", *p); // 20
    *p+=1;
    println!("{}", *p); // 21

}