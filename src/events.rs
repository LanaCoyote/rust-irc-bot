use rustirc::client;
use rustirc::message;
use std::cell;

use command;

pub struct EventDispatcher <'ed> {
  cmd_callbacks : Vec < cell::RefCell < command::Command <'ed> > >,
}

impl <'ed> EventDispatcher <'ed> {
  pub fn new <'e> ( ) -> EventDispatcher <'e> {
    EventDispatcher {
      cmd_callbacks : Vec::new(),
    }
  }
  
  pub fn close ( &mut self ) {
    self.cmd_callbacks.clear( );
  }
  
  pub fn handle_msg ( &mut self, msg : message::Message, cnt : &mut client::Client ) {
    for cmcell in self.cmd_callbacks.as_mut_slice( ).iter( ) {
      let mut cmd = cmcell.borrow_mut( );
      if cmd.is_match( msg.clone( ) ) {
        cmd.call( msg.clone( ), cnt );
      }
    }
  }
  
  pub fn register_command ( &mut self, patt : &str, cb : Box < command::Cmd + 'ed > ) {
    let cmd = command::Command::new( patt, cb, command::Code::Privmsg );
    self.cmd_callbacks.push( cell::RefCell::new( cmd ) );
  }
  
  pub fn register_raw_command ( &mut self, patt : &str, cb : Box < command::Cmd + 'ed > ) {
    let cmd = command::Command::new( patt, cb, command::Code::Raw );
    self.cmd_callbacks.push( cell::RefCell::new( cmd ) );
  }
  
  pub fn register_code_command( &mut self, code : &str, patt : &str, cb : Box < command::Cmd + 'ed > ) {
    let cmd = command::Command::new( patt, cb, command::Code::Other( String::from_str( code ) ));
    self.cmd_callbacks.push( cell::RefCell::new( cmd ) );
  }
}