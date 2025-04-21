/*

Com uso de structs, referências e Vecs, fazer um software para uma livraria
(pode ter UX básica (texto) em terminal) que armazene livros e que seja capaz
de fazer as seguintes operações.

O livro terá que ter campos para:
    ISBN
    Título
    Autor
    Palavras Chave

O software terá que ter a capacidade de saber quantos exemplares temos de cada livro.

E também a capacidade de: adicionar livros, remover livros, requisitar livros e devolver livros.

*/

use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Livro {
    isbn: String,
    titulo: String,
    autor: String,
    _palavras_chave: Vec<String>,
    exemplares_em_stock: u32,
    exemplares_emprestados: u32,
}

impl Livro {
    fn novo(
        isbn: String,
        titulo: String,
        autor: String,
        palavras_chave: Vec<String>,
        quantidade: u32,
    ) -> Self {
        Livro {
            isbn,
            titulo,
            autor,
            _palavras_chave: palavras_chave,
            exemplares_em_stock: quantidade,
            exemplares_emprestados: 0,
        }
    }

    fn requesitar(&mut self, quantidade: Option<u32>) -> bool {
        let quantidade = quantidade.unwrap_or(1);
        if self.exemplares_em_stock >= quantidade {
            self.exemplares_em_stock -= quantidade;
            self.exemplares_emprestados += quantidade;
            return true;
        } else {
            return false;
        }
    }

    fn devolver(&mut self, quantidade: Option<u32>) -> bool {
        let quantidade = quantidade.unwrap_or(1);
        if self.exemplares_emprestados >= quantidade {
            self.exemplares_emprestados -= quantidade;
            self.exemplares_em_stock += quantidade;
            return true;
        } else {
            return false;
        }
    }
}

struct Livraria {
    livros: HashMap<String, Livro>, // chave: ISBN
}

impl Livraria {
    fn new() -> Self {
        Livraria {
            livros: HashMap::new(),
        }
    }

    fn adicionar_livro(&mut self, livro: Livro) {
        let isbn_copy = livro.isbn.clone();

        let livro_from_hash_map = self.livros.get_mut(&isbn_copy);
        match livro_from_hash_map {
            Some(livro) => {
                livro.exemplares_em_stock += 1;
            }

            None => {
                self.livros.insert(isbn_copy, livro);
            }
        }
    }

    fn remover_livro(&mut self, isbn: &str) -> bool {
        self.livros.remove(isbn).is_some()
    }

    fn requisitar_livro(&mut self, isbn: &str) -> bool {
        let livro_option = self.livros.get_mut(isbn);
        match livro_option {
            Some(livro) => return livro.requesitar(None),
            None => {
                println!("Livro not in stock");
                false
            }
        }
    }

    fn devolver_livro(&mut self, isbn: &str) -> bool {
        let livro_option = self.livros.get_mut(isbn);
        match livro_option {
            Some(livro) => return livro.devolver(None),
            None => {
                println!("Livro not in stock");
                false
            }
        }
    }

    fn listar_livros(&self) {
        for livro in self.livros.values() {
            println!(
                "{} - {} por {} | Stock: {} | Emprestados: {}",
                livro.isbn,
                livro.titulo,
                livro.autor,
                livro.exemplares_em_stock,
                livro.exemplares_emprestados
            );
        }
    }
}

fn main() {
    let mut livraria = Livraria::new();

    loop {
        println!(
            "\n1. Adicionar livro\n2. Remover livro\n3. Requisitar livro\n4. Devolver livro\n5. Listar livros\n6. Sair"
        );

        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).unwrap();
        let opcao = opcao.trim();

        match opcao {
            "1" => {
                let (isbn, titulo, autor, palavras_chave, quantidade) = ler_dados_livro();
                let livro = Livro::novo(isbn, titulo, autor, palavras_chave, quantidade);
                livraria.adicionar_livro(livro);
            }
            "2" => {
                let isbn = ler_input("ISBN do livro a remover: ");
                if livraria.remover_livro(&isbn) {
                    println!("Livro removido.");
                } else {
                    println!("Livro não encontrado.");
                }
            }
            "3" => {
                let isbn = ler_input("ISBN do livro a requisitar: ");
                if livraria.requisitar_livro(&isbn) {
                    println!("Livro requisitado.");
                } else {
                    println!("Livro indisponível.");
                }
            }
            "4" => {
                let isbn = ler_input("ISBN do livro a devolver: ");
                if livraria.devolver_livro(&isbn) {
                    println!("Livro devolvido.");
                } else {
                    println!("Livro não estava emprestado.");
                }
            }
            "5" => {
                livraria.listar_livros();
            }
            "6" => {
                println!("Saindo...");
                break;
            }
            _ => println!("Opção inválida."),
        }
    }
}

fn ler_dados_livro() -> (String, String, String, Vec<String>, u32) {
    let isbn = ler_input("ISBN: ");
    let titulo = ler_input("Título: ");
    let autor = ler_input("Autor: ");
    let palavras = ler_input("Palavras-chave (separadas por vírgula): ");
    let palavras_chave = palavras.split(',').map(|s| s.trim().to_string()).collect();
    let quantidade: u32 = ler_input("Quantidade em estoque: ").parse().unwrap_or(1);

    (isbn, titulo, autor, palavras_chave, quantidade)
}

fn ler_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).unwrap();
    entrada.trim().to_string()
}
