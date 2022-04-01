use std::io; //Biblioteca

fn main (){

    println!("Adivinhe o Número");

    println!("Entre com seu palpite.");

    let mut guess = String::new(); //Variável que recebe o valor

    io::stdin() //Função que receberá o input (Função da biblioteca) 
        .read_line(&mut guess) //Linha que coleta o input do usuário
        .expect("Falha ao ler a linha"); //Linha de teste?

    println!("Você colocou: {}", guess); // {} segura valores como um tipo de vetor
}