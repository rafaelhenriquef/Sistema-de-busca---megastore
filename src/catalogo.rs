use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Produto {
    pub nome: String,
    pub categoria: String,
}

pub struct Catalogo {
    produtos: HashMap<String, Produto>,
}

impl Catalogo {
    pub fn new() -> Self {
        Catalogo {
            produtos: HashMap::new(),
        }
    }

    pub fn adicionar(&mut self, nome: &str, categoria: &str) {
        let produto = Produto {
            nome: nome.to_string(),
            categoria: categoria.to_string(),
        };
        self.produtos.insert(nome.to_string(), produto);
    }

    pub fn listar(&self) -> Vec<Produto> {
        self.produtos.values().cloned().collect()
    }
}
