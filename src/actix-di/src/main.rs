use std::collections::HashMap;
use std::sync::Mutex;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};

#[cfg(feature = "nowhere")]
type ChosenSender = GenericService<NowhereSender>;

#[cfg(feature = "somewhere")]
type ChosenSender = GenericService<SomewhereSender>;

struct DynamicDispatchService
{
    store: Mutex<Box<dyn DataStore + Send>>,
}

struct GenericService<S>
where S : Sender
{
    sender: Mutex<S>
}

trait DataStore {
    fn store(&mut self, card_id: EntityId, value: String);
}

struct MemoryStore {
    store: HashMap<EntityId, String>
}

impl MemoryStore {
    fn new() -> Self {
        MemoryStore {
            store: HashMap::new()
        }
    }
}

impl DataStore for MemoryStore {
    fn store(&mut self, id: EntityId, value: String) {
        self.store.insert(id, value);
    }
}

trait Sender {
    fn send(&self, message: String);
}

struct NowhereSender;
struct SomewhereSender;

impl Sender for NowhereSender {
    fn send(&self, _: String) {
        println!("Send message nowhere")
    }
}

impl Sender for SomewhereSender {
    fn send(&self, _: String) {
        println!("Send message somewhere")
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct EntityId(u32);

#[get("/save")]
async fn save(data: web::Data<DynamicDispatchService>) -> HttpResponse {
    let mut tracker = data.store.lock().unwrap();

    println!("hello");
    tracker.store(EntityId(1234), String::from("Hello"));

    HttpResponse::Ok().finish()
}

#[get("/message")]
async fn message(data: web::Data<ChosenSender>) -> HttpResponse {
    let sender = data.sender.lock().unwrap();

    println!("world");
    sender.send(String::from("world"));

    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // each thread with create its own instance of HttpServer so shared state needs to be instantiated outside of this factory
    let memory_store = MemoryStore::new();

    // Wrapping box in mutex requires casting
    // see https://users.rust-lang.org/t/how-to-use-dyn-trait-wrapped-with-mutex/66295
    let store = Mutex::new(Box::new(memory_store) as Box<dyn DataStore + Send>);

    let dyn_service = web::Data::new(DynamicDispatchService {
        store,
    });

    #[cfg(feature = "nowhere")]
    let gen_service = web::Data::new(GenericService{
        sender: Mutex::new(NowhereSender),
    });

    #[cfg(feature = "somewhere")]
        let gen_service = web::Data::new(GenericService{
        sender: Mutex::new(SomewhereSender),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(dyn_service.clone())
            .app_data(gen_service.clone())
            .service(save)
            .service(message)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}