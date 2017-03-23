/*
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
*/
fn main(){

let teststring1 = "x a e i a d ks fd";
let teststring2 = "as fg d a r w m b";

let mut cou1 = 0;
let mut cou2 = 0;

for b in teststring1.chars() {
    
    cou1 += 1;

    println!("{}, ", b);
    println!("Anzahl bits {}", cou1);
    }

for b in teststring2.chars() {
    
    cou2 += 1;

    println!("{}, ", b);
    println!("Anzahl bits {}", cou2);
    }

let mut cryp = "".to_string();

for b in teststring1.as_bytes(){

    let mut a = b;

    cryp.push_str("{}", a);

    }

//cryp.push_str(", teststring1.as_bytes ");

println!("{}", cryp);

}

/*
let mut file = File::open("test.txt")?;
let mut contents = String::new();

file.read_to_string(&mut contents)?;

println!("Inhalt des Files {}", contents);
*/
/*
let mut s = "Hello".to_string(); // mut s: String
println!("{}", s)

s.push_str(", world.");
println!("{}", s);

}
*/

/*
//function for read from file to string
fn name(arg: Type) -> RetType {
    let path = Path::new("test.txt");
let display = path.display();

let mut file = match File::open(&path){
    Err(why) => panic!("couldn't open {}: {}", display, why.description()),
    Ok(file) => file,
};

let mut contents = String::new();

match file.read_to_string(&mut contents) {
    Err(why) => panic!("couldn't read {}: {}", display, why.description()),
    Ok(_) => print!("{} contains:\n{}", display, contents),
    }
}

//function for count length of the string
fn name(arg: Type) -> RetType {

}

//function for confert string to vector and asci to hex
fn name(arg: Type) -> RetType {
    unimplemented!();
}

//function for printing out a the string an vector
fn name(arg: Type) -> RetType {
    unimplemented!();
}
*/