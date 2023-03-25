use yew::{html, Component};

/// Component which displays the description of a GIF.
pub struct Description;

#[derive(yew::Properties, Clone, PartialEq)]
pub struct DescriptionProperties {
    #[prop_or_default]
    pub text: String,
}

impl Component for Description {
    type Message = ();

    type Properties = DescriptionProperties;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <div class="description">{&ctx.props().text}</div>
        }
    }
}
