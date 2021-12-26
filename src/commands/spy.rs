use clap::ArgMatches;

pub fn run(_info: &ArgMatches) {
    let ctx = zmq::Context::new();
    let server = ctx.socket(zmq::PAIR).unwrap();
    assert!(server.bind("tcp://*:5634").is_ok());

    println!("🕵️ bf-spy waiting for client connection");
    let mut msg = zmq::Message::new();
    loop {
        server.recv(&mut msg, 0).unwrap();
        println!("{}", msg.as_str().unwrap());
    }
}
