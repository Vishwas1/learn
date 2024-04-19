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
fn struct_keyword(){
    enum BoxShape {
        Oval,
        Rectangle
    }
    struct ShippingBox {
        depth: i32,
        width: i32,
        height: i32,
        shape: BoxShape,
    }

    let my_box = ShippingBox {
        depth: 3,
        width: 1,
        height: 1,
        shape: BoxShape::Rectangle
    };

    // println!("the box is {:?}", my_box.shape);

    match my_box.shape {
        BoxShape::Rectangle => println!("box shape Is rectangle"),
        BoxShape::Oval => println!("box shape Is oval"),
        _ => println!("box shape is shapeless"),
    }
}

fn tuples_keyword(){
    // a type of record
    // no need to name fields
    // can be "destructured" easily into variables
    // useful to retun pairs of data from function
    let coord = (2, 3);
    println!("{:?} , {:?}", coord.0,coord.1);


    let (x, y) = coord; 
    println!("{:?} , {:?}", x,y);

    let user_info = ("Emma", 20);
    let (name, age) = user_info;
    println!("Username: {:?} , Age: {:?}", name,age);

    // fn tuples() -> (i32, i32, i32) {
    //     (1, 2, 3)
    // }
}
fn expressions(){
    // expression allows nested logic 

    enum Access {
        Admin,
        Manager, 
        User,
        Guest
    }

    // secret files: admin only
    let access_level = Access::Admin;
    // here we are assigning expression to a variable
    let can_access_file = match access_level {
        Access::Admin => true,
        _ => false,
    };

    // here we are using that expression in form of variable
    println!("Can access secret file - {:?}", can_access_file);

    // -------------

    let my_num = 1;
    let is_lt_5 = if my_num < 5 {
        true
    } else {
        false
    };

    let is_lt_5 = my_num < 5; // true/false


    let message = match my_num {
        1 => "hello",
        _ => "hi"
    };
    println!("{message}")
}

fn owernship(){
    // Rust uses ownership model for memory management
    // The "ownwer" of data  must clean up the mempory
    // The cleanup  occurs automatically at the end of the scope
    // Default bahaviour is to "move" memory to new onwer in new scope. e.g passing the variable as function param
    // howeever, we can use "&" to allow code to "borrow" (refernce) memeory

    struct Book {
        id: i32,
        pages: i32
    }

    let book: Book = Book {
        id : 1,
        pages: 200
    };

    // ---------------------------
    // fn printBookId(book: Book){
    //     println!("{}", book.id);
    // }
    // fn printBookPages(book: Book){
    //     println!("{}", book.pages);
    // }
    // printBookId(book); // this will pass
    // printBookPages(book); // this shall fail since onwership of `book` is already moved to function `printBookId()`
   
    // ---------------------------
    // hence use `&` 
    // book is not "moved" but "borrowed" by printBookId()
    // so ownership of user still remains within scope of `owernship()`
    fn printBookId(book: &Book){
        println!("{}", book.id);
    }
    fn printBookPages(book: &Book){
        println!("{}", book.pages);
    }
    printBookId(&book);
    printBookPages(&book);
}

fn impl_keyword1(){
    struct Temperature {
        degree_f: f64,
    }

    // impl keyword followed by name of the struct
    // impl Temperature {
    //     fn show_temp(temp: Temperature){
    //         println!("{}", temp.degree_f);
    //     }
    // }
    // let hot: Temperature = Temperature { degree_f: 99.9 };
    // Temperature::show_temp(hot)

    // More clearner way is to use `self` keywork
    // impl Temperature {
    //     fn show_temp(&self){
    //         println!("{}", self.degree_f);
    //     }
    // }
    // let hot: Temperature = Temperature { degree_f: 99.9 };
    // hot.show_temp()

    // 
    impl Temperature {
        fn freezing() -> Self {
            Self { degree_f: 32.0 }
        }

        fn boiling() -> Self {
            Self { degree_f: 100.0 }
        }

        fn show_temp(&self){
            println!("{}", self.degree_f);
        }
    }
    let hot: Temperature = Temperature { degree_f: 99.9 };
    hot.show_temp();

    let cold = Temperature::freezing();
    cold.show_temp();

    let boiling = Temperature::boiling();
    boiling.show_temp();
    
}

fn impl_keyword() {
    enum Color {
        Red,
        Black
    }

    impl Color {
        fn print(&self){
            match self {
                Color::Red => println!("Color: Red"),
                Color::Black => println!("Color: Black"),
            }
        }
    }
    struct Dimensions {
        width: f64,
        height: f64,
        depth: f64
    } 

    impl Dimensions {
        fn print(&self){
            println!("width: {:?}", self.width);
            println!("height: {:?}",  self.height);
            println!("depth: {:?}",  self.depth);
        }
    }

    struct ShippingBox {
        dimensions: Dimensions,
        weight: f64,
        color: Color
    }

    impl ShippingBox {
        fn new(weight: f64, dimensions: Dimensions,  color: Color) -> Self {
            Self {
                dimensions,
                weight,
                color
            }
        }

        fn print(&self){
            self.color.print();
            self.dimensions.print();
            println!("weight: {:?}", self.weight);
        }
    }


    let small_dimensions = Dimensions {
        width: 1.0,
        height: 1.0,
        depth: 1.0,
    };

    let small_box = ShippingBox::new(5.0, small_dimensions, Color::Red);
    small_box.print()
}

