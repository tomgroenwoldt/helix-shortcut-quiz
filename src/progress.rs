use yew::{html, Component, Properties};

use crate::categories::Category;

pub struct Progress;

#[derive(Properties, Clone, PartialEq)]
pub struct ProgressProps {
    pub played_gifs: Vec<String>,
    pub current_gif: String,
    pub category: Category,
}

impl Component for Progress {
    type Message = ();

    type Properties = ProgressProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    #[allow(clippy::needless_return)]
    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let gifs = ctx
            .props()
            .category
            .get_gifs()
            .iter()
            .map(|&gif| gif.0.to_string())
            .collect::<Vec<_>>();
        html! {
            <div class="progress">
                {for gifs.iter().map(|gif| {
                    if ctx.props().played_gifs.contains(gif) {
                        return html! {<div class="played"></div>};
                    } else if ctx.props().current_gif.eq(gif) {
                        return html! {<div class="active"></div>};
                    } else {
                        return html! {<div></div>};
                    }
                })}
            </div>
        }
    }
}
