use rustirc::client;
use rustirc::info;
use rustirc::message;

use command;
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
      help   : help::HelpHandler::new( "Unnamed", "Someone", "0.0" ),
    }
  }
  
  fn close ( &mut self ) {
    self.client.stop( );
    self.events.close( );
  }
  
  fn handle_dispatch ( &mut self, msg : message::Message ) {
    self.events.handle_msg( msg, &mut self.client );
  }
  
  pub fn add_help( &mut self, cmd : &str, help : &str ) {
    self.help.add_help( cmd, help );
  }
  
  pub fn init_help ( &mut self, cmd : &str, nice : &str ) {
    self.help.add_help( nice, "Provides help information for this bot." );
    self.events.register_command( cmd, Box::new( self.help.clone( ) ) );
  }
  
  pub fn add_cmd ( &mut self, patt : &str, cb : Box < command::Cmd + 'bl > ) {
    self.events.register_command( patt, cb );
  }
  
  pub fn add_raw_cmd ( &mut self, patt : &str, cb : Box < command::Cmd + 'bl > ) {
    self.events.register_raw_command( patt, cb );
  }
  
  pub fn add_code_cmd ( &mut self, code : &str, patt : &str, cb : Box < command::Cmd + 'bl > ) {
    self.events.register_code_command( code, patt, cb );
  }
  
  pub fn set_help_info ( &mut self, name : &str, author : &str, version : &str ) {
    self.help.info.name     = name.to_string( );
    self.help.info.author   = author.to_string( );
    self.help.info.version  = version.to_string( );
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