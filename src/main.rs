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
        if message.amount + self.total_invested >= self.investment_cap {
            println!(
                "rejecting purchase, total invested: {1}{0}",
                self.total_invested, message.ticker
            );
            let msn = String::from("fail");
            let _ = message.respond_to.send(msn);
        } else {
            self.total_invested += message.amount;
            println!(
                "processing purchase, total invested: {1}{0}",
                self.total_invested, message.ticker
            );

            // Respuesta al
            let msn = String::from("success");
            let _ = message.respond_to.send(msn);
        }
    }

    async fn run(mut self) {
        println!("actor is running");
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg);
        }
    }
}

//==============================================================================
struct BuyOrder {
    pub ticker: String,
    pub amount: f32,
    pub order: Order,
    pub sender: Sender<Message>,
}

impl BuyOrder {
    fn new(amount: f32, ticker: String, sender: Sender<Message>) -> Self {
        return BuyOrder {
            ticker,
            amount,
            order: Order::BUY,
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

    // tx_one thread 1
    tokio::spawn(async move {
        for _ in 1..4 {
            let buy_actor = BuyOrder::new(5.0, "$".to_owned(), tx.clone());
            buy_actor.send().await;
        }
        drop(tx);
    });

    // tx thread 2
    tokio::spawn(async move {
        for _ in 1..4 {
            let buy_actor = BuyOrder::new(5.0, "$".to_owned(), tx_one.clone());
            buy_actor.send().await;
        }
        drop(tx_one);
    });

    // init actor
    let actor = OrderBookActor::new(rx, 20.0);
    actor.run().await;
}
