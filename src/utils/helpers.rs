pub struct Helpers;

impl Helpers {
    // Método para limpar a entrada do usuário removendo espaços em branco extras e quebras de linha
    pub fn limpar_entrada(entrada: &str) -> String {
        entrada.trim().to_string()
    }
}
