use rustirc::message;

use bot;
use callback;

struct Command <'cl> {
  cmd : String,
  cb  : &'cl (callback::CmdCallback + 'cl),
}

struct EventDispatcher <'ed> {
  cmd_delimiter : char,
  msg_callbacks : Vec < &'ed (callback::IrcCallback + 'ed) >,
  cmd_callbacks : Vec < Command <'ed> >,
}

impl <'a> EventDispatcher <'a> {
  pub fn new <'a> ( delim : char ) -> EventDispatcher <'a> {
    EventDispatcher {
      cmd_delimiter : delim,
      msg_callbacks : Vec::new(),
      cmd_callbacks : Vec::new(),
    }
  }
  
  pub fn close ( &mut self ) {
    self.msg_callbacks.clear( );
    self.cmd_callbacks.clear( );
  }
  
  pub fn handle_msg ( &self, msg : message::Message, bot : &bot::Bot ) {
    for cb in self.msg_callbacks.iter( ) {
      cb.on_any( msg, bot );
      match msg.code.as_slice( ) {
        "INVITE"  => cb.on_invite( msg, bot ),
        "JOIN"    => cb.on_join( msg, bot ),
        "KICK"    => cb.on_kick( msg, bot ),
        "PRIVMSG" => {
          cb.on_msg( msg, bot );
          match msg.nick( ) {
            Some( s ) => {
              if s.as_slice( ).starts_with( "#" ) {
                cb.on_chanmsg( msg, bot );
              } else {
                cb.on_privmsg( msg, bot );
              }
            },
            None      => (),
          }
        },
        "NOTICE"  => cb.on_notice( msg, bot ),
        "PART"    => cb.on_part( msg, bot ),
        "003"     => cb.on_welcome( msg, bot ),
      }
    }
    
    if msg.code.as_slice( ) == "PRIVMSG" {
      match msg.trailing( ) {
        Some ( t )  => {
          if t.char_at( 0 ) == self.cmd_delimiter {
            let tcmd = match t.split( " " ).next( ) {
              Some ( s ) => s.slice_from( 1 ),
              None       => "",
            };
            for cb in self.cmd_callbacks.iter( ) {
              if cb.cmd == tcmd {
                cb.cb.on_cmd( String::from_str( t ), bot );
              }
            }
          }
        },
        None        => (),
      }
    }
  }
  
  pub fn register_callback <'a> ( &mut self, cb : &'a (callback::IrcCallback + 'a) ) {
    self.msg_callbacks.push( cb );
  }
  
  pub fn register_command <'a> ( &mut self, cmd : String, cb : &'a (callback::CmdCallback + 'a) ) {
    self.cmd_callbacks.push( Command { cmd : cmd, cb : cb } );
  }
}