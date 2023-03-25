use yew::{html, Callback, Component, Properties};

use crate::categories::Category;

pub struct Progress;

#[derive(Properties, Clone, PartialEq)]
pub struct ProgressProps {
    pub played_gifs: Vec<String>,
    pub current_gif: String,
    pub category: Category,
    pub on_reset_click: Callback<Category>,
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

        let reset_button = if !gifs.contains(&ctx.props().current_gif) {
            let category_clone = ctx.props().category.clone();
            let callback_clone = ctx.props().on_reset_click.clone();
            let onclick = Callback::from(move |_| {
                let c = category_clone.clone();
                callback_clone.emit(c);
            });
            html! {
                <div class="reset" {onclick}>
                    <div>{"Reset this category"}</div>
                </div>
            }
        } else {
            html! {}
        };

        html! {
            <>
                {reset_button}
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
            </>
        }
    }
}
