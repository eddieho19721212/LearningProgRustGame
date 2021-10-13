use std::io::{stdin,};

struct Visitor {
    name:String,
    greeting:String
}

impl Visitor {
    fn new(p_strName: &str, p_strGreeting: &str) ->Self{
        Self {
            name: p_strName.to_string(),
            greeting:p_strGreeting.to_string()
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting)
    }
}

fn what_is_your_name() -> String {
    let mut name = String::new();
    stdin().read_line(&mut name).expect("Fail to read input");
    name.trim().to_lowercase()
}

fn main() {
    println!("Hello, what's your name?");
    let name = what_is_your_name();

    let visitor_list = [
        Visitor::new("bert", "Hello Bert, enjoy your treehouse."),
        Visitor::new("steve","Hi Steve. Your milk is in the fridge."),
        Visitor::new("fred", "Wow, who invited Fred?"),
    ];

    let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);
    match known_visitor {
        Some(visitor)=>visitor.greet_visitor(),
        None => println!("You are not on the visitor list. Please leave.")
    }

    //println!("Hello, {} ", name);
}
