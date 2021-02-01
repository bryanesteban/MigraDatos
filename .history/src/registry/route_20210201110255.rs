use crate::note::Note;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;
#[get("/registry")]
async fn find_all() -> impl Responder {
    HttpResponse::Ok().json(vec![
        Persona{
           codigo: 1,
           identificacion : "1721347562".to_string(),
           nombre: "Bryan Vasquez".to_string(),
           genero: "M".to_string(),
           estadocivil: "Casado".to_string(),
           fecha_nacimiento: "1994/01/02".to_string(),
           telefono: "5930987609531".to_string(),
           direccion: "Tumbaco Rumihuaico".to_string(),
           email: "bevasquez@espe.edu.ec".to_string(),
           valido: "1".to_string(),
           observacion:"Valido".to_string()
        },
        Persona{
            codigo: 2,
            identificacion : "1722347562".to_string(),
            nombre: "Gissel Andrade".to_string(),
            genero: "F".to_string(),
            estadocivil: "Casada".to_string(),
            fecha_nacimiento: "1992/08/29".to_string(),
            telefono: "5930982607916".to_string(),
            direccion: "Tumbaco Rumihuaico".to_string(),
            email: "gandrade@udla.edu.ec".to_string(),
            valido: "1".to_string(),
            observacion:"Valido".to_string()
        },
    ])
}
#[get("/registry/{codigo}")]
async fn find() -> impl Responder {
    HttpResponse::Ok().json(Persona{
       codigo: 2,
            identificacion : "1722347562".to_string(),
            nombre: "Gissel Andrade".to_string(),
            genero: "F".to_string(),
            estadocivil: "Casada".to_string(),
            fecha_nacimiento: "1992/08/29".to_string(),
            telefono: "5930982607916".to_string(),
            direccion: "Tumbaco Rumihuaico".to_string(),
            email: "gandrade@udla.edu.ec".to_string(),
            valido: "1".to_string(),
            observacion:"Valido".to_string()
    })
}
pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
}