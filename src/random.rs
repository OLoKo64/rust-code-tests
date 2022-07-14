use rayon::prelude::*;

#[derive(Debug)]
pub enum Teste {
    A,
    B(i32),
    C { x: i32, y: i32 },
}

pub struct Teste2 {
    pub a: Teste,
}

pub fn random () {
    let mut count = 0;
    let vec = vec![1, 2, 3, 5, 6, 7, 8, 9, 10];
    let teste: Vec<i32> = vec.par_iter().map(|&i| ten_times(i) ).collect();

    println!("{:?}", teste);
    
    // Testing Enums and Enum Structs
    let a = Teste::B(1);
    let b = Teste::C{x: 10, y:11};
    let c = Teste2 { a: Teste::C{x: 10, y:11} };
    if let Teste::B(x) = a {
        println!("{}", x);
    }
    if let Teste::C{x, y} = b {
        println!("{}", x);
        println!("{}", y);
    }
    println!("{:?}", b);
    println!("{:?}", c.a);

    let add = |x: i32, y: i32| x + y;

    println!("{}", add(1, 2));
    loop {
        println!("Before -> {}", count);
        count += 1;
        if count == 5 {
            assert_eq!(count, 5);
            break;
        }
        println!("After -> {}", count);
    }

    match optional() {
        Some(x) => println!("{}", x),
        None => println!("None"),
    }
}

pub fn ten_times(value: i32) -> i32 {
    value * 10
}

pub fn optional() -> Option<String> {
    let value = 10; 
    if value == 102 {
        Some("10".to_string())
    } else {
        None
    }
}
