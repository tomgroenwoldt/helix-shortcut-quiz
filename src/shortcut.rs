use yew::{classes, html, Component, Html, Properties};

pub struct Shortcut;

#[derive(Properties, Clone, PartialEq)]
pub struct ShortcutProperties {
    pub keys: Vec<String>,
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
            <div class="shortcut">
                {for ctx.props()
                    .keys
                    .iter()
                    .enumerate()
                    .map(|(index, key)|
                        Self::render_key(key, ctx.props().guess.get(index), index != 0)
                    )
            }
            </div>
        }
    }
}

impl Shortcut {
    fn render_key(key: &str, guess: Option<&String>, render_plus: bool) -> Html {
        let class = if let Some(c) = guess {
            if c.eq(key) {
                "correct-key"
            } else {
                "wrong-key"
            }
        } else {
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
