use std::io;
use std::mem;
use std::fmt;
use rand::Rng;

fn main() {
    //#[allow(dead_code)]
    println!("Hello, world!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess : ");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    if guess == secret_number.to_string() {
        println!("You win!");
    } else {
        println!("You lose! The secret number was: {}", secret_number);
    }
    
    struct Structure(i32);

    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    println!("This struct `{}` won't print...", Structure(3));    

    let number: u32 = 30; 
    //let number2: u8 = 256; // i8 is not large enough to hold 256
    println!("number is: {number}");
    //println!("number2 is: {}", number2);
    println!("Max u8: {}, min u8: {}", std::u8::MAX, std::u8::MIN);
    //println!("but size of number2 is: {} bytes", mem::size_of_val(&number2));

    struct Vecteur(i32, i32, i32); 
    let v = Vecteur(1, 2, 3);
    println!("v is: ({}, {}, {})", v.0, v.1, v.2); 

    impl fmt::Display for Vecteur {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {}, {})", self.0, self.1, self.2)
        }
    }

    println!("v is: {}", v); 

    struct Point {
        x: i32,
        y: i32,
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    struct MinMax(i32, i32);
    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if self.0 < self.1 {
                write!(f, "Max: {}, Min: {}", self.1, self.0)
            } else {
                write!(f, "Max: {}, Min: {}", self.0, self.1)
            }
        }
    }

    println!("{}", MinMax(0, 14));
    println!("{}", MinMax(4, 2));


    struct Eleve {
        nom: String,
        age: u8,
        note: f32,
    }

    impl fmt::Display for Eleve {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "L'eleve s'appelle {}, il a {} ans\n Sa moyenne est de {}", self.nom, self.age, self.note)
        }
    }
    impl fmt::Debug for Eleve {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Eleve {{ nom: {}, age: {}, note: {} }}", self.nom, self.age, self.note)
        }
    }

    let e1 = Eleve { nom: String::from("Alice"), age: 20, note: 15.5 };
    let e2 = Eleve { nom: String::from("Bob"), age: 22, note: 13.0 };

    println!("{}", e1);
    println!("{:?}", e2);

    println!("Size of e1 is: {} bytes", mem::size_of_val(&e1));

    println!("-----------------------------------");
    println!("{}", Color { red: 128, green: 255, blue: 90 });
    println!("{}", Color { red: 0, green: 3, blue: 254 });
    println!("{}", Color { red: 0, green: 0, blue: 0 });


}

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RGB ({}, {}, {}) 0x{:02X}{:02X}{:02X}", self.red, self.green, self.blue, self.red, self.green, self.blue)
    }
}