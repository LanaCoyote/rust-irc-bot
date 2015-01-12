use rustirc::client;
use rustirc::info;
use rustirc::message;
use std::sync::mpsc;

use events;

struct Bot {
  client : client::Client,
  events : events::EventDispatcher,
}

impl Bot {
  pub fn connect ( host : &str, port : &str, pass : &str, info : info::IrcInfo ) -> Bot {
    Bot {
      client : client::Client::connect( host, port, pass, info ),
      events : events::EventDispatcher::new( ),
    }
  }
  
  fn close ( &self ) {
    //self.client.close( );
    self.events.close( );
  }
  
  fn handle_dispatch ( &self, msg : message::Message ) {
    self.events.handle_msg( msg );
  }
  
  pub fn start ( &self ) {
    let rx = self.client.start_thread( );
    for msg in rx.iter( ) {
      self.handle_dispatch( msg );
    }
    self.close( );
  }
}