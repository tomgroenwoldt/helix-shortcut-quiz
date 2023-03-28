use yew::{classes, html, Component, Html, Properties};

use crate::categories::Category;

/// Component which renders the input fields for the user.
/// The number of input fields directly depend on the length
/// of the solution length.
pub struct Shortcut {
    view_solution: bool,
}

pub enum Msg {
    ShowSolution,
}

#[derive(Properties, Clone, PartialEq)]
pub struct ShortcutProperties {
    /// The solution keys of the current GIF.
    pub solution: Vec<String>,
    /// The current user input.
    pub guess: Vec<String>,
    pub category: Option<Category>,
}

impl Component for Shortcut {
    type Message = Msg;

    type Properties = ShortcutProperties;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            view_solution: false,
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ShowSolution => self.view_solution = !self.view_solution,
        }
        true
    }

    fn changed(&mut self, _ctx: &yew::Context<Self>, _old_props: &Self::Properties) -> bool {
        // If the solution button was activated, and a user starts to type again, we disable the button.
        // On a skip we do this as well.
        self.view_solution = false;
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::ShowSolution);
        let class = if self.view_solution { "active" } else { "" };
        let prefix = if let Some(category) = ctx.props().category.clone() {
            category.prefix()
        } else {
            vec![]
        };
        html! {
            <div class="shortcut">
                <div class="input">
                    // Render an input field for every solution element.
                    {for prefix
                        .iter()
                        .map(Self::render_prefix)
                    }
                    {for ctx.props()
                        .solution
                        .iter()
                        .enumerate()
                        .map(|(index, key)|
                            // Set `render_plus` for every key, except the first.
                            Self::render_key(key, ctx.props().guess.get(index), index != 0, self.view_solution))
                    }
                </div>
                if !ctx.props().solution.is_empty() {
                    <div class="solution">
                        <div {class} {onclick}>{"Show solution"}</div>
                    </div>
                }
            </div>
        }
    }
}

impl Shortcut {
    /// Renders the key. Distinguishes between correct, wrong and empty user input.
    fn render_key(
        key: &str,
        guess: Option<&String>,
        render_plus: bool,
        view_solution: bool,
    ) -> Html {
        let class = if let Some(c) = guess {
            let mut class = String::from("");
            if c.eq(key) {
                class += " correct-key";
            } else {
                class += " wrong-key";
            }
            // Make the control key extra long to fit "Control" in it.
            if c.eq("Control") {
                class += " long";
            }
            class
        } else {
            // If no user input was given, render a neutral key element.
            String::from("")
        };
        html! {
            <>
                if render_plus {
                    <span class="plus">{"+"}</span>
                }
                if view_solution {
                    <div class="key solution-key">{key}</div>
                } else {
                    <div class={classes!("key", class)}>{guess}</div>
                }
            </>
        }
    }

    fn render_prefix(key: &String) -> Html {
        let class = if key.eq("Control") {
            "key prefix long"
        } else {
            "key prefix"
        };
        html! {
            <>
                <div {class}>{key}</div>
                <span class="plus">{"+"}</span>
            </>
        }
    }
}
