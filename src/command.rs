use regex;
use rustirc::client;
use rustirc::message;

use bot;

static WELCOME              : &str = "003";
static ERR_NOSUCHNICK       : &str = "401";
static ERR_NOSUCHSERVER     : &str = "402";
static ERR_NOSUCHCHANNEL    : &str = "403";
static ERR_CANNOTSENDTOCHAN : &str = "404";
static ERR_TOOMANYCHANNELS  : &str = "405";
static ERR_WASNOSUCHNICK    : &str = "406";
static ERR_TOOMANYTARGETS   : &str = "407";
static ERR_NOORIGIN         : &str = "409";
static ERR_NORECIPIENT      : &str = "411";
static ERR_NOTEXTTOSEND     : &str = "412";
static ERR_NOTOPLEVEL       : &str = "413";
static ERR_WILDTOPLEVEL     : &str = "414";

pub enum Code {
  Raw,
  Privmsg,
  Other ( _ ),
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
      Raw             => msg.raw,
      Privmsg         => {
        match msg.code.as_slice( ) {
          "PRIVMSG" | "NOTICE" => msg.trailing( ).expect_or( String::new( ) ),
          _                    => return false,
      },
      Other ( code )  => {
        match String::from_str( code ) {
          msg.code => msg.trailing( ).expect_or( String::new( ) ),
          _        => return false,
        }
      },
    };
    self.regex.is_match( cmpmsg )
  }
}