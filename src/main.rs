extern crate yew;
extern crate kenya;

use yew::prelude::*;
use kenya::Model;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
