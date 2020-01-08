use std::io;

mod integer_list;
mod str_pig_latin;
mod employees;

// pub use crate::integer_list;
// pub use crate::integer_list::int_list;
// pub use crate::str_pig_latin::str_pig;
pub use crate::employees::append;

struct MenuAnswer {
    quit: bool,
    error: bool,
}

fn main() {
    let mut answer = MenuAnswer {
        quit: false,
        error: false,
    };
    display_menu(answer.error);
    loop {
        let mut opt = String::new();
        io::stdin().read_line(&mut opt)
            .expect("Failed to read the line.");

        println!("\nChoice: {}", opt.trim().to_ascii_lowercase().as_str());
        match opt.trim().parse() {
            Ok(ch) => {
                answer = select_entry(ch);
            },
            Err(_) => {
                answer.error = true;
                println!("Unknown value!");
            },
        };

        if answer.quit {
            break;
        } else {
            display_menu(answer.error);
            continue;
        }
    }
}

fn display_menu(error: bool) {
    if error {
        println!("Valid choices:");
    } else {
        println!("Menu\n====");
        println!("Choose an exercice:");
    }
    println!("\t1) info on a list of integers,");
    println!("\t2) strings to pig latin,");
    println!("\t3) text interface to add employees.");
    println!("\nh) this menu,\nq) quit.\n\n");
}

fn select_entry(opt: char) -> MenuAnswer {
    let mut answer = MenuAnswer {
        quit: false,
        error: false,
    };
    match opt {
        '1' => integer_list::main(),
        '2' => str_pig_latin::main(),
        '3' => append::test(),
        'h' => display_menu(answer.error),
        'q' => {
            println!("Goodbye!");
            answer.quit = true;
        },
        _ => {
            println!("Unknown value!");
            answer.error = true;
        },
    }
    answer
}
