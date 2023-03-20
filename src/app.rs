use gloo_events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::window;
use yew::prelude::*;

use crate::{
    constants::NORMAL_MODE_MOVEMENT, description::Description, gif::Gif, help::Help,
    shortcut::Shortcut,
};

pub struct App {
    gifs: Vec<(String, Vec<String>, String)>,
    current_gif: (String, Vec<String>, String),
    current_guess: Vec<String>,
    state: AppState,
}

#[derive(PartialEq)]
pub enum AppState {
    InProgress,
    End,
}

pub enum Msg {
    Key(String),
    Backspace,
    Escape,
    Next,
    Forward,
    Backward,
}

impl Component for App {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut gifs = vec![];

        NORMAL_MODE_MOVEMENT.iter().rev().for_each(|(k, v, d)| {
            gifs.push((
                String::from(*k),
                v.iter().map(|c| c.to_string()).collect::<Vec<_>>(),
                String::from(*d),
            ));
        });

        let current_gif = gifs.pop().expect("No GIFs found.");

        Self {
            gifs,
            current_gif,
            current_guess: vec![],
            state: AppState::InProgress,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        if self.state == AppState::End {
            return false;
        }
        let (_, solution, _) = &self.current_gif;
        match msg {
            Msg::Key(s) if self.current_guess.len() < solution.len() => {
                self.current_guess.push(s);
                true
            }
            Msg::Backspace => {
                self.current_guess.pop();
                true
            }
            Msg::Escape => {
                self.current_guess.clear();
                true
            }
            Msg::Next if self.current_guess.eq(solution) => {
                self.current_guess.clear();
                if let Some(new_gif) = self.gifs.pop() {
                    self.current_gif = new_gif;
                } else {
                    self.state = AppState::End;
                }
                true
            }
            Msg::Forward => {
                if let Some(next_gif) = self.gifs.pop() {
                    // Reinsert the skipped GIF.
                    self.gifs.insert(0, self.current_gif.clone());
                    self.current_guess = vec![];
                    self.current_gif = next_gif;
                    true
                } else {
                    // The last GIF should be displayed again, when trying to move forward.
                    false
                }
            }
            Msg::Backward => {
                // The last GIF should be displayed again, when trying to move backward.
                if self.gifs.len() < 2 {
                    return false;
                }
                let next_gif = self.gifs.remove(0);
                // Reinsert the skipped GIF.
                self.gifs.push(self.current_gif.clone());
                self.current_guess = vec![];
                self.current_gif = next_gif;
                true
            }
            _ => false,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let (path, solution, description) = &self.current_gif;
        match self.state {
            AppState::InProgress => {
                html! {
                    <div class="layout">
                        <div class="main">
                            <Description text={description.clone()}/>
                            <Gif path={path.clone()}/>
                            <Help />
                        </div>
                        <Shortcut
                            keys={solution.clone()}
                            guess={self.current_guess.clone()} />
                    </div>
                }
            }
            AppState::End => {
                html! {
                    <div class="layout">
                        <div class="main">
                            <Description text={"Congratulations! Thank you for playing!"}/>
                            <Gif path={"https://media.tenor.com/v1dPoOluqiwAAAAC/ferris-rust.gif"} web={true}/>
                            <Help end={true}/>
                        </div>
                    </div>
                }
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }

        // Add key event listener.
        let on_keypress = ctx.link().batch_callback(handle_keypress);
        let window = window().expect("No window? Where am I?");
        EventListener::new(&window, "keydown", move |e: &Event| {
            if let Ok(e) = e.clone().dyn_into::<KeyboardEvent>() {
                on_keypress.emit(e);
            }
        })
        .forget();
    }
}

/// Handles all key events triggered by the user.
fn handle_keypress(e: KeyboardEvent) -> Option<Msg> {
    let key = e.key();
    if key == "Backspace" {
        Some(Msg::Backspace)
    } else if key == "ArrowRight" {
        Some(Msg::Forward)
    } else if key == "ArrowLeft" {
        Some(Msg::Backward)
    } else if key == "Escape" {
        Some(Msg::Escape)
    } else if key == "Enter" {
        Some(Msg::Next)
    } else if key == "CapsLock" || key == "Shift" {
        None
    } else {
        Some(Msg::Key(key))
    }
}
