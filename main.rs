use std::io::{self, BufRead};

fn main() {
    println!("Een monster zit je achterna.");
    println!("Je komt bij een T splitsing.");
    println!("Het is erg spannend!");
    println!("Wat wil je doen?");
    println!("A) Links");
    println!("B) Rechts");
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    if input.trim() == "A" {
        println!("Links was fout! Je bent nu dood. GAME OVER :-(");
    } else if input.trim() == "B" {
        println!("Rechts was goed! Je bent ontsnapt! GAME WON!");
    } else {
        println!("Foute input! Hierdoor: GAME OVER :-)");
    }
}
