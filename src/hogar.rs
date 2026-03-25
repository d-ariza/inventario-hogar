use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Hogar {
    pub personas: u32,
    pub periodo_dias: u32,
    pub categoria: String,
    pub moneda: String,
    pub total_mensual_aproximado: u64,
    pub total_redondeado_decena_mil: u64,
}

impl Default for Hogar {
    fn default() -> Self {
        Self {
            personas: 1,
            periodo_dias: 30,
            categoria: "general".to_string(),
            moneda: "COP".to_string(),
            total_mensual_aproximado: 0,
            total_redondeado_decena_mil: 0,
        }
    }
}

impl Hogar {
    pub fn new(personas: u32, periodo_dias: u32, categoria: String, moneda: String) -> Self {
        Self {
            personas,
            periodo_dias,
            categoria,
            moneda,
            total_mensual_aproximado: 0,
            total_redondeado_decena_mil: 0,
        }
    }

    pub fn calcular_totales(&mut self, total: u64) {
        self.total_mensual_aproximado = total;
        self.total_redondeado_decena_mil = ((total + 5000) / 10000) * 10000;
    }
}