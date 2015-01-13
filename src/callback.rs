use rustirc::client;
use rustirc::message;

use bot;

pub trait IrcCallback {
  fn on_any      ( &mut self, msg : message::Message, bot : &bot::Bot ) {}
  fn on_chanmsg  ( &mut self, msg : message::Message, bot : &bot::Bot ) {}
  fn on_invite   ( &mut self, msg : message::Message, bot : &bot::Bot ) {}
  fn on_join     ( &mut self, msg : message::Message, bot : &bot::Bot ) {}
  fn on_kick     ( &mut self, msg : message::Message, bot : &bot::Bot ) {}
  fn on_msg      ( &mut self, msg : message::Message, bot : &bot::Bot ) {}
  fn on_notice   ( &mut self, msg : message::Message, bot : &bot::Bot ) {}
  fn on_part     ( &mut self, msg : message::Message, bot : &bot::Bot ) {}
  fn on_privmsg  ( &mut self, msg : message::Message, bot : &bot::Bot ) {}
  fn on_welcome  ( &mut self, msg : message::Message, bot : &bot::Bot ) {}
}

pub trait CmdCallback {
  fn on_cmd ( &mut self, params : String, cnt : &mut client::Client ) {}
}