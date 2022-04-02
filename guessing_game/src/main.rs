use std::io; //Biblioteca
use rand::Rng; //Números aleátorios
use std::cmp::Ordering;

fn main (){

    println!("Adivinhe o Número");

    let numero_secreto = rand::thread_rng().gen_range(1..101);

    println!("O Número Secreto é: {}", numero_secreto); // {} segura valores como um tipo de vetor

    println! ("Insira o seu palpite.");

    let mut guess = String::new(); //Variável que recebe o valor

    io::stdin() //Função que receberá o input (Função da biblioteca) 
        .read_line(&mut guess) //Linha que coleta o input do usuário
        .expect("Falha ao ler a linha"); //Linha de teste?

    println! ("Você colocou: {}", guess);

    
}