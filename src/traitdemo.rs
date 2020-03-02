struct Person {
    name : String,
    age : u8
}

trait HasVoicBox {
    fn speak(&self);

    fn can_speak(&self) -> bool;
}

impl HasVoicBox for Person {
    fn speak(&self) {
        println!("hello , my name is {}",self.name)
    }

     fn can_speak(&self) -> bool {
        return self.age > 0;
    }
}

fn main() {
    let person = Person {
        name : String::from("Bob"),
        age : 12
    };
    println!("Can {} speak? {}",person.name,person.can_speak());
}