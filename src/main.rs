use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Jogo de adivinhação");

    let numero_gerado = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Escreva um número: ");

        let mut numero = String::new();

        io::stdin()
            .read_line(&mut numero)
            .expect("Falha em ler linha.");

        let numero: u32 = match numero.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Você digitou {}", numero);

        match numero.cmp(&numero_gerado) {
            Ordering::Less => println!("Adivinhação muito baixa."),
            Ordering::Greater => println!("Adivinhação muito alta."),
            Ordering::Equal => {
                println!("Você adivinhou!");
                break;
            }
        }
    }
}
