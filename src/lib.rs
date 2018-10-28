extern crate neovim_lib as neovim;

use std::sync::mpsc;

pub fn start() {
    let mut session = neovim::Session::new_parent().unwrap();

    let (sender, receiver) = mpsc::channel();

    session.start_event_loop_handler(Handler::new(sender));

    for event in receiver {
        match event {
            Event::Quit => break,
        }
    }
}

enum Event {
    Quit,
}

struct Handler {
    sender: mpsc::Sender<Event>,
}

impl Handler {
    fn new(sender: mpsc::Sender<Event>) -> Handler {
        Handler { sender }
    }
}

impl neovim::Handler for Handler {
    fn handle_notify(&mut self, name: &str, _args: Vec<neovim::Value>) {
        match name {
            "quit" => self.sender.send(Event::Quit).unwrap(),
            _ => {}
        }
    }

    fn handle_request(
        &mut self,
        name: &str,
        args: Vec<neovim::Value>,
    ) -> Result<neovim::Value, neovim::Value> {
        match name {
            "nth" => {
                let n = args[0].as_u64().unwrap();
                Ok(neovim::Value::from(n * 2))
            }
            _ => Err(neovim::Value::from("unknown request")),
        }
    }
}
