fn main() {
    // Exemplo para Exercício 1: Trabalhando com Strings
    let palavras = vec![
        "rust".to_string(), 
        "codigo".to_string(), 
        "rust".to_string(), 
        "tutorial".to_string(),
        "rust".to_string()
    ];
    let alvo = "rust";

    println!("--- EXERCÍCIOS PROPOSTOS ---");
    println!("Vetor de busca: {:?}", palavras);
    println!("Alvo: \"{}\"\n", alvo);

    // Execução dos exercícios 2 e 4
    let inicio = Instant::now();
    let posicoes = buscar_todas_posicoes(&palavras, alvo);
    let duracao = inicio.elapsed();

    println!("Exercício 2 & 4: Resultados");
    println!("* Posições encontradas: {:?}", posicoes); [cite: 233]
    println!("* Total de ocorrências: {}", posicoes.len()); [cite: 229]
    println!("* Tempo de execução: {:?}", duracao);
}

/// Exercício 1 e 4: Busca genérica que retorna todas as posições encontradas
/// Implementado para aceitar &str para atender ao requisito de Strings 
fn buscar_todas_posicoes(vetor: &[String], alvo: &str) -> Vec<usize> {
    let mut encontradas = Vec::new();
    
    for (i, item) in vetor.iter().enumerate() {
        if item == alvo {
            encontradas.push(i); [cite: 233]
        }
    }
    
    encontradas
}