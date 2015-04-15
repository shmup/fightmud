mod mud;

use mud::server::ServerFactory;

fn main() {
    let mut server = ServerFactory::new()
        .ip("122.122.122.122")
        .port(1337)
        .create();

    server.start();
}
