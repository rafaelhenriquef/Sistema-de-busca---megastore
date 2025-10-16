use crate::catalogo::{Catalogo, Produto};

pub fn buscar_produtos(catalogo: &Catalogo, termo: &str) -> Vec<Produto> {
    catalogo.listar()
        .into_iter()
        .filter(|p| p.nome.to_lowercase().contains(&termo.to_lowercase()))
        .collect()
}
