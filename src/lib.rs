extern crate neovim_lib;

use neovim_lib::{Neovim, NeovimApi, Session};

pub fn start(port: &str) {
    let addr = format!("127.0.0.1:{}", port);
    let mut session = Session::new_tcp(&addr).expect("couldn't create session");
    session.start_event_loop();
    let mut nvim = Neovim::new(session);
}
