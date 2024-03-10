// Importe as bibliotecas externas necessárias
extern crate reqwest;

use std::io::{self, Write};

// Função para buscar a incidência solar pelo CEP
fn buscar_incidencia_solar(cep: &str) -> Result<f64, reqwest::Error> {
    // Aqui você implementaria a lógica para fazer a requisição à API que fornece dados de incidência solar com base no CEP
    // Esta é apenas uma estrutura básica, você deve substituir pelo código real
    
    // Exemplo: Faz uma requisição GET à API
    let response = reqwest::blocking::get(&format!("https://exemplo.com/api/cep/{}/solar", cep))?;

    // Verifica se a requisição foi bem sucedida
    if response.status().is_success() {
        // Se a requisição for bem sucedida, você pode extrair os dados da resposta e retorná-los
        // Aqui é onde você deve interpretar a resposta da API e extrair a incidência solar
        // Este é apenas um exemplo, você deve adaptá-lo à sua API real
        let solar_data: f64 = response.text()?.parse().unwrap();
        Ok(solar_data)
    } else {
        // Se a requisição não for bem sucedida, retorna um erro
        Err(reqwest::Error::from(response.status()))
    }
}

// Função para calcular a quantidade de placas necessárias com base na incidência solar
fn calcular_placas_necessarias(incidencia_solar: f64) -> usize {
    // Aqui você implementaria a lógica para calcular a quantidade de placas necessárias com base na incidência solar
    // Esta é apenas uma estrutura básica, você deve substituir pelo código real
    // Por exemplo:
    // let placas_necessarias = incidencia_solar / fator_de_conversao;
    // placas_necessarias.round() as usize
    10 // Exemplo de retorno fixo para fins de demonstração
}

fn main() {
    println!("Bem-vindo ao cepSolar!");

    // Solicita ao usuário que insira um CEP
    println!("Por favor, insira o CEP:");
    let mut cep = String::new();
    io::stdin().read_line(&mut cep).expect("Falha ao ler a entrada do usuário.");

    // Remove espaços em branco e quebras de linha extras do CEP
    cep = cep.trim().to_string();

    // Chama a função para buscar a incidência solar pelo CEP
    match buscar_incidencia_solar(&cep) {
        Ok(incidencia_solar) => {
            println!("Incidência solar para o CEP {} é {} kWh/m².", cep, incidencia_solar);
            // Chama a função para calcular a quantidade de placas necessárias
            let placas_necessarias = calcular_placas_necessarias(incidencia_solar);
            println!("Para essa incidência solar, serão necessárias {} placas.", placas_necessarias);
        },
        Err(e) => {
            eprintln!("Erro ao buscar a incidência solar para o CEP {}: {}", cep, e);
            std::process::exit(1);
        }
    }
}
