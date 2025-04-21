/*

Extender a livraria anterior para poder ter livros, audio books, estátuas e quadros.
Deve manter as mesmas capacidades.
Deve também ter o máximo de elementos comuns em zonas partilhadas,
utilizando composição para partilhar o máximo de código possivel
(por exemplo, todos os elementos têm título e autor, mas apenas os audio books têm durações,
apenas as estátuas e quadros têm dimensões físicas).

*/

use std::collections::HashMap;
use std::io;

enum Item {
    Livro(Livro),
    AudioBook(AudioBook),
    Estatuas(Estatua),
    Quadros(Quadro),
}

#[derive(Debug)]
struct MediaGeral {
    titulo: String,
    autor: String,
    localizacao: String,
}

#[derive(Debug)]
struct MediaTextualOuAuditiva {
    palavras_chave: Vec<String>,
}

#[derive(Debug)]
struct MediaFisica {
    dimensao: (f32, f32, f32), // largura, altura, profundidade (em metros)
    peso: f32,                 // peso em kg
}

#[derive(Debug)]
struct MediaEmprestavel {
    exemplares_em_stock: u32,
    exemplares_emprestados: u32,
}

#[derive(Debug)]
struct Livro {
    media_geral: MediaGeral,
    media_textual_ou_auditiva: MediaTextualOuAuditiva,
    media_emprestavel: MediaEmprestavel,
    media_fisica: MediaFisica,
    isbn: String,
}

#[derive(Debug)]
struct AudioBook {
    media_geral: MediaGeral,
    media_textual_ou_auditiva: MediaTextualOuAuditiva,
    media_emprestavel: MediaEmprestavel,
    duracao: f32, // duração em horas
}

#[derive(Debug)]
struct Estatua {
    media_geral: MediaGeral,
    media_fisica: MediaFisica,
}

#[derive(Debug)]
struct Quadro {
    media_geral: MediaGeral,
    media_fisica: MediaFisica,
}

impl MediaEmprestavel {
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
                livro.media_emprestavel.exemplares_em_stock += 1;
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
            Some(livro) => return livro.media_emprestavel.requesitar(None),
            None => {
                println!("Livro not in stock");
                false
            }
        }
    }

    fn devolver_livro(&mut self, isbn: &str) -> bool {
        let livro_option = self.livros.get_mut(isbn);
        match livro_option {
            Some(livro) => return livro.media_emprestavel.devolver(None),
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
                livro.media_geral.titulo,
                livro.media_geral.autor,
                livro.media_emprestavel.exemplares_em_stock,
                livro.media_emprestavel.exemplares_emprestados
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
                let livro = Livro {
                    media_geral: MediaGeral {
                        titulo: titulo.clone(),
                        autor: autor.clone(),
                        localizacao: String::from("Livraria"),
                    },
                    media_textual_ou_auditiva: MediaTextualOuAuditiva {
                        palavras_chave: palavras_chave.clone(),
                    },
                    media_emprestavel: MediaEmprestavel {
                        exemplares_em_stock: quantidade,
                        exemplares_emprestados: 0,
                    },
                    media_fisica: MediaFisica {
                        dimensao: (0.0, 0.0, 0.0), // Não aplicável para livros
                        peso: 0.0,                 // Não aplicável para livros
                    },
                    isbn: isbn.clone(),
                };
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
