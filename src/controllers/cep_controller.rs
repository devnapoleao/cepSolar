use crate::SolarData;

pub struct CepController;

impl CepController {
    // Método para lidar com a entrada do usuário e realizar as operações necessárias
    pub fn handle_user_input(cep: &str) {
        // Chama a função para buscar a incidência solar pelo CEP
        match SolarData::buscar_por_cep(cep) {
            Ok(solar_data) => {
                println!("Incidência solar para o CEP {} é {:?}", cep, solar_data);
                // Chama o método para calcular a quantidade de placas necessárias
                let placas_necessarias = solar_data.calcular_placas_necessarias();
                println!("Para essa incidência solar, serão necessárias {} placas.", placas_necessarias);
            },
            Err(e) => {
                eprintln!("Erro ao buscar a incidência solar para o CEP {}: {}", cep, e);
                std::process::exit(1);
            }
        }
    }
}
