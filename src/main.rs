use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write};
use tokio::{
    sync::mpsc,
    time::{sleep, Duration},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Order {
    BUY,
    SELL,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub order: Order,
    pub ticker: String,
    pub amount: f32,
}

// Definimos un actor que recibe mensajes y los procesa
async fn actor(mut receiver: mpsc::Receiver<Message>) {
    let mut messages: Vec<Message> = Vec::new();

    while let Some(message) = receiver.recv().await {
        println!("Actor recibió el mensaje: {:?}", message);

        // Almacena los mensajes
        messages.push(message.clone());

        // Guardamos los mensajes en un archivo JSON al finalizar
        let json_data = serde_json::to_string(&messages).unwrap();
        let mut file = File::create("db/messages.json").unwrap();
        file.write_all(json_data.as_bytes()).unwrap();

        // Wait while saving
        sleep(Duration::from_secs(1)).await;
        println!("message saved with success.!");
    }
}

#[tokio::main]
async fn main() {
    // Creamos un canal para enviar mensajes entre actores
    let (tx, rx) = mpsc::channel::<Message>(1);

    // Creamos un actor que recibirá mensajes,
    // le pasamos el receptor rx estableciendo un canal de comunicación.
    tokio::spawn(async move {
        actor(rx).await;
    });

    // Buffer de mensajes para enviar al actor
    let buffer = [
        Message {
            order: Order::BUY,
            amount: 5.5,
            ticker: "BTC".to_owned(),
        },
        Message {
            order: Order::BUY,
            amount: 9.5,
            ticker: "ETH".to_owned(),
        },
        Message {
            order: Order::BUY,
            amount: 2.5,
            ticker: "PKT".to_owned(),
        },
    ];

    // Enviamos mensajes a través del canal
    for message in buffer {
        if let Err(e) = tx.send(message.clone()).await {
            println!("Error al enviar el mensaje: {:?}", e);
            break;
        }
        println!("Mensaje enviado: {:?}", message);
        // Simulamos un intervalo de tiempo entre el envío de mensajes
        sleep(Duration::from_secs(2)).await;
    }

    // Cerramos el canal para indicar que no se enviarán más mensajes
    drop(tx);
}
