use actix_web::{web, App, HttpResponse, HttpServer};
use chrono::{DateTime, Utc};

use serde::Serialize;
use sqlx::FromRow;
use sqlx::PgPool;
use std::{collections::HashMap, fmt::Debug};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[derive(Serialize, FromRow, Debug)]
struct Subscription {
    id: Uuid,
    email: String,
    name: String,
    subscribed_at: DateTime<Utc>,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    // insert to db and return as json
    let row = sqlx::query_as::<_, Subscription>(
        "INSERT INTO subscriptions (email, name) VALUES ($1, $2) RETURNING id, email, name, subscribed_at",
    )
    .bind(&form.email)
    .bind(&form.name)
    .fetch_one(pool.as_ref())
    .await
    .unwrap();

    HttpResponse::Ok().json(row)
}

pub async fn get_subscription(pool: web::Data<PgPool>) -> HttpResponse {
    // get all from subscriptions and return as json
    let rows = sqlx::query_as::<_, Subscription>("SELECT * from subscriptions")
        .fetch_all(pool.as_ref())
        .await
        .unwrap();

    println!("{}", serde_json::to_string_pretty(&rows).unwrap());

    HttpResponse::Ok().json(rows)
}
