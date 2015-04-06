mod mud;

use mud::{player, world};
use mud::server::ServerFactory;
use mud::player::Point;

fn main() {
    let server = ServerFactory::new()
        .ip("122.122.122.122")
        .port(1337)
        .create();

    server.start();
}
