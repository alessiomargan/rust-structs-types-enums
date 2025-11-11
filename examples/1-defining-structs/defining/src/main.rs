
#[derive(Debug)]
//#[allow(dead_code)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

impl Person {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    
    fn is_adult(&self) -> bool {
        self.age >= 18
    }
}

fn main() {
    let person = Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 25,
    };
   
    println!("{:?}", person);
    println!("Full Name: {}", person.full_name());
    println!("Is Adult: {}", person.is_adult());
}