use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use std::env;

mod models;
mod db;

use models::{Fazenda, NewFazenda};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::establish_connection().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/fazendas", web::get().to(list_fazendas))
            .route("/fazendas", web::post().to(create_fazenda))
            .route("/fazendas/{id}", web::get().to(get_fazenda))
            .route("/fazendas/{id}", web::delete().to(delete_fazenda))
            .route("/fazendas/{id}", web::put().to(update_fazenda))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// Listar Fazendas
async fn list_fazendas(pool: web::Data<PgPool>) -> impl Responder {
    let fazendas = sqlx::query_as::<_, Fazenda>("SELECT * FROM fazendas")
        .fetch_all(pool.get_ref())
        .await
        .unwrap();
    
    HttpResponse::Ok().json(fazendas)
}

// Criar Fazenda
async fn create_fazenda(
    pool: web::Data<PgPool>, 
    form: web::Json<NewFazenda>
) -> impl Responder {
    let _ = sqlx::query(
        "INSERT INTO fazendas (nome, matricula, localizacao, dono, area, tipo_plantacao) 
         VALUES ($1, $2, $3, $4, $5, $6)"
    )
    .bind(&form.nome)
    .bind(&form.matricula)
    .bind(&form.localizacao)
    .bind(&form.dono)
    .bind(&form.area)
    .bind(&form.tipo_plantacao)
    .execute(pool.get_ref())
    .await;
    
    HttpResponse::Ok().json("Fazenda criada com sucesso!")
}

// Obter uma Fazenda
async fn get_fazenda(pool: web::Data<PgPool>, fazenda_id: web::Path<i32>) -> impl Responder {
    let fazenda = sqlx::query_as::<_, Fazenda>("SELECT * FROM fazendas WHERE id = $1")
        .bind(fazenda_id.into_inner())
        .fetch_one(pool.get_ref())
        .await
        .unwrap();

    HttpResponse::Ok().json(fazenda)
}

// Deletar uma Fazenda
async fn delete_fazenda(pool: web::Data<PgPool>, fazenda_id: web::Path<i32>) -> impl Responder {
    let _ = sqlx::query("DELETE FROM fazendas WHERE id = $1")
        .bind(fazenda_id.into_inner())
        .execute(pool.get_ref())
        .await;

    HttpResponse::Ok().json("Fazenda deletada com sucesso!")
}

// Atualizar uma Fazenda
async fn update_fazenda(
    pool: web::Data<PgPool>, 
    fazenda_id: web::Path<i32>, 
    form: web::Json<NewFazenda>
) -> impl Responder {
    let _ = sqlx::query(
        "UPDATE fazendas 
         SET nome = $1, matricula = $2, localizacao = $3, dono = $4, area = $5, tipo_plantacao = $6 
         WHERE id = $7"
    )
    .bind(&form.nome)
    .bind(&form.matricula)
    .bind(&form.localizacao)
    .bind(&form.dono)
    .bind(&form.area)
    .bind(&form.tipo_plantacao)
    .bind(fazenda_id.into_inner())
    .execute(pool.get_ref())
    .await;

    HttpResponse::Ok().json("Fazenda atualizada com sucesso!")
}
