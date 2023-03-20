use yew::Component;

pub struct Shortcut;

#[derive(Properties, Clone, PartialEq)]
pub struct ShortcutProps {}

impl Component for Shortcut {
    type Message = ();

    type Properties;

    fn create(ctx: &yew::Context<Self>) -> Self {
        todo!()
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        todo!()
    }
}
