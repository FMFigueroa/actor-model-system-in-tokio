use tokio::{
    sync::mpsc,
    time::{sleep, Duration},
};

#[derive(Debug, Clone)]
pub enum Order {
    BUY,
    SELL,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub order: Order, // BUY or SELL
    pub symbol: String,
    pub amount: f32,
}

// Definimos un actor que recibe mensajes y los procesa
async fn actor(mut receiver: mpsc::Receiver<Message>) {
    while let Some(message) = receiver.recv().await {
        println!("Actor recibió el mensaje: {:?}", message);
        // Simulamos un procesamiento de mensaje
        sleep(Duration::from_secs(1)).await;
        println!("Actor ha terminado de procesar el mensaje");
    }
}

#[tokio::main]
async fn main() {
    // Creamos un canal para enviar mensajes entre actores
    let (tx, rx) = mpsc::channel::<Message>(1);

    // Creamos un actor que recibirá mensajes,
    // Le pasamos el recptor rx estableciendo un canal de comunicación.
    tokio::spawn(async move {
        actor(rx).await;
    });

    // Buffer de Mensajes para enviar al actor
    let buffer = [
        Message {
            order: Order::BUY,
            amount: 5.5,
            symbol: "BTC".to_owned(),
        },
        Message {
            order: Order::BUY,
            amount: 9.5,
            symbol: "ETH".to_owned(),
        },
        Message {
            order: Order::BUY,
            amount: 2.5,
            symbol: "PKT".to_owned(),
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
