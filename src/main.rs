mod mud;

use mud::{player, world};
use mud::server::ServerFactory;
use mud::player::Point;

fn main() {
    let player = player::Player::new();
    player.attack();

    let x = player.name.len();

    match x {
        e @ 1 ... 3 => println!("got a range of {}", e),
        e @ 3 ... 4 => println!("hi got a range of {}", e),
        _ => println!("more than that!"),
    }

    match player.position {
        Point { x: 1, y: y } => println!("no"),
        Point { x: 0, y: y } => println!("yep"),
        _ => println!("whatever"),
    }

    for l in player.name.chars() {
        println!("{}", l);
    }

    world::World::new();

    let server = ServerFactory::new()
        .ip("122.122.122.122")
        .port(1337)
        .start();

    println!("{}", server.port);
    println!("{}", server.ip);
}
