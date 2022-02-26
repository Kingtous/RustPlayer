mod app;
mod ui;
mod handler;
mod media;
mod config;

use ui::*;
use app::*;
use handler::*;
use media::*;


fn main() {
    let mut app = App::new().unwrap();
    match app.run() {
        Ok(_) => {},
        Err(e) => {
            println!("{:?}",e);
        },
    }
}