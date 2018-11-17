extern crate yew;
extern crate hello_yew;

use yew::prelude::*;
use hello_yew::Model;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
