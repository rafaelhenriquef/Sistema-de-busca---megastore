# Sistema de busca para catálogo de produtos

## projeto
Sistema de busca para o catálogo de produtos da loja fictícia MegaStore, permitindo consultas e filtragem por nome.

## tecnologias utilizadas
- Rust
- Crates: hashbrown, serde, serde_json
- Ferramentas de teste: cargo test

## execução
terminal

cargo run


## execução dos testes
terminal

cargo test


## utilização do sistema
- ao abrir o sistema, busque por um produto como por exemplo: "notebook"
- o sistema irá mostrar se está disponível.
- caso o produto não esteja disponível, o sistema irá responder que não foi encontrado.

## arquitetura do sistema
- `catalogo.rs`: gerenciamento de produtos
- `busca.rs`: lógica de busca
- `main.rs`: ponto de entrada

## estruturas de dados
- uso de `HashMap` para indexação eficiente

## contribuições
pull requests são bem-vindos
