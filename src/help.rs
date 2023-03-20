use yew::{classes, html, Component, Properties};

use crate::constants::COMMANDS;

pub struct Help {
    commands: Vec<(String, String)>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct HelpProperties {
    #[prop_or(false)]
    pub end: bool,
}

impl Component for Help {
    type Message = ();

    type Properties = HelpProperties;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        let commands = COMMANDS
            .iter()
            .map(|(key, description)| (key.to_string(), description.to_string()))
            .collect::<Vec<_>>();
        Self { commands }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let commands = &self.commands;
        let (highlight_class, hide_class) = if ctx.props().end {
            ("highlight", "hide")
        } else {
            ("", "")
        };
        if let Some(reload_command) = commands.iter().last() {
            html! {
                <div class="sidebox">
                    <dl class="help">
                        {for commands.iter().take(commands.len() - 1).map(|(key, description)|

                            html! {
                                <>
                                    <dt class={classes!(hide_class, "command")}>{String::from(key) + ":"}</dt>
                                    <dd class={classes!(hide_class)}>{description}</dd>
                                    <br/>
                                </>
                            }
                        )}
                        <dt class={classes!(highlight_class, "command")}>{String::from(&reload_command.0) + ":"}</dt>
                        <dd class={classes!(highlight_class)}>{&reload_command.1}</dd>
                        <br/>
                    </dl>
                </div>
            }
        } else {
            html! {}
        }
    }
}
