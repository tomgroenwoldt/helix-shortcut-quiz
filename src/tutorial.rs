use yew::{html, Component};

pub struct Tutorial;

impl Component for Tutorial {
    type Message = ();

    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <div class="modal">
                <div class="modal-left">{"Left."}</div>
                <div class="modal-middle">{"Middle."}</div>
                <div class="modal-right">{"Right."}</div>
            </div>
        }
    }
}
