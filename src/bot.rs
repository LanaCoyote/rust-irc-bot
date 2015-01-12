use rustirc::client;
use rustirc::info;
use rustirc::message;
use std::sync::mpsc;

use events;

struct Bot <'bl> {
  client : client::Client,
  events : events::EventDispatcher <'bl>,
}

impl <'bl> Bot <'bl> {
  pub fn connect <'a> ( host : &str, port : u16, pass : &str, info : info::IrcInfo, delim : char ) -> Bot <'a> {
    Bot {
      client : client::Client::connect( host, port, pass, info ),
      events : events::EventDispatcher::new( delim ),
    }
  }
  
  fn close ( &self ) {
    //self.client.close( );
    self.events.close( );
  }
  
  fn handle_dispatch ( &self, msg : message::Message ) {
    self.events.handle_msg( msg, self );
  }
  
  pub fn start ( &self ) {
    let rx = self.client.start_thread( );
    for msg in rx.iter( ) {
      self.handle_dispatch( msg );
    }
    self.close( );
  }
}