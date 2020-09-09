use std::thread;

use log::{info};

use crate::handler;

pub fn server_task(context: &zmq::Context, settings: &config::Config) {
    let frontend_addr = settings.get_str("Config.ZMQ.Frontend.Addr").unwrap();
    let frontend = context.socket(zmq::ROUTER).unwrap();
    frontend
        .bind(frontend_addr.as_str())
        .expect("zmq server failed binding frontend");

    let backend_addr = settings.get_str("Config.ZMQ.Backend.Addr").unwrap();
    let backend = context.socket(zmq::DEALER).unwrap();
    backend
        .bind(backend_addr.as_str())
        .expect("zmq server failed binding backend");

    let workers  = settings.get_int("Config.ZMQ.Backend.Workers").unwrap();
    for _ in 0..workers {
        let ctx = context.clone();
        let cfg = settings.clone();
        thread::spawn(move || worker_task(&ctx, &cfg));
    }

    match zmq::proxy(&frontend, &backend) {
        Err(_) => {
            info!("zmq server was stopped")
        },
        _ => {}
    }
}

pub fn worker_task(context: &zmq::Context, settings: &config::Config) {
    let backend_addr = settings.get_str("Config.ZMQ.Backend.Addr").unwrap();
    let worker = context.socket(zmq::REP).unwrap();
    worker
        .connect(backend_addr.as_str())
        .expect("zmq worker failed to connect to backend");

    loop {
        let identity = match worker.recv_string(0) {
            Ok(identity)  => identity,
            Err(err) => {
                if err == zmq::Error::ETERM {
                    break;
                }
                else {
                    panic!("Failed to receive zmq identity: {}", err);
                }
            },
        }.unwrap();

        let message = match worker.recv_string(0) {
            Ok(identity)  => identity,
            Err(err) => {
                if err == zmq::Error::ETERM {
                    break;
                }
                else {
                    panic!("Failed to receive zmq message: {}", err);
                }
            },
        }.unwrap();

        let response = handler::handle_request(&message);

        match worker.send(&identity, zmq::SNDMORE) {
            Err(err) => {
                if err == zmq::Error::ETERM {
                    break;
                }
                else {
                    panic!("Failed to send zmq identity: {}", err);
                }
            },
            _ => {}
        };

        match worker.send(&response, 0) {
            Err(err) => {
                if err == zmq::Error::ETERM {
                    break;
                }
                else {
                    panic!("Failed to send zmq message: {}", err);
                }
            },
            _ => {}
        }
    }
}