fn vector_keyword() {
    // vectors allowes to store multple pieces of similar data
    // use for..in to interate through vector
    // used for lists of informations 
    // allows data ot be added , moved etc

    // Declaration
    let my_numbers1 = vec![1,2,4];
    // or
    let mut my_numbers = Vec::new();

    my_numbers.push(1);
    my_numbers.push(2);
    my_numbers.push(3);
    //my_numbers.pop();
    my_numbers.len();

    // accesssing data
    // by index
    let two = my_numbers[1];
    // by iterations
    for num in my_numbers{
        println!("{:?}", num);
    }
}

fn vector_demo(){
    struct Test {
        score: i32,
    }

    let my_scores: Vec<Test> = vec![
        Test { score: 90 }, 
        Test { score: 88 }, 
        Test { score: 77 }, 
    ];

    
    // itegrating over vector
    // for score in my_scores {
    //     println!("{:?}", score.score);
    // }
	// Error: now we are trying to access len() but the vairable my_scores has already been "moved" to for loop
	// println!("lenght {:?}", my_scores.len());


    // to fix this we gonna just pass the reference ("borrowed") to/by the for loop
    for score in &my_scores {
        println!("{:?}", score.score);
    }
    // now it should work
    println!("lenght {:?}", my_scores.len());
}

/// Explains about string type
fn string_type(){
    // Two commmonly used types of strings
        // 1. String - owned
        // 2. &str - borrowed String slice
    // Must use an owned String to store in struct
    // Use &str when passing to a function
    // Use .to_owned() or String:from() to create an owned copy of a string slice
    // Use an owned String when stroing in a struct

    fn print_it(data: &str){
        println!("{:?}", data);
    }

    print_it("It is a borrowed string");
    let owned_string = "Owned string".to_owned();
    let another_owned_string = String::from("another owned string");
    print_it(&owned_string);
    print_it(&another_owned_string);


    struct Employee {
        name: String,
    }
    
    let emp_name = String::from("Vishwas");
    let empployee = Employee {
        name: emp_name
    };
    print_it(&empployee.name);

    let another_emp_name = "Vikram".to_owned();
    let another_employee = Employee {
        name: another_emp_name
    };
    print_it(&another_employee.name);
}
fn result_data_type(){
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E)
    // }

    #[derive(Debug)]
    enum MenuChoice {
        MainMenu,
        Start,
        Quit
    }
    fn get_choice(input: &str) -> Result<MenuChoice, String> {
        match input {
            "mainmenu" => Ok(MenuChoice::MainMenu),
            "start" => Ok(MenuChoice::Start),
            "quit" => Ok(MenuChoice::Quit),
            _ => Err("Invliad menu type".to_owned()),
        }
    }

    fn print_choice(choice: &MenuChoice){
        println!("choice {:?} ", choice)
    }



    let choice: Result<MenuChoice, _> = get_choice("1mainmenu");
    match choice {
        Ok(inner_choice) => print_choice(&inner_choice),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn hash_map(){
    use std::collections::HashMap;
    // store data as key value pairs
    // 

    let mut people = HashMap::new();
    people.insert("Susan", 21);
    people.insert("Vishwas", 22);

    // iterate through all key values in hashmap
    for (person, age) in people.iter(){
        println!("Person = {:?}, age = {:?}", person, age);
    }

    let t = format!("{:?}", people.get("Vishwas"));
    println!("{:?}", t)

} 

fn traits_keyword(){
    // traits are basically abstract class


    struct NewsArticle {
        author: String,
        headline: String,
        content: String,
    }
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {}", self.headline, self.author)
        }
    }

    struct Tweet {
        username: String,
        content: String,
        reply: bool,
        retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}, by {}", self.content, self.username)
        }
    }

    trait Summary {
        fn summarize(&self)-> String;
    }


    let article = NewsArticle {
        author: "Vishwas".to_owned(),
        headline: "The sky is Falling".to_owned(),
        content: String::from("The is not actually Falling")
    };

    let tweet  = Tweet {
        username: "Vishwas".to_owned(),
        content: "Hello, world!".to_owned(),
        reply: true,
        retweet: false,
    };


    println!("Article Summary: {}", article.summarize());
    println!("Tweet Summary: {}", tweet.summarize());
}

fn main() {
    match_expression();
    loop_statement();
    while_loop_statement();
    enum_keyword();
    struct_keyword();
    tuples_keyword();
    expression();
    ownership();
    impl_keyword();
    vector_keyword();
    string_type();
    result_data_type();
    hash_map();
    traits_keyword();

}

