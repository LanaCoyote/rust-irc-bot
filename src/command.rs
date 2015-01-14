use regex;
use rustirc::client;
use rustirc::message;

use bot;

static WELCOME              : &'static str = "003";
static ERR_NOSUCHNICK       : &'static str = "401";
static ERR_NOSUCHSERVER     : &'static str = "402";
static ERR_NOSUCHCHANNEL    : &'static str = "403";
static ERR_CANNOTSENDTOCHAN : &'static str = "404";
static ERR_TOOMANYCHANNELS  : &'static str = "405";
static ERR_WASNOSUCHNICK    : &'static str = "406";
static ERR_TOOMANYTARGETS   : &'static str = "407";
static ERR_NOORIGIN         : &'static str = "409";
static ERR_NORECIPIENT      : &'static str = "411";
static ERR_NOTEXTTOSEND     : &'static str = "412";
static ERR_NOTOPLEVEL       : &'static str = "413";
static ERR_WILDTOPLEVEL     : &'static str = "414";

pub enum Code {
  Raw,
  Privmsg,
  Other ( String ),
}

pub trait Cmd {
  fn on_cmd ( &mut self, msg : message::Message, cnt : &mut client::Client ) { }
}

pub struct Command <'cl> {
  pattern : String,
  cb      : Box < Cmd + 'cl >,
  regex   : regex::Regex,
  code    : Code,
}

impl <'cl> Command <'cl> {
  pub fn new <'b> ( patt : &str, cb : Box < Cmd + 'b >, code : Code ) -> Command <'b> {
    let re = match regex::Regex::new ( patt ) {
      Ok ( re ) => re,
      Err ( e ) => panic! ( "error creating command regex : {}", e.msg ),
    };
    Command {
      pattern : patt.to_string( ),
      cb      : cb,
      regex   : re,
      code    : code,
    }
  }
  
  pub fn call ( &mut self, msg : message::Message, cnt : &mut client::Client ) {
    self.cb.on_cmd( msg, cnt )
  }
  
  pub fn is_match ( &self, msg : message::Message ) -> bool {
    let cmpmsg = match self.code {
      Code::Raw             => msg.raw.as_slice( ),
      Code::Privmsg         => {
        match msg.code.as_slice( ) {
          "PRIVMSG" | "NOTICE" => msg.trailing( ).unwrap_or( "" ),
          _                    => return false,
        }
      },
      Code::Other ( ref cd ) => {
        if msg.code == *cd {
          msg.trailing( ).unwrap_or( "" )
        } else {
          return false;
        }
      },
    };
    self.regex.is_match( cmpmsg )
  }
}