fn main(){
    let x = 5;

    let x = x +1;

    {
        let x = x * 2;
        println!("O valor de 'x' no escopo do meio é: {}", x);

    }

    println! ("O valor de x é: {}", x);

    /*const TRES_HORAS_EM_SEGUNDOS: u32 = 60*60*3;
    println! ("3 Horas em segundos: {}", TRES_HORAS_EM_SEGUNDOS);*/


}