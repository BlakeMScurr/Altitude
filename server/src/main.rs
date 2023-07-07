use futures_lite::stream::StreamExt;

use crate::model::QueryRoot;
use crate::merkle::get_new_merkle_tree;
use crate::routes::{graphql_handler, graphql_playground, health};
use async_graphql::{EmptySubscription, Schema};
use axum::{extract::Extension, routing::get, Router, Server};
use lapin::publisher_confirm::Confirmation;
use std::sync::Arc;
use futures::lock::Mutex;
use model::MutationRoot;
use tokio_postgres::NoTls;
use lapin::{
    options::*, types::FieldTable, BasicProperties, Connection, ConnectionProperties,
};
mod model;
mod routes;
mod merkle;

#[tokio::main]
async fn main() {
    /////////////// experimental postgresql integration ///////////////
    // Connect to the database.
    let (client, connection) = tokio_postgres::connect("host=localhost user=dev dbname=arcpay", NoTls).await.unwrap();

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let rows = client
        .query("SELECT * FROM state_tree", &[])
        .await.unwrap();

    dbg!(&rows);

    for row in rows {
        let leaf: i64 = row.get(0);
        let owner: i32 = row.get(1);
        let coin_low: i64 = row.get(2);
        let coin_high: i64 = row.get(3);
        let index: i32 = row.get(4);

        // Process the retrieved values as needed
        println!("leaf: {}, owner: {}, coin_low: {}, coin_high: {}, index: {}", leaf, owner, coin_low, coin_high, index);
    }
    ///////////////////////////////////////////////////////////////////
    //////////////// experimental RabbitMQ integration ////////////////
    // Connect to the RabbitMQ server
    let addr = "amqp://guest:guest@localhost:5672/%2f";
    let conn = Connection::connect(addr, ConnectionProperties::default()).await.unwrap();

    // Create a channel
    let channel = conn.create_channel().await.unwrap();

    // Declare a queue
    let queue_name = "send_request_queue";
    channel
        .queue_declare(queue_name, QueueDeclareOptions::default(), FieldTable::default())
        .await.unwrap();

    // Consume messages from the queue
    let mut consumer = channel
        .basic_consume(
            queue_name,
            "my_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await.unwrap();

    println!("Waiting for messages...");

    // Process incoming messages in a separate thread.
    tokio::spawn(async move {
        while let Some(delivery) = consumer.next().await {
            let delivery = delivery.expect("error in consumer");
            delivery
                .ack(BasicAckOptions::default())
                .await
                .expect("ack");
            // dbg!(delivery); // uncomment if you want to see continuous dump.
        }
    });

    // Publish messages to the queue in a separate thread.
    let message = b"Hello, RabbitMQ!";
    tokio::spawn(async move {
        loop {
            let confirm = channel
                .basic_publish(
                    "",
                    queue_name,
                    BasicPublishOptions::default(),
                    message,
                    BasicProperties::default(),
                )
                .await.unwrap().await.unwrap();
            assert_eq!(confirm, Confirmation::NotRequested);
        }
    });
    ///////////////////////////////////////////////////////////////////
    //////////////// experimental GraphQL integration /////////////////

    let mt = get_new_merkle_tree(2);
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(Arc::new(Mutex::new(mt)))
        .finish();
    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .route("/health", get(health))
        .layer(Extension(schema));
    Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
