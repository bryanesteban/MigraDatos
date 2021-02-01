// src/user/model.rs
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct Persona {
    pub codigo: i32,
    pub identificacion: String,
    pub nombre: String,
    pub genero: String,
    pub estadocivil: String,
    pub fecha_nacimiento: String,
    pub telefono: String,
    pub direccion: String,
    pub email: String,
    pub valido: bool,
    pub observacion: String,
}