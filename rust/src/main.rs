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

fn main() {
    match_expression()
}

