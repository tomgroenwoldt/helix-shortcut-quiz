use yew::{classes, html, Component, Html, Properties};

/// Component which renders the input fields for the user.
/// The number of input fields directly depend on the length
/// of the solution length.
pub struct Shortcut;

#[derive(Properties, Clone, PartialEq)]
pub struct ShortcutProperties {
    /// The solution keys of the current GIF.
    pub solution: Vec<String>,
    /// The current user input.
    pub guess: Vec<String>,
}

impl Component for Shortcut {
    type Message = ();

    type Properties = ShortcutProperties;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        html! {
            <div class="main-bottom-box">
                <div class="shortcut">
                    // Render an input field for every solution element.
                    {for ctx.props()
                        .solution
                        .iter()
                        .enumerate()
                        .map(|(index, key)|
                            // Set `render_plus` for every key, except the first.
                            Self::render_key(key, ctx.props().guess.get(index), index != 0)
                        )
                }
                </div>
            </div>
        }
    }
}

impl Shortcut {
    /// Renders the key. Distinguishes between correct, wrong and empty user input.
    fn render_key(key: &str, guess: Option<&String>, render_plus: bool) -> Html {
        let class = if let Some(c) = guess {
            if c.eq(key) {
                "correct-key"
            } else {
                "wrong-key"
            }
        } else {
            // If no user input was given, render a neutral key element.
            ""
        };
        html! {
            <>
                if render_plus {
                    <span class="plus">{"+"}</span>
                }
                <div class={classes!("key", class)}>
                    {guess}
                </div>
            </>
        }
    }
}
