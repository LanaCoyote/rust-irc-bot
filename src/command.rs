use regex;
use rustirc::client;
use rustirc::message;

use bot;

pub trait Cmd {
  fn on_cmd ( &mut self, msg : message::Message, cnt : &mut client::Client ) { }
}

pub struct Command <'cl> {
  pattern : String,
  cb      : Box < Cmd + 'cl >,
  regex   : regex::Regex,
}

impl <'cl> Command <'cl> {
  pub fn new <'b> ( patt : &str, cb : Box < Cmd + 'b > ) -> Command <'b> {
    let re = match regex::Regex::new ( patt ) {
      Ok ( re ) => re,
      Err ( e ) => panic! ( "error creating command regex : {}", e.msg ),
    };
    Command {
      pattern : patt.to_string( ),
      cb      : cb,
      regex   : re,
    }
  }
  
  pub fn call ( &mut self, msg : message::Message, cnt : &mut client::Client ) {
    self.cb.on_cmd( msg, cnt )
  }
  
  pub fn is_match ( &self, msg : message::Message ) -> bool {
    self.regex.is_match( msg.trailing( ).expect( "No trailing on message" ) )
  }
}