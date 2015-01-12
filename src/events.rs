

enum EventTrigger {
  ConCommand ( String ),
  MsgCode ( String ),
  MsgAny,
}

struct Event {
  trigger : EventTrigger,
  cb      : FnOnce,
}

struct EventDispatcher {
  events : Vec < Event >,
}

impl EventDispatcher {
  pub fn new ( ) -> EventDispatcher {
    EventDispatcher {
      events : Vec::new(),
    }
  }
}