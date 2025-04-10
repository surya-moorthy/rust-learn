use clap::Parser;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[derive(Parser,Clone)]
struct  Cli {
    #[arg(short,long)]
    url : String,
    #[arg(short,long)]
    msg : String
}


impl Cli {
    async fn handle_conn(&self) {
        let client_url = self.url.clone();
        
            let (ws_stream,_) = connect_async(client_url.clone()).await.expect("Failed to connect with the given client");
            println!("Connected to the websocket of the url : {} ", client_url);
    
            let msg = self.msg.clone();
    
            let (mut write ,mut  read) = ws_stream.split();
    
            (&mut write).send(msg.into()).await.expect("failed to write the msg");
            if let Some( message) = read.next().await {
                    let message = message.expect("Failed to get the message");
                    println!("received a message : {}",message);
            };
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    cli.handle_conn().await;
}
