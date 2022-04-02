use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main(){

    
    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..101);

        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler a linha");

    let guess: u32 = guess.trim().parse().expect("Por favor digite um número");

    println!("Você adivinhou: {}",guess);

    match guess.cmp(&secret_number){

        Ordering::Less => println!("Muito pequeno"),
        Ordering::Greater => println! ("Muito grande"),
        Ordering::Equal => print! ("Você ganhou"),
    }
}