use std::{io::Read, net::TcpListener};


fn dispatch(stream: &mut dyn Read) -> std::io::Result<()> {
    let mut request_type = vec![0u8, 2];
    stream.read_exact(&mut request_type)?;
    let req_type = String::from_utf8_lossy(&mut request_type);
    let mut stream = stream;
    match req_type.as_ref() {
        "BN" => {
            //broadcast_nick(&mut stream);
        }
        "BC" => {
            //broadcast_msg_to_client(&mut stream);
        }
        "BM" => {
            //broadcast_msg_to_all(&mut stream);
        }
        _ => {
            println!("El servidor no pudo atender la solicitud");
        }
    }
    Ok(())
}

fn main(){

    //let usuarios_conectados: Vec<client::Client>;

    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                std::thread::spawn(move|| {
                    // connection succeeded
                    dispatch(&mut stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}