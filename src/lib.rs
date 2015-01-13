extern crate regex;
extern crate rustirc;

pub mod bot;
mod callback;
pub mod command;
mod console;
pub mod events;

// #[test]
// fn it_works() {
  // let inf = rustirc::info::IrcInfo::gen( "Chefbot", "Chefbot", "Chefbot",
    // vec! [ "#thefuture" ] );
  // let mut chef = bot::Bot::connect( "irc.gamesurge.net", 6667, "", inf );
  // chef.events.register_command( "[Hh]ello", Box::new( TestCallback ) );
  // chef.events.register_command( "^!join", Box::new( JoinCallback ) );
  // chef.start( );
// }

// struct TestCallback;
// impl command::Cmd for TestCallback {
  // fn on_cmd ( &mut self, msg : rustirc::message::Message, cnt : &mut rustirc::client::Client ) {
    // let outmsg = rustirc::message::Message::new(
      // rustirc::message::Source::None,
      // "PRIVMSG",
      // "#thefuture :Hello!" );
    // cnt.send_msg( outmsg );
  // }
// }

// struct JoinCallback;
// impl command::Cmd for JoinCallback {
  // fn on_cmd ( &mut self, msg : rustirc::message::Message, cnt : &mut rustirc::client::Client ) {
    // println! ( "{}", msg.raw );
    // if msg.is_public( ) { return };

    // let re      = regex::Regex::new( r"^!join (\S+)" ).unwrap( );
    // let outmsg  = match re.captures( msg.trailing( ).expect( "" ).as_slice( ) ) {
      // Some( cap ) => { format! ( "JOIN {}", cap.at( 1 ).unwrap( ) ) },
      // None        => { format! ( "PRIVMSG {} :Please specify a channel!", msg.nick( ).expect( "" ) ) },
    // };
    // cnt.send_str( outmsg.as_slice( ) );
  // }
// }