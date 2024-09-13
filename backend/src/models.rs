use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Fazenda {
    pub id: i32,
    pub nome: String,
    pub matricula: String,
    pub localizacao: Option<String>,
    pub dono: Option<String>,
    pub area: Option<f32>,
    pub tipo_plantacao: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewFazenda {
    pub nome: String,
    pub matricula: String,
    pub localizacao: Option<String>,
    pub dono: Option<String>,
    pub area: Option<f32>,
    pub tipo_plantacao: Option<String>,
}
