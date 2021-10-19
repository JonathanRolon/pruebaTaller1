use std::{collections::HashMap, sync::{Arc, Mutex}};
use crate::conn::{self, Conn};

#[cfg(debug_assertions)]
macro_rules! debug {
  ($( $args:expr ), *) => { println!( $( $args ), * ); }
}

#[cfg(not(debug_assertions))]
macro_rules! debug {
  ($( $args:expr ),*) => {}
}

#[derive(Clone)]
pub struct Connections {
  pub counter: Arc<Mutex<u32>>,
  pub connections: Arc<Mutex<HashMap<u32, conn::Conn>>>
}

impl Connections {
  
  pub fn store(&self, conn: Conn) -> u32 {
    let mut counter = self.counter.lock().unwrap();
    *counter += 1;
    let id = *counter;
    self.connections.lock().unwrap().insert(id, conn);
    return id;
  } 

  fn remove(&self, id: u32) {
    self.connections.lock().unwrap().remove(&id);
  }

  pub fn broadcast(&self, buf: &[u8]) {
    /* Loop over all connections in map and write the given buffer */
    for (id, conn) in self.connections.lock().unwrap().iter() {
      match conn.write(&buf) {
        Ok(size) => { debug!("[{}] Wrote {} to connection...", id, size); },
        Err(e) => { debug!("[{}] Error writing to connection {}", id, e); },
      }
    }
  }

  pub fn new() -> Connections {
    Connections { 
      counter: Arc::new(Mutex::new(0)),
      connections: Arc::new(Mutex::new(HashMap::new())),
    }
  }
}