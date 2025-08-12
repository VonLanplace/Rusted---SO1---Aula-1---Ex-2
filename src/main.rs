use std::{io::stdin, process::Output};

fn main() {
    let texto = input_str("Insira o Texto");
    resposta(texto);
}
fn input_str(texto: &str) -> String {
    let mut input = String::new();
    loop {
        println!("{texto}");
        if stdin().read_line(&mut input).is_err() {
            println!("Erro de leitura!!");
            continue;
        };
        break;
    }
    input.trim().to_owned()
}
fn resposta(texto: String) {
    let palavras = texto.split(";");
    let mut i = 0;
    for _palavra in palavras {
        i += 1;
    }
    println!("Tem {i} palavras.");
}
/*
2. Fazer uma aplicação Java em Eclipse que tenha uma operação que se permita entrar com um
texto, conforme exemplo abaixo, por Scanner ou JOptionPane, divida o texto em partes, com
split e exiba quantas partes aquele texto tem. A aplicação deve ter uma classe de controle
com métodos para operações e uma classe de visão que instancie a classe de controle para
a comunicação, A resposta da tarefa deve ser o print do console com as quantidades.
Texto 1: abóbora;abobrinha;alcachofra;aspargos;batata-doce;berinjela;beterraba
Texto 2: abacate;ameixa;amora;banana;cajá;figo;maçã;melancia;uva;seriguela;manga
Texto 3: acelga;alface;alho-poró;coentro;endívia;escarola;repolho;rúcula;agriãok
*/
