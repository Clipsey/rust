use std::io::stdin;

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new("bert", VisitorAction::Accept, 16),
        Visitor::new(
            "mori",
            VisitorAction::AcceptWithNote {
                note: "Calli".to_string(),
            },
            25,
        ),
        Visitor::new("john", VisitorAction::Refuse, 55),
    ];

    loop {
        println!("Hello, what's your name?");
        let name = what_is_your_name();

        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

        match known_visitor {
            Some(visitor) => match &visitor.action {
                VisitorAction::Accept => {
                    println!("Welcome to the show, {}", visitor.name);
                    if visitor.age < 21 {
                        println!("Note: No Alcohol")
                    }
                }
                VisitorAction::AcceptWithNote { note } => {
                    println!("Welcome to the show, {} (Note: {})", visitor.name, note);
                    if visitor.age < 21 {
                        println!("Note: No Alcohol")
                    }
                }
                VisitorAction::Probation => println!("Watch yourself, {}", visitor.name),
                VisitorAction::Refuse => println!("Nope, leave {}", visitor.name),
            },
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is not on the visitors list. Adding.", name);
                    visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 0));
                }
            }
        }
    }

    println!("The final list of visitors: ");
    println!("{:#?}", visitor_list);
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();

    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");

    your_name.trim().to_lowercase()
}
