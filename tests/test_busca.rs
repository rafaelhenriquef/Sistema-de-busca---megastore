use megastore::catalogo::Catalogo;
use megastore::busca::buscar_produtos;

#[test]
fn test_busca_varios_produtos() {
    let mut catalogo = Catalogo::new();
    catalogo.adicionar("Notebook", "Eletrônicos");
    catalogo.adicionar("Computador", "Eletrônicos");
    catalogo.adicionar("Fone de ouvido", "Periféricos");

    let resultados = buscar_produtos(&catalogo, "Computador");
    assert_eq!(resultados.len(), 1);
    assert_eq!(resultados[0].nome, "Computador");
}
