use yew::{html, Component, ComponentLink, Html, ShouldRender};

struct Model {
    link: ComponentLink<Self>,
}

enum Msg {
    DoIt,
}

impl Component for Model {
    // Some details omitted. Explore the examples to see more.

    type Message = Msg;
    // type Properties = ();

    // fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    //     Model { link }
    // }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DoIt => {
                // Update your model on events
                println!("Button pressed.");
                true
            }
        }
    }

    fn view(&self) -> Html {
        let onclick = self.link.callback(|_| Msg::DoIt);
        html! {
            // Render your model here
            <button onclick=onclick>{ Click me! }</button>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
