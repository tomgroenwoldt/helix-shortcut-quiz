use gloo_storage::{LocalStorage, Storage};
use yew::{html, Callback, Component};

use crate::constants::COMMANDS;

/// Component which displays all possible user commands defined
/// in `src/constants.rs`.
pub struct Help {
    commands: Vec<(String, String)>,
}

impl Component for Help {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        let commands = COMMANDS
            .iter()
            .map(|(key, description)| (key.to_string(), description.to_string()))
            .collect::<Vec<_>>();
        Self { commands }
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        let commands = &self.commands;

        let playground_new = match LocalStorage::get("playground_new") {
            Ok(value) => value,
            Err(_) => {
                LocalStorage::set::<bool>("playground_new", true).unwrap();
                true
            }
        };
        let onclick = Callback::from(move |_| {
            LocalStorage::set::<bool>("playground_new", false).unwrap();
        });
        let playground_class = if playground_new {
            "playground-new"
        } else {
            "playground"
        };
        // Handle the last command separately.
        html! {
            <div class="sidebox">
                <dl class="help">
                    // Handle the rest of the commands.
                    {for commands.iter().map(|(key, description)|

                        html! {
                            <>
                                <dt class="command">{String::from(key) + ":"}</dt>
                                <dd>{description}</dd>
                                <br/>
                            </>
                        }
                    )}
                </dl>
                <a class="source"
                    href="https://github.com/tomgroenwoldt/helix-shortcut-quiz"
                    target="_blank">
                    {"View source"}
                </a>
                <a class={playground_class}
                    href="https://tomgroenwoldt.github.io/helix-editor-playground/"
                    {onclick}
                    target="_blank">
                    {"Check out the playground"}
                </a>
            </div>
        }
    }
}
