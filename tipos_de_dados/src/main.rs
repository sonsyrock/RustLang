use std::io;

fn main(){
    let a = [1,2,3,4,5];

    println! ("Insira um indice do array.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Falha ao ler a linha");

    let index: usize = index
    .trim()
    .parse()
    .expect("Indice colocado não é um número");

    let element = a[index];

    println! (
        "O valor do elemento do indice {} é: {}", index,element
    );

}



/*fn main() {
   
    let soma = 5 + 10;

    let diferenca = 95.5 - 4.3;

    let multiplicacao = 4 * 30;

    let quotacao = 56.7 / 32.2;

    let floored = 2 / 3;

    let porcentagem = 43 % 5;

    //Booleano

    let t = true;

    let f: bool = false; 

    println! ("{}",f);

    println! ("{}",t);

    //Char

    let nome = 'G'; //Só aceita 1 valor (uma letra)

    /*Tupla (Possuí o tamanho limitado de acordo com quantos valores está na tupla)
      Aceita tipos diferentes de valores
    */

    let tupla: (i32,f64,u8) = (500, 6.4,1);

    /*Array não é Vetor(Tamanho é limitado! diferente de outras linguagens de programação e
     não aceita tipos diferentes de valores) Usamos o Array quando sabemos a quantidade de
     elementos que estão dentro dele*/

    let array = [1,2,3,4,5];

    let array_b: [i32;5] = [1,2,3,4,5]; //array do tipo int com o tamanho máximo de 5 setores

    let array_c = [3;5]; //array já iniciado com o valor "3" e terá o tamanho máximo de 5 setores todos começando com "3"

    let primeiro = array_c[0]; //Para consultar um array, precisamos converter dessa forma
    let segundo = array_c[1];


    println! ("{}",primeiro);

}
*/