# rust-irc-bot
A library for writing IRC bots in Rust

# Example
```rust
extern crate rustirc;
extern crate rustircbot;

use rustirc::client::Client;
use rustirc::info::IrcInfo;
use rustirc::message::Message;
use rustircbot::bot::Bot;
use rustircbot::command::Cmd;

struct HelloCallback;
impl Cmd for HelloCallback {
  fn on_cmd( &mut self, msg : Message, cnt : Client ) {
    match msg.nick( ) {
      Some( nick ) => cnt.send_msg( Message::privmsg( nick, "Hello!" ),
      None         => (),
    }
  }
}

fn main ( ) {
  let info = IrcInfo::new( "MyBot", "MyBot", "MyBot", vec![ "#irc" ] );
  let bot  = Bot::connect( "irc.freenode.net", 6667, "", info );
  
  bot.events.register_command( "[Hh]ello", Box::new( HelloCallback ) );
  bot.start( );
}
```
