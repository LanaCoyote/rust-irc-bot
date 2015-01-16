use rustirc::client;
use rustirc::info;
use rustirc::message;

use events;

pub struct Bot <'bl> {
  pub client : client::Client,
  pub events : events::EventDispatcher <'bl>,
}

impl <'bl> Bot <'bl> {
  pub fn connect <'a> ( host : &str, port : u16, pass : &str, info : info::IrcInfo ) -> Bot <'a> {
    Bot {
      client : client::Client::connect( host, port, pass, Box::new( info ) ),
      events : events::EventDispatcher::new( ),
    }
  }
  
  fn close ( &mut self ) {
    self.client.stop( );
    self.events.close( );
  }
  
  fn handle_dispatch ( &mut self, msg : message::Message ) {
    self.events.handle_msg( msg, &mut self.client );
  }
  
  pub fn start ( mut self ) {
    let (rx,cnt)    = self.client.start_thread( );
    self.client     = cnt;
    for msg in rx.iter( ) {
      self.handle_dispatch( msg );
    }
    self.close( );
  }
}