enum Continent{
    Europe,
    Asia,
    Africa,
    America,
    Oceania,
}

enum CardinalPoint{ North, South, West, East }

enum Result{
    Success(f64),
    Failure(u16, char),
    Uncertainly,
}

fn main(){
    //
    println!("##### enum_demo #####");
    enum_demo();
    
    println!("##### match_switch #####");
    match_switch();
    
    println!("##### match_all #####");
    match_all();
    
    println!("##### match_non_enum #####");
    match_non_enum();
    
    println!("##### match_result #####");
    match_result();
    
    println!("##### match_expr #####");
    match_expr();
    
    println!("##### match_guard #####");
    match_guard();
}

fn match_guard(){
    for n in -2..5{
        println!("{} is {}", n, match n{
            0 => "zero",
            1 => "one",
            _ if n<0 => "negative",
            _ => "plural",
        });
    }
}

fn match_expr(){
    let direction = CardinalPoint::South;
    print!("{} ", match direction{
        CardinalPoint::North => 'N',
        CardinalPoint::South => 'S',
        _ => '*',
    });
}

fn match_result(){
    let outcome_1 = Result::Success(23.67);
    match outcome_1{
        Result::Success(value) => print!("Result: {} ", value),
        Result::Failure(error_code, module) => print!("Error n. {} in module {} ", error_code, module),
        Result::Uncertainly => {},
    }
    
    let outcome_2 = Result::Success(23.67);
    match outcome_2{
        Result::Success(_) => print!("OK "),
        Result::Failure(error_code, module) => print!("Error n. {} in module {} ", error_code, module),
        Result::Uncertainly => {},
    }
    
    let outcome_3 = Result::Failure(1200, 'X');
    match outcome_3{
        Result::Success(value) => print!("Result: {} ", value),
        Result::Failure(error_code, module) => print!("Error n. {} in module {} ", error_code, module),
        Result::Uncertainly => {},
    }
}

fn match_non_enum(){
    match "value"{
        "val"=>print!("value "),
        _ => print!("other "),
    }

    match 3{
        3 => print!("three "),
        4 => print!("four "),
        5 => print!("five "),
        _ => print!("other "),
    }

    match '.'{
        ':' => print!("colon "),
        '.' => print!("point "),
        _ => print!("other "),
    }
}

fn match_all(){
    let direction = CardinalPoint::South;
    // 1.
    match direction{
        CardinalPoint::North=> print!("NORTH"),
        CardinalPoint::South=> print!("SOUTH"),
        CardinalPoint::East=> {},
        CardinalPoint::West=> {},
    }
    // 2.
    match direction{
        CardinalPoint::North=> print!("NORTH"),
        CardinalPoint::South=> print!("SOUTH"),
        _=> {},
    }
}

fn match_switch(){
    let mut contin = Continent::Asia;
    match contin{
        Continent::Europe=>{
            contin = Continent::Asia;
            print!("E");
        },
        Continent::Asia=>{let _a = 3.14;},
        Continent::Africa=>print!("Af"),
        Continent::America=>print!("Am"),
        Continent::Oceania=>print!("O"),
    }

}

fn enum_demo(){
    let contin = Continent::Asia;
    match contin{
        Continent::Europe=>print!("E"),
        Continent::Asia=>print!("As"),
        Continent::Africa=>print!("Af"),
        Continent::America=>print!("Am"),
        Continent::Oceania=>print!("O"),
    }

    //let n:i32 = Continent::Asia; // error: expected i32, found enum `main::Continent`
    //let m:Continent = 1;         // error: expected `main::Continent`, found integer variable 
}