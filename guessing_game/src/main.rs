use rand::Rng;
use std::cmp::Ordering; //Faz comparação entre números
use std::io; //Permite que o usuário insira valores a uma variável

fn main(){
    
    
    println! ("Adivinhe o Número!");
    let secret_number = rand::thread_rng().gen_range(1..101); //Variável do número secreto que gera valores aleatórios
  
    println! ("O número secreto é: {}", secret_number);
    println! ("Favor digite um número:");

    let mut guess = String::new(); //guess é mutável e recebe o valor digitado pelo usuário
        io::stdin()
            .read_line(&mut guess) //Lê a variável
            .expect("Falha ao ler a linha");
           
    let guess: u32 = guess.trim().parse().expect("Por favor digite um número."); //verifica se foi digitado um número ou uma letra

    println! ("Você digitou: {}", guess);

    match guess.cmp(&secret_number){ //Faz a comparação entre a variável guess e secret_number
        
        Ordering::Less => println!("Muito pequeno"),
        Ordering::Greater => println! ("Muito grande"),
        Ordering::Equal => print! ("Você ganhou"),
    }
}