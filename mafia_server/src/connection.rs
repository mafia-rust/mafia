use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;

pub struct Connection{
    tcp_stream: WebSocketStream<TcpStream>,
    //lobby
    //player
}
impl Connection{
    pub fn new()->Self{
        Self { tcp_stream: () }
    }
}