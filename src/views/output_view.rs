pub struct OutputView;

impl OutputView {
    // Método para exibir uma mensagem de boas-vindas
    pub fn exibir_mensagem_boas_vindas() {
        println!("Bem-vindo ao cepSolar!");
    }

    // Método para exibir a incidência solar e a quantidade de placas necessárias
    pub fn exibir_resultados(cep: &str, incidencia_solar: f64, placas_necessarias: usize) {
        println!("Incidência solar para o CEP {} é {:.2} kWh/m².", cep, incidencia_solar);
        println!("Para essa incidência solar, serão necessárias {} placas.", placas_necessarias);
    }

    // Método para exibir mensagens de erro
    pub fn exibir_erro(mensagem: &str) {
        eprintln!("Erro: {}", mensagem);
    }
}
