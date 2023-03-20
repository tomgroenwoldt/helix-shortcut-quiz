use yew::{html, Component, Properties};

/// Simple wrapper for an image tag which takes in a path to a GIF
/// and displays it in and endless loop.
pub struct Gif;

#[derive(Properties, Clone, PartialEq)]
pub struct GifProps {
    pub path: String,
    #[prop_or(false)]
    pub web: bool,
}

impl Component for Gif {
    type Message = ();

    type Properties = GifProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let path = ctx.props().path.clone();
        match ctx.props().web {
            true => {
                html! {
                    <div class="gif">
                        <img src={path} />
                    </div>
                }
            }
            false => {
                html! {
                    <div class="gif">
                        <img src={String::from("gifs/") + path.as_str()} />
                    </div>
                }
            }
        }
    }
}
