#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]

use rand::Rng;
use std::fmt;
use std::io;
use std::mem;
use std::time::Instant;
use std::hint::black_box;

mod pi_calc;
mod UART;
mod fibo;

use crate::UART::*;
use crate::fibo::*; // Import everything from fibo module
use crate::pi_calc::*;

fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

enum Error {
    UART_INIT_ERROR,
    SPI_INIT_ERROR,
    I2C_INIT_ERROR,
    NO_ERROR,
    NB_ERROR,
}

enum CarType {
    Sedan,
    SUV,
    Coupe,
    Hatchback,
    Convertible,
    Truck,
    Van,
}

impl fmt::Debug for CarType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let type_str = match self {
            CarType::Sedan => "Sedan",
            CarType::SUV => "SUV",
            CarType::Coupe => "Coupe",
            CarType::Hatchback => "Hatchback",
            CarType::Convertible => "Convertible",
            CarType::Truck => "Truck",
            CarType::Van => "Van",
        };
        write!(f, "{}", type_str)
    }
}

fn main() {
    //#[allow(dead_code)]
    println!("Hello, world!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess : ");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    if guess == secret_number.to_string() {
        println!("You win!");
    } else {
        println!("You lose! The secret number was: {}", secret_number);
    }

    println!("-----------------------------------");

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

    println!("-----------------------------------");

    struct Vecteur(i32, i32, i32);
    let v = Vecteur(1, 2, 3);
    println!("v is: ({}, {}, {})", v.0, v.1, v.2);

    impl fmt::Display for Vecteur {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {}, {})", self.0, self.1, self.2)
        }
    }

    println!("v is: {}", v);

    println!("-----------------------------------");
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

    println!("-----------------------------------");
    struct Eleve {
        nom: String,
        age: u8,
        note: f32,
    }

    impl fmt::Display for Eleve {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "L'eleve s'appelle {}, il a {} ans\n Sa moyenne est de {}",
                self.nom, self.age, self.note
            )
        }
    }
    impl fmt::Debug for Eleve {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "Eleve {{ nom: {}, age: {}, note: {} }}",
                self.nom, self.age, self.note
            )
        }
    }

    let e1: Eleve = Eleve {
        nom: String::from("Alice"),
        age: 20,
        note: 15.5,
    };
    let e2: Eleve = Eleve {
        nom: String::from("Bob"),
        age: 22,
        note: 13.0,
    };

    println!("{}", e1);
    println!("{:?}", e2);

    println!("Size of e1 is: {} bytes", mem::size_of_val(&e1));

    println!("-----------------------------------");

    println!(
        "{}",
        Color {
            red: 128,
            green: 255,
            blue: 90
        }
    );
    println!(
        "{}",
        Color {
            red: 0,
            green: 3,
            blue: 254
        }
    );
    println!(
        "{}",
        Color {
            red: 0,
            green: 0,
            blue: 0
        }
    );

    println!("-----------------------------------");

    // Fixed-size array (type signature is superfluous).
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value.
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0.
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    // `len` returns the count of elements in the array.
    println!("Number of elements in array: {}", xs.len());

    // Arrays are stack allocated.
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices.
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    // Slices can point to a section of an array.
    // They are of the form [starting_index..ending_index].
    // `starting_index` is the first position in the slice.
    // `ending_index` is one more than the last position in the slice.
    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1..4]);

    // Example of empty slice `&[]`:
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // Same but more verbose

    // Arrays can be safely accessed using `.get`, which returns an
    // `Option`. This can be matched as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happily continue.
    for i in 0..xs.len() + 1 {
        // Oops, one element too far!
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

    println!("-----------------------------------");

    // Out of bound indexing on array with constant value causes compile time error.
    //println!("{}", xs[5]);
    // Out of bound indexing on slice causes runtime error.
    //println!("{}", xs[..][5]);

    fn check_error(err: Error) {
        match err {
            Error::UART_INIT_ERROR => println!("UART init error"),
            Error::SPI_INIT_ERROR => println!("SPI init error"),
            Error::I2C_INIT_ERROR => println!("I2C init error"),
            Error::NO_ERROR => println!("No error"),
            Error::NB_ERROR => println!("Number of errors"),
        }
    }

    check_error(Error::SPI_INIT_ERROR);

    println!("-----------------------------------");

    type Personne = Eleve;
    let p1: Personne = Personne {
        nom: String::from("Charlie"),
        age: 23,
        note: 14.0,
    };
    println!("{}", p1);

    println!("-----------------------------------");

    struct Voiture {
        marque: String,
        modele: String,
        annee: u16,
        puissance: u16,
        consommation: f32,
        km: u32,
        categorie: CarType,
    }
    impl fmt::Display for Voiture {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "{} {} ({}), {} ch, {} L/100km, {} km",
                self.marque, self.modele, self.annee, self.puissance, self.consommation, self.km
            )
        }
    }

    impl fmt::Debug for Voiture {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "Voiture {{ marque: {}, modele: {}, annee: {}, puissance: {}, consommation: {}, km: {}, categorie: {:?} }}",
                self.marque,
                self.modele,
                self.annee,
                self.puissance,
                self.consommation,
                self.km,
                self.categorie
            )
        }
    }

    let v1: Voiture = Voiture {
        marque: String::from("Toyota"),
        modele: String::from("Corolla"),
        annee: 2020,
        puissance: 132,
        consommation: 6.5,
        km: 15000,
        categorie: CarType::Sedan,
    };

    use crate::CarType::*;

    let v2: Voiture = Voiture {
        marque: String::from("Ford"),
        modele: String::from("Mustang"),
        annee: 2019,
        puissance: 450,
        consommation: 12.0,
        km: 30000,
        categorie: Coupe,
    };

    println!("{}", v1);
    println!("{:?}", v2);

    println!("------------------------------------");

    let config = UART::UART_Config {
        baud_rate: UART::UART_BaudRate::Baud115200,
        parity: UART::UART_Parity::None,
        stop_bits: UART::UART_StopBits::One,
    };

    let uart_result = UART::UART_init(config);

    match uart_result {
        Ok(uart) => {
            println!("UART initialized with config: {:?}", uart.config);
            // Further UART operations can be performed here
        }
        Err(e) => {
            if e != UART_Error::NO_ERROR {
                println!("No error during UART initialization.");
            } else {
                println!("An error occurred during UART initialization: {:?}", e);
            }
        }
    }

    println!("------------------------------------");
    
    fn fibo_calc_all(n: u128) {

    // Fast doubling
    let t4 = Instant::now();
    let fd = black_box(fib_fast_doubling(n as u128));
    let tfd = t4.elapsed();
    println!("fib_fd({n}) = {fd}, took {:?}", tfd);

    // Itérative
    let t1 = Instant::now();
    let i = black_box(fibonacci_iterative(n as u128));
    let ti = t1.elapsed();
    println!("fib_iter({n}) = {i}, took {:?}", ti);
    
    // Matrices
    let t2 = Instant::now();
    let m = black_box(fib_matrix(n as u128));
    let tm = t2.elapsed();
    println!("fib_mat({n}) = {m}, took {:?}", tm);

    // Mémoïsation
    let t3 = Instant::now();
    let me = black_box(fib_memo(n as usize));
    let tme = t3.elapsed();
    println!("fib_me({n}) = {me}, took {:?}", tme);

    // Récursive (naïve)
    // let t0 = Instant::now();
    // let r = black_box(fibonacci_recursive(n as u128));
    // let tr = t0.elapsed();
    // println!("fib_rec({n}) = {r}, took {:?}", tr);

    }

    for n in 1..100 {
        fibo_calc_all(n);
        println!("------------------------------------");
    }

    for n in [10, 50, 100, 200, 500, 1000, 2000, 5000, 10000] {
        let pi = pi_calculator(n);
        println!("Approximation of pi with n={} terms: {:.15}", n, pi);
        println!("------------------------------------");
    }
}


struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RGB ({}, {}, {}) 0x{:02X}{:02X}{:02X}",
            self.red, self.green, self.blue, self.red, self.green, self.blue
        )
    }
}

// End of main.rs
