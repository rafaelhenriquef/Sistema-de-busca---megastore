use std::io;
mod catalogo;
mod busca;

use catalogo::Catalogo;
use busca::buscar_produtos;

fn main() {
    let mut catalogo = Catalogo::new();
    catalogo.adicionar("Notebook", "Eletrônicos");
    catalogo.adicionar("Jogo de videogame", "Videogames");
    catalogo.adicionar("Computador", "Eletrônicos");
    catalogo.adicionar("Fone de ouvido", "Periféricos");
    catalogo.adicionar("Celular", "Eletrônicos");

    println!("Sistema de busca da MegaStore.");
    println!("Digite o nome do produto para buscar ou 'sair'.");

    loop {
        let mut termo = String::new();
        print!("> ");
        let _ = io::Write::flush(&mut io::stdout()); 
        io::stdin()
            .read_line(&mut termo)
            .expect("Erro");

        let termo = termo.trim();

        if termo.eq_ignore_ascii_case("sair") {
            println!("Encerrando o sistema. Até mais!");
            break;
        }

        let resultados = buscar_produtos(&catalogo, termo);

        if resultados.is_empty() {
            println!("Nenhum produto encontrado para '{}'.", termo);
        } else {
            for produto in resultados {
                println!("Produto encontrado: {} - {}", produto.nome, produto.categoria);
            }
        }
    }
}
