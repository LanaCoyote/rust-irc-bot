use rustirc::client;
use rustirc::info;
use rustirc::message;

use events;
use help;

pub struct Bot <'bl> {
  pub client : client::Client,
  pub events : events::EventDispatcher <'bl>,
  pub help   : help::HelpHandler <'bl>,
}

impl <'bl> Bot <'bl> {
  pub fn connect <'a> ( host : &str, port : u16, pass : &str, info : Box < info::IrcInfo > ) -> Bot <'a> {
    Bot {
      client : client::Client::connect( host, port, pass, info.clone( ) ),
      events : events::EventDispatcher::new( ),
      help   : help::HelpHandler::new( info.nick_name.as_slice( ), "", "" ),
    }
  }
  
  fn close ( &mut self ) {
    self.client.stop( );
    self.events.close( );
  }
  
  fn handle_dispatch ( &mut self, msg : message::Message ) {
    self.events.handle_msg( msg, &mut self.client );
  }
  
  pub fn init_help ( &mut self, cmd : &str, nice : &str ) {
    self.help.add_help( nice, "Provides help information for this bot." );
    self.events.register_command( cmd, Box::new( self.help.clone( ) ) );
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