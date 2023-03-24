use gloo_events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::window;
use yew::prelude::*;

use crate::{
    categories::{Categories, Category},
    constants::{EMPTY_PLACEHOLDER, END_PLACEHOLDER, NORMAL_MODE_CHANGES, NORMAL_MODE_MOVEMENT},
    description::Description,
    gif::Gif,
    help::Help,
    shortcut::Shortcut,
};

pub struct App {
    /// Stores all GIFs except the current one.
    gifs: Vec<(String, Vec<String>, String)>,
    current_gif: (String, Vec<String>, String),
    /// The input of user.
    current_guess: Vec<String>,
    state: AppState,
    categories: Vec<Category>,
}

#[derive(PartialEq)]
pub enum AppState {
    /// State when GIFs are still remaining.
    InProgress,
    /// State when no GIFs are found.
    Empty,
    /// State when all GIFs have been solved.
    End,
}

pub enum Msg {
    /// Adds supplied key as string to the user input.
    Key(String),
    /// Pops user input vector.
    Backspace,
    /// Clears user input.
    Escape,
    /// Moves to the next GIF by popping an element of the GIFs stored in `App`.
    Next,
    /// Skips current GIF and moves forward.
    Forward,
    /// Skips current GIF and moves backward.
    Backward,
    /// Toggles a category and reinitializes GIFs.
    ToggleCategory(Category),
}

impl Component for App {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut gifs = vec![];

        // Normal mode movement GIFs are set by default.
        gifs.append(&mut App::get_gifs(NORMAL_MODE_MOVEMENT));

        let current_gif = gifs.pop().expect("No GIFs found.");

        Self {
            gifs,
            current_gif,
            current_guess: vec![],
            state: AppState::InProgress,
            categories: vec![Category::NormalModeMovement],
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        // Don't update anything if we are finished with the game.
        if self.state == AppState::End {
            return false;
        }
        let (_, solution, _) = &self.current_gif;
        match msg {
            // We shouldn't allow a user input longer than the solution length.
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
                    // If this was the last GIF, end application.
                    self.state = AppState::End;
                    let (path, description) = END_PLACEHOLDER;
                    self.current_gif = (path.to_owned(), vec![], description.to_owned());
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
            Msg::ToggleCategory(category) => {
                let categories = category.toggle_category(&self.categories);
                self.set_gifs(&categories);
                self.categories = categories;
                if let Some(gif) = self.gifs.pop() {
                    // Display the first GIF found in the newly set GIFs.
                    self.current_gif = gif;
                    self.state = AppState::InProgress;
                } else {
                    // If no GIF is found within the selected categories,
                    // go into the empty state.
                    self.state = AppState::Empty;
                    let (path, description) = EMPTY_PLACEHOLDER;
                    self.current_gif = (path.to_owned(), vec![], description.to_owned());
                };
                true
            }
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (path, solution, description) = &self.current_gif;
        // Callback for category click.
        let on_click = ctx.link().callback(handle_category_click);
        let end = self.state.eq(&AppState::End);
        html! {
            <div class="layout">
                <Categories categories={self.categories.clone()} callback={on_click}/>
                <div class="main">
                    <Description text={description.clone()} />
                    <Gif path={path.clone()} />
                    <Shortcut
                        solution={solution.clone()}
                        guess={self.current_guess.clone()} />
                </div>
                <Help {end}/>
            </div>
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

impl App {
    /// Set GIFs with respect to passed in categories.
    fn set_gifs(&mut self, categories: &[Category]) {
        let mut gifs = vec![];
        categories.iter().for_each(|category| match category {
            Category::NormalModeMovement => gifs.append(&mut App::get_gifs(NORMAL_MODE_MOVEMENT)),
            Category::NormalModeChanges => gifs.append(&mut App::get_gifs(NORMAL_MODE_CHANGES)),
            Category::NormalModeSelect => {}
            Category::NormalModeSearch => {}
            Category::ViewMode => {}
            Category::GotoMode => {}
            Category::MatchMode => {}
            Category::WindowMode => {}
            Category::SpaceMode => {}
            Category::InsertMode => {}
            Category::SelectMode => {}
            Category::Picker => {}
            Category::Prompt => {}
        });
        self.gifs = gifs;
    }

    /// Converts static stored GIF collections into an application usable object.
    // This is a convenience feature as I don't want to deal with lifetimes.
    fn get_gifs(constant: &[(&str, &[&str], &str)]) -> Vec<(String, Vec<String>, String)> {
        let mut gifs = vec![];
        constant.iter().rev().for_each(|(k, v, d)| {
            gifs.push((
                String::from(*k),
                v.iter().map(|c| c.to_string()).collect::<Vec<_>>(),
                String::from(*d),
            ));
        });
        gifs
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

/// Handles click on a category. This function is passed down
/// to the `Categories` component.
fn handle_category_click(category: Category) -> Msg {
    Msg::ToggleCategory(category)
}
