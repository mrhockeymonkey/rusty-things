use std::collections::HashMap;
use std::sync::Mutex;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};

struct AppState
{
    app_name: String,
    tracker: Mutex<Box<dyn DataStore>>,
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

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct EntityId(u32);

#[post("/save")]
async fn save(data: web::Data<AppState>) -> HttpResponse {
    let mut tracker = data.tracker.lock().unwrap();

    tracker.store(EntityId(1234), String::from("Hello"));

    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // each thread with create its own instance of HttpServer so shared state needs to be instantiated outside of this factory
    let memory_store = MemoryStore::new();

    // Wrapping box in mutex requires casting
    // see https://users.rust-lang.org/t/how-to-use-dyn-trait-wrapped-with-mutex/66295
    let tracker = Mutex::new(Box::new(memory_store) as Box<dyn DataStore>);

    let app_data = web::Data::new(AppState{
        app_name: String::from("foo"),
        tracker,
    });


    HttpServer::new(|| {
        App::new()
            .app_data(app_data.clone())
            .service(save)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}