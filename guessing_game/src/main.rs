use rand::Rng;
use std::cmp::Ordering; //Faz comparação entre números
use std::io; //Permite que o usuário insira valores a uma variável

fn main(){
    
    
    println! ("Adivinhe o Número!");

    let secret_number = rand::thread_rng().gen_range(1..101); //Variável do número secreto que gera valores aleatórios
  
    //println! ("O número secreto é: {}", secret_number); Comentado para não aparecer o número secreto

    loop {

    println! ("Favor digite um número:");

    let mut guess = String::new(); //guess é mutável e recebe o valor digitado pelo usuário
        io::stdin()
            .read_line(&mut guess) //Lê a variável
            .expect("Falha ao ler a linha");
           
    //let guess: u32 = guess.trim().parse().expect("Por favor digite um número."); 


/*
    Assim como com o "Ordering" aqui utilizamos o match para verificar as condições, da "Ok" se for um número
    ou dá "Err" se não for e continua o programa ao invés de exibir um erro.
    Se o parse for bem sucedido (converter string para número) vai retornar o resultado com o número e mandar para "num" que depois armazena
    na variável "guess".
*/

    let guess: u32 = match guess.trim().parse(){ //verifica se foi digitado um número ou uma letra
        Ok(num) => num,
        Err(_) => continue, // O "_" foi utilizado para ele pegar qualquer tipo de valor, não importa o tipo.
    };

    println! ("Você digitou: {}", guess);

    match guess.cmp(&secret_number){ //Faz a comparação entre a variável guess e secret_number
        
        Ordering::Less => println!("Muito pequeno"),
        Ordering::Greater => println! ("Muito grande"),
        Ordering::Equal => {
            
            println! ("Você ganhou!");
            break;
            }

        }

    }

}