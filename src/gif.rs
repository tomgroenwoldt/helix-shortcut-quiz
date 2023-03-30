use yew::{html, Component, Properties};

// TODO: Documentation. Also change documentation of GIF component.
#[derive(Clone, PartialEq)]
pub struct GifWrapper {
    pub path: String,
    pub solution: Vec<String>,
    pub description: String,
    pub prefix: Vec<String>,
}

impl From<(&str, &[&str], &str, &[&str])> for GifWrapper {
    fn from(value: (&str, &[&str], &str, &[&str])) -> Self {
        Self {
            path: value.0.to_owned(),
            solution: value.1.iter().map(|c| c.to_string()).collect::<Vec<_>>(),
            description: value.2.to_owned(),
            prefix: value.3.iter().map(|c| c.to_string()).collect::<Vec<_>>(),
        }
    }
}

/// Simple wrapper for an image tag which takes in a path to a GIF
/// and displays it in and endless loop.
pub struct Gif;

#[derive(Properties, Clone, PartialEq)]
pub struct GifProps {
    /// Path to location of GIF.
    pub path: String,
}

impl Component for Gif {
    type Message = ();

    type Properties = GifProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let path = ctx.props().path.clone();
        html! {
            <div class="gif">
                <img src={String::from("gifs/") + path.as_str()} />
            </div>
        }
    }
}
