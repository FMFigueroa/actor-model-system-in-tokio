use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write};
use tokio::sync::{mpsc, mpsc::Sender, oneshot};

//==============================================================================
#[derive(Debug, Clone)]
pub enum Order {
    BUY,
    SELL,
}

#[derive(Debug)]
pub struct Message {
    pub order: Order,
    pub ticker: String,
    pub amount: f32,
    pub respond_to: oneshot::Sender<String>,
}

pub struct OrderBookActor {
    pub receiver: mpsc::Receiver<Message>,
    pub total_invested: f32,
    pub investment_cap: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderRecord {
    pub order: Order,
    pub ticker: String,
    pub amount: f32,
    pub response: String,
}

impl Serialize for OrderRecord {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("OrderRecord", 4)?;
        state.serialize_field("order", &self.order)?;
        state.serialize_field("ticker", &self.ticker)?;
        state.serialize_field("amount", &self.amount)?;
        state.serialize_field("response", &self.response)?;
        state.end()
    }
}

impl OrderBookActor {
    // Constructor
    fn new(receiver: mpsc::Receiver<Message>, investment_cap: f32) -> Self {
        return OrderBookActor {
            receiver,
            total_invested: 0.0,
            investment_cap,
        };
    }

    fn handle_message(&mut self, message: Message) {
        let msn = String::from("success");
        match message.order {
            Order::BUY => {
                if message.amount > self.investment_cap + self.total_invested {
                    println!("Rechazando compra de {}{}", message.ticker, message.amount);
                    let msn = String::from("fail");
                    let _ = message.respond_to.send(msn);
                } else {
                    self.total_invested = self.total_invested - message.amount;
                    println!(
                        "Procesando compra, cantidad: {}{:0.2}",
                        message.ticker, message.amount
                    );
                    //let msn = String::from("success");
                    //let _ = message.respond_to.send(msn);
                }
            }
            Order::SELL => {
                self.total_invested = self.total_invested + message.amount;
                // Simplemente imprime un mensaje para simular el procesamiento de una orden de venta
                println!(
                    "Procesando venta, cantidad: {}{:0.2}",
                    message.ticker, message.amount
                );
                //let msn = String::from("success");
                //let _ = message.respond_to.send(msn);
            }
        }

        // Almacenar la orden en un archivo JSON
        let order = OrderRecord {
            order: message.order,
            ticker: message.ticker,
            amount: message.amount,
            response: msn,
        };

        fn store_order(order: Message) -> std::io::Result<()> {
            let json = serde_json::to_string(&order)?;
            let mut file = File::create("db/orders.json")?;
            file.write_all(json.as_bytes())?;
            Ok(())
        }

        println!(
            " ->> Saldo disponible: {}{:0.2}",
            message.ticker,
            self.investment_cap + self.total_invested
        );

        // Enviar la respuesta al remitente
        let _ = message.respond_to.send(msn);
    }

    async fn run(mut self) {
        println!("actor is running");
        println!("investment capital: {:0.2}", self.investment_cap);
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg);
        }
    }
}

//==============================================================================

//==============================================================================
struct OrderActor {
    pub ticker: String,
    pub amount: f32,
    pub order: Order,
    pub sender: Sender<Message>,
}

impl OrderActor {
    fn new(amount: f32, ticker: String, order: Order, sender: Sender<Message>) -> Self {
        return OrderActor {
            ticker,
            amount,
            order,
            sender,
        };
    }

    async fn send(self) {
        let (send, recv) = oneshot::channel();
        let message = Message {
            order: self.order,
            amount: self.amount,
            ticker: self.ticker,
            respond_to: send,
        };

        let _ = self.sender.send(message).await;
        match recv.await {
            Err(e) => println!("{}", e),
            Ok(outcome) => println!("here is the outcome: {}", outcome),
        }
    }
}

//==============================================================================
#[tokio::main]
async fn main() {
    // init channel
    let (tx, rx) = mpsc::channel::<Message>(1);
    // other thread
    let tx_one = tx.clone();

    // other thread
    let tx_two = tx.clone();

    // tx_one thread 1
    tokio::spawn(async move {
        for _ in 0..3 {
            let buy_actor = OrderActor::new(5.0, "$".to_owned(), Order::BUY, tx.clone());
            buy_actor.send().await;
        }
        drop(tx);
    });

    // tx thread 2
    tokio::spawn(async move {
        for _ in 0..5 {
            let sell_actor = OrderActor::new(10.0, "$".to_owned(), Order::SELL, tx_one.clone());
            sell_actor.send().await;
        }
        drop(tx_one);
    });

    tokio::spawn(async move {
        for _ in 0..5 {
            let buy_actor = OrderActor::new(10.0, "$".to_owned(), Order::BUY, tx_two.clone());
            buy_actor.send().await;
        }
        drop(tx_two);
    });

    // init actor
    let actor = OrderBookActor::new(rx, 10.0);
    actor.run().await;
}
