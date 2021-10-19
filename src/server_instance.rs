use crate::conn::Conn;
use crate::connections::Connections;
use crate::user;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use std::{
    sync::{Arc, Mutex}
};

struct ServerInstance {
    usuarios_conectados: Connections,
}

impl ServerInstance {

    pub fn new(&self) -> ServerInstance {
        ServerInstance {
            usuarios_conectados: Connections::new()
        }
    }

    fn agregarUsuario(& self, socket: TcpStream){

        let conn = Conn {
            stream: Arc::new(Mutex::new(socket))
        };
        
        self.usuarios_conectados.store(conn.clone());
    }

    fn server_run(&'static mut self, address: &str) -> std::io::Result<()> {
        let listener = TcpListener::bind(address)?;
        listener.set_nonblocking(true);
        println!("Server listening on port: {:?}", address);
        for socket in listener.incoming() {
            match socket {
                Ok(socket) => {

                    self.agregarUsuario(socket);
                    
                    println!("New connection: {}", socket.peer_addr().unwrap());
                    std::thread::spawn(move || {
                        // connection succeeded
                        self.dispatch(&mut socket);
                    }).join();
                    
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
                Err(e) => {
                    println!("Error: {}", e);
                    /* connection failed */
                }
            }
        }

        // close the socket server
        drop(listener);
        Ok(())
    }
 
    fn dispatch(&'static self, stream: &mut TcpStream) -> std::io::Result<()> {
        let mut request_type = vec![0u8, 2];
        stream.read_exact(&mut request_type)?;
        let req_type = String::from_utf8_lossy(&mut request_type);
        //let mut stream = stream;
        match req_type.as_ref() {
            //Broadcast nick_name
            "BN" => {
                let nick = self.read_nick_name(stream)?;
                //enviar el tamaño
                self.usuarios_conectados.broadcast(&nick.len().to_be_bytes());
                //enviar el nick
                self.usuarios_conectados.broadcast(nick.as_bytes());
            }
            "BC" => {
                //broadcast_msg_to_client(&mut stream);
            }
            "BM" => {
                //broadcast_msg_to_all(&mut stream);
            }
            _ => {
                println!("El servidor no pudo atender la solicitud");
                //conn write 400
            }
        }
        Ok(())
    }

    fn read_nick_name(&self, stream: &mut dyn Read) -> std::io::Result<String> {
        //leer tamaño del nick
        let mut size_buffer = [0u8; 4];
        // leo exactamente por el tamaño del buffer
        stream.read_exact(&mut size_buffer).unwrap();
        // casteo de bytes a u32
        let field_len = u32::from_be_bytes(size_buffer);
        // creo buffer del tamanio del string a leer
        let mut field_buffer = vec![0; field_len as usize];
        // leo exactamente por el tamaño del buffer
        stream.read_exact(&mut field_buffer).unwrap();
        // Convierto de bytes a string.
        let mut field_str =
            std::str::from_utf8(&field_buffer).expect("SERVER_ERR: Error al leer nickname.");
        let nickname = field_str.to_owned();
    
        Ok(nickname.clone())
    }
}

/*
fn broadcast_msg_to_client(stream: &mut dyn Read) {}

fn broadcast_msg_to_all(stream: &mut dyn Read) {} */
