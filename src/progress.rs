use std::{cell::RefCell, rc::Rc};

use yew::{html, Callback, Component, Properties};

use crate::{categories::Category, gif::GifWrapper};

pub struct Progress;

#[derive(Properties, Clone, PartialEq)]
pub struct ProgressProps {
    pub gifs: Vec<Rc<RefCell<GifWrapper>>>,
    pub current_gif: String,
    pub category: Category,
    pub end: bool,
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
        let gifs = ctx.props().gifs.clone();

        let reset_button = if ctx.props().end {
            let category_clone = ctx.props().category.clone();
            let callback_clone = ctx.props().on_reset_click.clone();
            let onclick = Callback::from(move |_| {
                let c = category_clone.clone();
                callback_clone.emit(c);
            });
            html! {
                <div class="reset">
                    <div {onclick}>{"Reset this category"}</div>
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
                        let gif = gif.borrow();
                        if gif.played {
                            return html! {<div class="played"></div>};
                        } else if gif.path.eq(&ctx.props().current_gif) {
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
