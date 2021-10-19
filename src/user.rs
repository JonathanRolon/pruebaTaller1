use std::{io::{Read, Write}, net::{IpAddr, Ipv4Addr}};

#[derive(Debug)]
pub struct User {
    pub nick_name: String,
    pub ip: IpAddr,
    pub port: u16
}

impl User {
    pub fn new(nick_name: String, ip: IpAddr, port: u16) -> Self {
        User { nick_name, ip, port}
    }

    pub fn write_to(&self, stream: &mut dyn Write) -> std::io::Result<()> {
        let size_nick_be = (self.nick_name.len() as u32).to_be_bytes();
        let size_ip_be = (self.ip.len() as u32).to_be_bytes();
        //paso el tamaño a leer del nick
        stream.write(&size_nick_be)?;
        //paso el nick
        stream.write(&self.nick_name.as_bytes())?;
        // paso el largo del ip
        stream.write(&size_ip_be)?;
        // paso el ip
        stream.write(&self.ip.as_bytes())?;

        Ok(())
    }

    fn read_field(&self, stream: &mut dyn Read) -> std::io::Result<String> {
        // creo buffer de 4 bytes 
        let mut size_buffer = [0u8; 4];
        // leo exactamente por el tamaño del buffer
        stream.read_exact(&mut size_buffer)?;
        // casteo de bytes a u32
        let field_len = u32::from_be_bytes(size_buffer);
        // creo buffer del tamano del string a leer
        let mut field_buffer = vec![0; field_len as usize];
        // leo exactamente por el tamaño del buffer
        stream.read_exact(&mut field_buffer)?;
        // Convierto de bytes a string.
        let mut field_str =
            std::str::from_utf8(&field_buffer).expect("Error al leer campo de usuario");
        Ok(field_str.to_owned().clone())    
        //my_field = &mut field_str.to_owned();

        //Ok(())
    }

    pub fn read_from(&self, stream: &mut dyn Read) -> std::io::Result<User> {
        
        let mut nick_name = self.read_field(stream)?;
        let mut ip = self.read_field(stream)?;

        let user = User { nick_name, ip };
        Ok(user)
    }
}
