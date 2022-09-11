use tonic::{transport::Server, Request, Response, Status};
use rust_grpc::hello::say_server::{Say, SayServer};
use rust_grpc::hello::{SayResponse, SayRequest};

// use helloworld::greeter_server::{Greeter, GreeterServer}; //sarà corretto?
// use helloworld::{HelloResponse, HelloRequest}; // sarà corretto?


// Differenza tra "mod" e "use":
// mod: permette di includere le funzioni definite nel file build.rs.
//      I file rust sono chiamati "modules", ma in generale in un file.rs ci può essere anche
//      più di un modulo (ad esempio un modulo con i test di unità nello stesso file in cui ci
//      sono le funzioni da testare).
// use: permette di importare alcuni o tutti (*) i simboli di una libreria.
//      Le librerie rust sono chiamate "crates".
//      ad esempio: tonic è un crate, hello è un modulo
// I :: separano i nomi di moduli e crate dai simboli definiti in essi (struct, enum, funzioni)

/// derive è una macro: durante la compilazione viene espansa per implementare Default
/// (trait/interfaccia) al posto nostro, così possiamo istanziare la struct con dei valori
/// di default in tutti i campi della struct con
/// MySay::default()
#[derive(Default)]
pub struct MySay {
}

const ADDRESS_STRING: &str = "[::1]:50051";

// implementa la 'rpc' del 'service' definito nel file .proto
// Say: Trait cioè un'interfaccia
// MySay: la struct sopra
#[tonic::async_trait] // Necessario perché Rust non supporta (per ora) i metodi asincroni per i trait
impl Say for MySay {
    // Result è una Enum Rust usata per gestire gli errori in modo funzionale (map, unwrap_or ...) SENZA I NULL.
    // Il primo generic è il tipo di dato restituito dal metodo se non ci sono errori
    // Il secondo generic è il tipo di dato dell'errore
    // Per istanziare un Result, è necessario usare una delle 2 varianti della Enum:
    //  Ok(T) se il metodo va a buon fine, con T tipo di dato risultante. In questo caso T = Response<SayResponse>
    //  Err(E) se il metodo fallisce, con E tipo di dato dell'errore. In questo caso E = Status
    /// Questo metodo implementa rpc definito in proto
    async fn send(&self, request: Request<SayRequest>) -> Result<Response<SayResponse>, Status> {
        println!("Received {}", request.get_ref().name);
        Ok(Response::new(SayResponse { // le struct si istanziano esattamente come in Go
            message: format!("hello {}", request.get_ref().name),
        })) // Se alla fine manca il ';' significa che stiamo restituendo l'Ok (Result)
        // In teoria questo METODO va sempre a buon fine, ma ricordiamo che è asincrono
    }
}

// Non è possibile definire async main senza #[tokio::main]
// perché ancora non esiste un runtime per la programmazione asincrona nella libreria standard,
// quindi per ora sfruttiamo il crate tokio (il più popolare).
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> { // Se va a buon fine non restituisce niente
    // Definisco l'indirizzo
    let addr = ADDRESS_STRING.parse().expect("Impossibile parsare l'indirizzo!");
    // Creo il servizio
    let say = MySay::default(); // istanzia la struct impostando TUTTI i valori in default!
    // aggiungo l'indirizzo al server
    println!("Server listening on {}", addr);
    Server::builder()
        .add_service(SayServer::new(say)) // Qua si possono aggiungere altri service se vuoi!!!
        .serve(addr) // inizia a servire a questo indirizzo!
        .await?; // Attende! E se ci sono errori, restituisce un Result Err con il messaggio di errore
    Ok(()) // Restituisce una tupla vuota dentro un Result Ok!
}
