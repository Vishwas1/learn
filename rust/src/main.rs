fn match_expression(){
    // Match is similar to switch 
    // Prefer match over else..if when working with single variable
    let some_bool = true;
    match some_bool {
        true => println!("its true"),
        false => println!("its false"),
    }

    let some_integer = 5;
    match some_integer {
        1 => println!("its 1"),
        2 => println!("its 2"),
        3 => println!("its 3"),
        _ => println!("everything else"), // default of switch
    }

    // advance match 
    enum Discount {
        Percentage(i32),
        Flat(i32)
    }

    struct Ticket {
        event: String,
        price: i32
    }

    let n = 3;
    match n {
        3 => println!("three"),
        other => println!("number: {:?}", other)
    }

    let flat = Discount::Flat(10);
    match flat {
        Discount::Flat(2) => println!("flat 2"),
        Discount::Flat(amount) => println!("flat discount of {:?}", amount),
        _ => ()
    }

    let concert  = Ticket {
        event: "concert".to_owned(),
        price: 50
    };
    match concert {
        Ticket { price, ..} => println!("Price = {:?}", price),
        Ticket { price: 50, event} => println!("event @ 50  = {:?}", event)
    }

}

fn loop_statement(){
    let mut a = 1;
    loop {
        if a == 5 {
            break;
        }
        println!("looping {a}");
        a = a + 1;
    }
}

fn while_loop_statement(){
    let mut a = 1;
    while a <= 5 {
        println!("looping {a}");
        a = a + 1;
    }
}
fn enum_keyword(){
 enum Direction {
    Up,
    Down,
    Left, 
    Right,
    Middle
 }

 let go: Direction = Direction::Left;
 match go {
    Direction::Up => println!("up"),
    Direction::Down =>  println!("down"),
    _ =>  println!("Anything else"),
 }

 // enums can also contain data
 // can aslo contain other enum
 enum Mouse {
    LeftClick,
    RightClick,
    MiddleClick,
    Scroll(i32),
    Move(i32, i32)
 }
}

fn main() {
    match_expression();
    loop_statement();
    while_loop_statement();
    enum_keyword();
}

