extern crate rustirc;

pub mod bot;
mod console;
mod events;

#[test]
fn it_works() {
  let inf = rustirc::info::IrcInfo::gen( "Chefbot", "Chefbot", "Chefbot", 
    vec! [ "#thefuture" ] );
  let clnt = rustirc::client::Client::connect( "irc.gamesurge.net", 6667, "", inf );
  
  let rx = clnt.start_thread( );
  loop {
    match rx.recv( ) {
      Ok ( msg ) => println! ( " > {}", msg.raw ),
      Err ( e )  => println! ( "!! receive error!!" ),
    }
  }
}
