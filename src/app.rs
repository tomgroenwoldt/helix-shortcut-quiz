use gloo_events::EventListener;
use gloo_storage::{LocalStorage, Storage};
use wasm_bindgen::JsCast;
use web_sys::window;
use yew::prelude::*;

use crate::{
    categories::{Categories, Category},
    constants::{EMPTY_PLACEHOLDER, END_PLACEHOLDER},
    description::Description,
    gif::{Gif, GifWrapper},
    help::Help,
    progress::Progress,
    shortcut::Shortcut,
};

pub struct App {
    /// Stores all GIFs except the current one.
    gifs: Vec<GifWrapper>,
    current_gif: GifWrapper,
    /// The input of user.
    current_guess: Vec<String>,
    state: AppState,
    active_category: Option<Category>,
    played_gifs: Vec<String>,
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
    /// Reset played GIFs in local storage for this category.
    Reset(Category),
}

impl Component for App {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let gifs = vec![];

        let current_gif = EMPTY_PLACEHOLDER.into();

        // Get played GIFs of old sessions from local storage.
        let played_gifs = match LocalStorage::get("played_gifs") {
            Ok(local_storage) => local_storage,
            Err(_) => {
                LocalStorage::set::<Vec<String>>("played_gifs", vec![]).unwrap();
                vec![]
            }
        };

        Self {
            gifs,
            current_gif,
            current_guess: vec![],
            state: AppState::InProgress,
            active_category: None,
            played_gifs,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        // // Don't update anything if we are finished with the game.
        // if self.state == AppState::End {
        //     return false;
        // }
        let solution = &self.current_gif.solution;
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
                self.played_gifs.push(self.current_gif.path.clone());
                LocalStorage::set("played_gifs", &self.played_gifs).unwrap();
                if let Some(new_gif) = self.gifs.pop() {
                    self.current_gif = new_gif;
                } else {
                    // If this was the last GIF, end application.
                    self.state = AppState::End;
                    self.current_gif = END_PLACEHOLDER.into();
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
                if category.is_disabled() {
                    return false;
                }
                self.active_category = match &self.active_category {
                    // A click on the current category deselects it and leads to the empty state.
                    Some(active_category) if active_category.eq(&category) => {
                        self.state = AppState::Empty;
                        self.gifs = vec![];
                        self.current_gif = EMPTY_PLACEHOLDER.into();
                        None
                    }
                    _ => {
                        self.set_gifs(Some(&category));
                        if let Some(gif) = self.gifs.pop() {
                            // Display the first GIF found in the newly set GIFs.
                            self.current_gif = gif;
                            self.state = AppState::InProgress;
                        } else {
                            // If no GIF is found within the selected categories,
                            // go into the empty state.
                            self.state = AppState::End;
                            self.current_gif = END_PLACEHOLDER.into();
                        };
                        Some(category)
                    }
                };
                true
            }
            Msg::Reset(category) => {
                let local_storage = LocalStorage::get::<Vec<String>>("played_gifs").unwrap();
                let updated_local_storage = local_storage
                    .into_iter()
                    .filter(|played_gif| !played_gif.contains(&category.base_path()))
                    .collect::<Vec<_>>();

                LocalStorage::set::<&Vec<String>>("played_gifs", &updated_local_storage).unwrap();
                self.played_gifs = updated_local_storage;
                self.set_gifs(Some(&category));
                if let Some(gif) = self.gifs.pop() {
                    // Display the first GIF found in the newly set GIFs.
                    self.current_gif = gif;
                    self.state = AppState::InProgress;
                }
                true
            }
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let GifWrapper {
            path,
            solution,
            description,
        } = &self.current_gif;
        // Callback for category click.
        let on_category_click = ctx.link().callback(handle_category_click);
        let on_reset_click = ctx.link().callback(handle_category_reset);

        let end = self.state.eq(&AppState::End);

        html! {
            <div class="layout">
                <Categories active_category={self.active_category.clone()} {on_category_click} on_reset_click={on_reset_click.clone()}/>
                <div class="main">
                    <div class="main-top-box">
                        <div class="title">
                            {"Helix Shortcut Quiz"}
                        </div>
                        <Description text={description.clone()} />
                    </div>
                    <Gif path={path.clone()} />
                    <div class="main-bottom-box">
                        if !end {
                            <Shortcut
                                solution={solution.clone()}
                                guess={self.current_guess.clone()}
                                category={self.active_category.clone()}/>
                        }
                        if self.active_category.is_some() {
                            <Progress
                                played_gifs={self.played_gifs.clone()}
                                current_gif={self.current_gif.path.clone()}
                                category={self.active_category.clone().unwrap()}
                                {on_reset_click} />
                        }
                    </div>
                </div>
                <Help />
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }

        // Add key event listener on first render.
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
    fn set_gifs(&mut self, category: Option<&Category>) {
        let mut gifs: Vec<GifWrapper> = vec![];
        if let Some(category) = category {
            let gif_store = category.get_gifs();
            gifs.append(&mut gif_store.iter().map(|&gif| gif.into()).collect());
        } else {
            // If no category is set, set gifs to the placeholder.
            let empty_gif = EMPTY_PLACEHOLDER.into();
            gifs.push(empty_gif);
        }
        let filtered_gifs = gifs
            .into_iter()
            .rev()
            .filter(|gif| !self.played_gifs.contains(&gif.path))
            .collect::<Vec<_>>();
        self.gifs = filtered_gifs;
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

/// Handles the click on the category reset button. This function
/// is passed down to the `Categories` and `Shortcut` component.
fn handle_category_reset(category: Category) -> Msg {
    Msg::Reset(category)
}
