use rustirc::client::Client;
use rustirc::message::Message;
use std::collections;
use std::io::timer;
use std::time::duration::Duration;

use command;

struct HelpInfo {
  pub name    : String,
  pub author  : String,
  pub version : String,
}

impl Clone for HelpInfo {
  fn clone ( &self ) -> HelpInfo {
    HelpInfo {
      name    : self.name.clone( ),
      author  : self.author.clone( ),
      version : self.version.clone( ),
    }
  }
}

pub struct HelpHandler <'hl> {
  info        : HelpInfo,
  helpmap     : collections::HashMap < String, Vec < String > >,
}

impl <'hl> HelpHandler <'hl> {
  pub fn new <'a> ( name : &str, author : &str, version : &str ) -> HelpHandler <'a> {
    HelpHandler {
      info      : HelpInfo {
        name    : name.to_string( ),
        author  : author.to_string( ),
        version : version.to_string( ),
      },
      helpmap   : collections::HashMap::new( ),
    }
  }

  pub fn add_help( &mut self, cmd : &str, help : &str ) {
    let mut helpvec : Vec < String > = Vec::new( );
    for line in help.lines( ) {
      helpvec.push( line.to_string( ) );
    }
    self.helpmap.insert( cmd.to_string( ), helpvec );
  }
}

impl <'hl> Clone for HelpHandler <'hl> {
  fn clone <'a> ( &self ) -> HelpHandler <'a> {
    HelpHandler {
      info    : self.info.clone( ),
      helpmap : self.helpmap.clone( ),
    }
  }
}

impl <'hl> command::Cmd for HelpHandler <'hl> {
  fn on_cmd( &mut self, msg : Message, cnt : &mut Client ) {
    // get the nick of whoever asked for help
    let targ = match msg.nick( ) {
      Some( nick ) => nick,
      None         => { return; },
    };
    
    // extract our parameters
    let params : Vec < &str > = msg.trailing( ).unwrap( ).words( ).collect( );
    
    // no command specified, print general help
    if params.len( ) == 1 {
      // give us a nice little header that shows the bot info
      let headline = format! ( "{} by {} - Version {}", 
        self.info.name, self.info.author, self.info.version );
      cnt.message( targ.as_slice( ), headline.as_slice( ) );
      
      // print the basic help for each command
      for (k,v) in self.helpmap.iter( ) {
        sleep( 100 );
        let helpline = format! ( "{} - {}", k, v[0] );
        cnt.message( targ.as_slice( ), helpline.as_slice( ) );
      }
      
    // commands given, print extended help
    } else {
      for i in range( 1, params.len( ) ) {
        // check if the command exists
        match self.helpmap.get( &String::from_str( params[i] ) ) {
          Some( help ) => {
            // print the command name
            let headline = format! ( " - {}", params[i] );
            cnt.message( targ.as_slice( ), headline.as_slice( ) );
            
            // print extended help
            for line in help.iter( ) {
              sleep( 100 );
              cnt.message( targ.as_slice( ), line.as_slice( ) );
            }
          },
          
          // command not found
          None         => {
            let outline = format! ( "Command '{}' not found", params[i] );
            cnt.message( targ.as_slice( ), outline.as_slice( ) );
          },
        }
        
        // sleep between commands
        sleep( 100 );
      }
    }
  }
}

fn sleep( ms : usize ) {
  return
}