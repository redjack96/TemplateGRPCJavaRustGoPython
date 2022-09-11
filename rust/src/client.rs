use std::env::args;
// Vedi il server.rs per più spiegazioni sulla sintassi di Rust
// sintassi per gli use grpc
// nome_progetto::nome_file_proto::nome_servizio+"_client"::NomeServizioClient
use rust_grpc::helloworld::greeter_client::GreeterClient;
// nome_progetto::nome_file_proto::NomeMessage
use rust_grpc::helloworld::HelloRequest;

// per passare parametri usa:
// cargo run --bin client -- tuoiparametri
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Prendo il primo elemento dalla linea di comando. Per default uso "Giacomo"
    let user = args().nth(1)// Option<String>
        // Option è come Optional in Java. In Rust è una Enum con 2 varianti: Some(T) e None
        // Se il metodo restituisce qualcosa, si ottiene Some(T). In questo caso T = String
        // se non restituisce nulla, si usa None (che è una variante di Enum e ha dei metodi definiti).
        .unwrap_or("Giacomo".to_owned()); // to_owned trasforma &str (stack) in String (heap)
    // unwrap_or e' SEMPRE da preferire ad unwrap perché non va in "panic".
    // unwrap_or Restituisce T se Option è Some(T) altrimenti restituisce il valore T di default specificato

    // Crea un canale per la connessione al server
    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
        .connect()
        .await?;
    // Creo un gRPC client
    let mut client = GreeterClient::new(channel);
    // Creo una Request del crate tonic
    let request = tonic::Request::new(
        HelloRequest {
            name: user
        },
    );
    // Invio la richiesta e attendo la risposta:
    let response = client.say_hello(request)
        .await?
        .into_inner();
    println!("Response received: {:?}", response);
    Ok(())
}
