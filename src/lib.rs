extern crate stdweb;
#[macro_use]
extern crate yew;

use yew::prelude::*;

pub struct Model {
    exists: bool,
}

pub enum Msg {
    Click,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model { exists: false }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.exists = !self.exists;
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let beluga: &'static str = "🐳 OH 🐳 BABY 🐳 BELUGA!!! 🐳";
        html! {
            <div>
                <button onclick=|_| Msg::Click,>{ "Click" }</button>
                <p>
                { if self.exists { beluga } else { "" } }
                </p>
            </div>
        }
    }
}
