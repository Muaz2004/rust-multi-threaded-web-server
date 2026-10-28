use hello::thread_pool::ThreadPool;
use hello::handle_connection;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878")
        .expect("Could not bind to 127.0.0.1:7878");

    let pool = ThreadPool::new(4);

    println!("Server listening at http://127.0.0.1:7878");
    println!("Accepting two connections before graceful shutdown.");

    for stream in listener.incoming().take(2) {
        match stream {
            Ok(stream) => {
                pool.execute(|| handle_connection(stream));
            }
            Err(error) => {
                eprintln!("Failed to accept connection: {error}");
            }
        }
    }

    println!("Request limit reached. Waiting for workers to finish.");
}
