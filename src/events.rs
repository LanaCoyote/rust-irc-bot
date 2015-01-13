use regex;
use rustirc::client;
use rustirc::message;
use std::cell;

use bot;
use callback;
use command;

pub struct EventDispatcher <'ed> {
  msg_callbacks : Vec < &'ed (callback::IrcCallback + 'ed) >,
  cmd_callbacks : Vec < cell::RefCell < command::Command <'ed> > >,
}

impl <'ed> EventDispatcher <'ed> {
  pub fn new <'e> ( ) -> EventDispatcher <'e> {
    EventDispatcher {
      msg_callbacks : Vec::new(),
      cmd_callbacks : Vec::new(),
    }
  }
  
  pub fn close ( &mut self ) {
    self.msg_callbacks.clear( );
    self.cmd_callbacks.clear( );
  }
  
  pub fn handle_msg ( &mut self, msg : message::Message, cnt : &mut client::Client ) {
    // OLD CALLBACK CODE, UNCOMMENT IF I REIMPLEMENT THEM
    // for cb in self.msg_callbacks.iter( ) {
      // cb.on_any( msg, bot );
      // match msg.code.as_slice( ) {
        // "INVITE"  => cb.on_invite( msg, bot ),
        // "JOIN"    => cb.on_join( msg, bot ),
        // "KICK"    => cb.on_kick( msg, bot ),
        // "PRIVMSG" => {
          // cb.on_msg( msg, bot );
          // match msg.nick( ) {
            // Some( s ) => {
              // if s.as_slice( ).starts_with( "#" ) {
                // cb.on_chanmsg( msg, bot );
              // } else {
                // cb.on_privmsg( msg, bot );
              // }
            // },
            // None      => (),
          // }
        // },
        // "NOTICE"  => cb.on_notice( msg, bot ),
        // "PART"    => cb.on_part( msg, bot ),
        // "003"     => cb.on_welcome( msg, bot ),
        // _         => (),
      // }
    // }
    
    if msg.code.as_slice( ) == "PRIVMSG" {
      for cmcell in self.cmd_callbacks.as_mut_slice( ).iter( ) {
        let mut cmd = cmcell.borrow_mut( );
        if cmd.is_match( msg.clone( ) ) {
          cmd.call( msg.clone( ), cnt );
        }
      }
    }
  }
  
  pub fn register_command ( &mut self, patt : &str, cb : Box < command::Cmd + 'ed > ) {
    let cmd = command::Command::new( patt, cb );
    self.cmd_callbacks.push( cell::RefCell::new( cmd ) );
  }
}