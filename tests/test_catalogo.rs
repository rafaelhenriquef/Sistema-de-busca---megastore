use megastore::catalogo::Catalogo;

#[test]
fn test_adicionar_produto() {
    let mut catalogo = Catalogo::new();
    catalogo.adicionar("Produto Teste", "Categoria Teste");
    let produtos = catalogo.listar();
    assert_eq!(produtos.len(), 1);
    assert_eq!(produtos[0].nome, "Produto Teste");
}
