use std::collections::HashMap;

fn main() {
    let mut marks = HashMap::new();

    marks.insert("Rust Programming", 96);
    // marks.insert("Web Development", 94);
    marks.insert("UX Design", 75);

    println!("How many subjects have you studies {} ?",marks.len());

    match marks.get("Web Development") {
        Some(mark) => {
            println!("You got {} for Web Dev!", mark)
        },
        None => println!("You got nothing!")
    }

    for (key,val) in &marks {
        println!("Subject is {}, score is {}",key,val);
    }
}