use std::net::SocketAddr;
use std::sync::Arc;

use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio_tungstenite::{WebSocketStream, tungstenite::Message};

pub struct Connection{
    tx: UnboundedSender<Message>,
    //rx: UnboundedReceiver<Message>,

    address: SocketAddr,

    listeners: Arc<Mutex<Vec<fn(&Message)>>>
}
impl Connection{
    pub fn new(
        tx : UnboundedSender<Message>,
        mut rx : UnboundedReceiver<Message>,
        address: SocketAddr
    )->Self{

        tx.send(Message::Text("Connection Established!!".to_owned()));

        let listeners: Arc<Mutex<Vec<fn(&Message)>>> = Arc::new(Mutex::new(vec![
            |m: &Message|{
                println!("{}", m.to_string());
            }
        ]));

        let thread_listeners = listeners.clone();

        tokio::spawn(async move{
            while let Some(m) = rx.recv().await {
                for l in thread_listeners.lock().await.iter(){
                    l(&m);
                }
            }
        });

        Self { tx, address, listeners }
    }

    pub async fn add_listener(&mut self, listener: fn(&Message)){
        self.listeners.lock().await.push(listener);
    }
    pub async fn remove_all_listeners(&mut self){
        *self.listeners.lock().await = vec![];
    }
}
