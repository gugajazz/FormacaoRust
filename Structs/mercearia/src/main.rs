/*
Defina todas as estruturas necessárias para fazer track do inventário de uma mercearia, organizado por fileiras, prateleiras e zonas dentro de cada prateleira.

Deve guardar informação relativa ao produto:
    identificador
    nome
    data de validade
    preço
    quantidade

Ter a capacidade de adicionar e remover produtos, movê-los de local dentro da mercearia, mudar o preço e o nome.
Ter a capacidade de adicionar e remover quantidade de produtos (restock).
*/

use serde::Serialize;
use std::collections::HashMap;
use std::io;

#[derive(Clone, Debug, Serialize)]
struct Item {
    nome: String,
    quantidade: u32,
    identificador: String,    // ex: "123456789"
    data_de_validade: String, // ex: "2023-12-31"
    preço: f32,               // ex: 1.99
}

impl Item {
    fn new(
        nome: String,
        quantidade: u32,
        identificador: String,
        data_de_validade: String,
        preço: f32,
    ) -> Self {
        Item {
            nome,
            quantidade,
            identificador,
            data_de_validade,
            preço,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
struct Zona {
    id: String, // ex: "A", "B", "C"
    itens: Vec<Item>,
}

impl Zona {
    fn new(id: String, itens: Vec<Item>) -> Self {
        Zona { id, itens }
    }
}

#[derive(Clone, Debug, Serialize)]
struct Prateleira {
    numero: u32,
    zonas: Vec<Zona>,
}

impl Prateleira {
    fn new(numero: u32, zonas: Vec<Zona>) -> Self {
        Prateleira { numero, zonas }
    }
}

#[derive(Clone, Debug, Serialize)]
struct Fileira {
    id: String, // ex: "F1", "F2"
    prateleiras: Vec<Prateleira>,
}

impl Fileira {
    fn new(id: String, prateleiras: Vec<Prateleira>) -> Self {
        Fileira { id, prateleiras }
    }
}

#[derive(Clone, Debug, Serialize)]
struct Mercearia {
    nome: String,
    fileiras: Vec<Fileira>,
}

impl Mercearia {
    fn new(nome: String, fileiras: Vec<Fileira>) -> Self {
        Mercearia { nome, fileiras }
    }

    fn encontrar_item_mut(&mut self, identificador: &str) -> Option<&mut Item> {
        for fileira in &mut self.fileiras {
            for prateleira in &mut fileira.prateleiras {
                for zona in &mut prateleira.zonas {
                    for item in &mut zona.itens {
                        if item.identificador == identificador {
                            return Some(item);
                        }
                    }
                }
            }
        }
        None
    }

    fn encontrar_item_index(&self, identificador: &str) -> Option<(usize, usize, usize, usize)> {
        for (i, fileira) in self.fileiras.iter().enumerate() {
            for (j, prateleira) in fileira.prateleiras.iter().enumerate() {
                for (k, zona) in prateleira.zonas.iter().enumerate() {
                    for (l, item) in zona.itens.iter().enumerate() {
                        if item.identificador == identificador {
                            return Some((i, j, k, l));
                        }
                    }
                }
            }
        }
        None
    }

    fn remover_item(&mut self, identificador: &str) -> bool {
        match self.encontrar_item_index(identificador) {
            Some((a, b, c, d)) => {
                self.fileiras[a].prateleiras[b].zonas[c]
                    .itens
                    .swap_remove(d);
                true
            }
            None => return false,
        }
    }

    fn adicionar_item(
        &mut self,
        fileira_id: &str,
        prateleira_num: u32,
        zona_id: &str,
        item: Item,
    ) -> bool {
        for fileira in &mut self.fileiras {
            if fileira.id == fileira_id {
                for prateleira in &mut fileira.prateleiras {
                    if prateleira.numero == prateleira_num {
                        for zona in &mut prateleira.zonas {
                            if zona.id == zona_id {
                                zona.itens.push(item);
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }

    fn mover_item(
        &mut self,
        identificador: &str,
        novo_fileira: &str,
        novo_prateleira: u32,
        novo_zona: &str,
    ) -> bool {
        // Procurar e clonar o item com o identificador dado
        let item_achado = self.encontrar_item_mut(identificador);

        // Se o item foi encontrado
        if let Some(item) = item_achado {
            // Clonar o item porque vamos remover o original
            let item_clonado = item.clone();

            // Tentar remover o item original
            let removido = self.remover_item(identificador);

            if removido {
                // Tentar adicionar o item na nova posição
                return self.adicionar_item(novo_fileira, novo_prateleira, novo_zona, item_clonado);
            }
        }

        // Se qualquer passo falhar, retorna false
        false
    }

    fn mudar_preco(&mut self, identificador: &str, novo_preco: f32) -> bool {
        match self.encontrar_item_mut(identificador) {
            Some(_) => {}
            None => {} // item.preço = novo_preco;
                       // return true;
        }
        false
    }

    fn mudar_nome(&mut self, identificador: &str, novo_nome: String) -> bool {
        if let Some(item) = self.encontrar_item_mut(identificador) {
            item.nome = novo_nome;
            return true;
        }
        false
    }

    fn adicionar_quantidade(&mut self, identificador: &str, quantidade: u32) -> bool {
        if let Some(item) = self.encontrar_item_mut(identificador) {
            item.quantidade += quantidade;
            return true;
        }
        false
    }

    fn remover_quantidade(&mut self, identificador: &str, quantidade: u32) -> bool {
        if let Some(item) = self.encontrar_item_mut(identificador) {
            if item.quantidade >= quantidade {
                item.quantidade -= quantidade;
                return true;
            }
        }
        false
    }
}

fn main() {
    let item1 = Item::new(
        "Arroz".to_string(),
        10,
        "123456789".to_string(),
        "2023-12-31".to_string(),
        1.99,
    );

    let item2 = Item::new(
        "Feijão".to_string(),
        5,
        "987654321".to_string(),
        "2023-11-30".to_string(),
        2.49,
    );

    let zona1 = Zona::new("A".to_string(), vec![item1]);
    let zona2 = Zona::new("B".to_string(), vec![item2]);

    let prateleira1 = Prateleira::new(1, vec![zona1]);
    let prateleira2 = Prateleira::new(2, vec![zona2]);

    let fileira1 = Fileira::new("F1".to_string(), vec![prateleira1]);
    let fileira2 = Fileira::new("F2".to_string(), vec![prateleira2]);

    let mercearia = Mercearia::new(
        "Supermercado do Bairro".to_string(),
        vec![fileira1, fileira2],
    );

    println!("{:?}", mercearia);
    // Use serde_json to pretty print
    let json = serde_json::to_string_pretty(&mercearia).unwrap();
    println!("{}", json);
}
