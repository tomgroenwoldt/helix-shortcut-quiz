use gloo_events::EventListener;
use gloo_storage::{LocalStorage, Storage};
use rand::{seq::SliceRandom, thread_rng};
use std::{cell::RefCell, rc::Rc};
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
    /// All GIFs in current game.
    gifs: Vec<Rc<RefCell<GifWrapper>>>,
    /// A current position of current GIF.
    current_position: usize,
    /// A reference to the current GIF.
    current_gif: Rc<RefCell<GifWrapper>>,
    /// The input of the user.
    current_guess: Vec<String>,
    state: AppState,
    active_category: Option<Category>,
    /// Played GIFs fetched from clients local storage.
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

#[derive(PartialEq)]
pub enum ForwardType {
    Skip,
    Success,
}

pub enum Msg {
    /// Adds supplied key as string to the user input.
    Key(String),
    /// Pops user input vector.
    Backspace,
    /// Clears user input.
    Escape,
    /// Moves forward, skips or saves progress respectively.
    Forward(ForwardType),
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

        let current_gif = Rc::new(RefCell::new(EMPTY_PLACEHOLDER.into()));

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
            current_position: 0,
            current_guess: vec![],
            state: AppState::InProgress,
            active_category: None,
            played_gifs,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let solution = &self.current_gif.borrow().solution.clone();
        match (msg, &self.state) {
            // We shouldn't allow a user input longer than the solution length.
            (Msg::Key(s), AppState::InProgress) if self.current_guess.len() < solution.len() => {
                self.current_guess.push(s);
                true
            }
            (Msg::Backspace, AppState::InProgress) => {
                self.current_guess.pop();
                true
            }
            (Msg::Escape, AppState::InProgress) => {
                self.current_guess.clear();
                true
            }
            (Msg::Forward(value), AppState::InProgress) => {
                if value.eq(&ForwardType::Success) {
                    if !self.current_guess.eq(solution) {
                        return false;
                    }
                    self.current_guess.clear();
                    self.current_gif.borrow_mut().played = true;

                    // Add played GIF to local storage.
                    self.played_gifs
                        .push(self.current_gif.borrow().path.clone());
                    LocalStorage::set("played_gifs", &self.played_gifs).unwrap();
                }
                // Find the first unplayed GIFs which comes after the
                // current position. This wraps around via the `cycle()` call.
                if self
                    .gifs
                    .iter()
                    .filter(|gif| !gif.borrow().played)
                    .count()
                    .eq(&0)
                {
                    // If this was the last GIF, end application.
                    self.state = AppState::End;
                    self.current_gif = Rc::new(RefCell::new(END_PLACEHOLDER.into()));
                    return true;
                }
                if let Some((new_position, next_gif)) = self
                    .gifs
                    .iter()
                    .enumerate()
                    .cycle()
                    .skip(self.current_position + 1)
                    .find(|(_, gif)| !gif.borrow().played)
                {
                    self.current_gif = Rc::clone(next_gif);
                    self.current_position = new_position;
                    return true;
                }
                false
            }
            (Msg::Backward, AppState::InProgress) => {
                // Find the first unplayed GIFs which comes in front of the
                // current position. This wraps around via the `cycle()` call.
                if let Some((new_position, previous_gif)) = self
                    .gifs
                    .iter()
                    .enumerate()
                    .rev()
                    .cycle()
                    .skip(self.gifs.len() - self.current_position)
                    .find(|(_, gif)| !gif.borrow().played)
                {
                    self.current_gif = Rc::clone(previous_gif);
                    self.current_position = new_position;
                    return true;
                }
                false
            }
            (Msg::ToggleCategory(category), _) => {
                if category.is_disabled() {
                    return false;
                }
                self.active_category = match &self.active_category {
                    // A click on the current category deselects it and leads to the empty state.
                    Some(active_category) if active_category.eq(&category) => {
                        self.state = AppState::Empty;
                        self.gifs = vec![];
                        self.current_gif.replace(EMPTY_PLACEHOLDER.into());
                        None
                    }
                    // Otherwise set GIFs for new category.
                    _ => {
                        self.set_gifs(Some(&category));
                        if let Some((new_position, new_gif)) = self
                            .gifs
                            .iter()
                            .enumerate()
                            .find(|(_, gif)| !gif.borrow().played)
                        {
                            self.state = AppState::InProgress;
                            self.current_gif = Rc::clone(new_gif);
                            self.current_position = new_position;
                        } else {
                            // No GIFs found within the selected category.
                            // This means all GIFs have been played with success.
                            // Go into the empty state.
                            self.state = AppState::End;
                            self.current_gif.replace(END_PLACEHOLDER.into());
                        };
                        Some(category)
                    }
                };
                true
            }
            (Msg::Reset(category), _) => {
                // Empties the local storage for specific category.
                let local_storage = LocalStorage::get::<Vec<String>>("played_gifs").unwrap();
                let updated_local_storage = local_storage
                    .into_iter()
                    .filter(|played_gif| !played_gif.contains(&category.base_path()))
                    .collect::<Vec<_>>();

                LocalStorage::set::<&Vec<String>>("played_gifs", &updated_local_storage).unwrap();
                self.played_gifs = updated_local_storage;
                self.set_gifs(Some(&category));
                if let Some(gif) = self.gifs.first() {
                    // Create reference to the first GIF found in the newly set GIFs.
                    self.current_gif = Rc::clone(gif);
                    self.state = AppState::InProgress;
                }
                true
            }
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // Callback for category click.
        let on_category_click = ctx.link().callback(handle_category_click);
        let on_reset_click = ctx.link().callback(handle_category_reset);

        let end = self.state.eq(&AppState::End);
        let current_gif = self.current_gif.borrow();

        html! {
            <div class="layout">
                <Categories active_category={self.active_category.clone()} {on_category_click} on_reset_click={on_reset_click.clone()}/>
                <div class="main">
                    <div class="main-top-box">
                        <div class="title">
                            {"Helix Shortcut Quiz"}
                        </div>
                        <Description text={current_gif.description.clone()} />
                    </div>
                    <Gif path={current_gif.path.clone()} />
                    <div class="main-bottom-box">
                        if !end {
                            <Shortcut
                                solution={current_gif.solution.clone()}
                                guess={self.current_guess.clone()}
                                category={self.active_category.clone()}
                                prefix={current_gif.prefix.clone()} />
                        }
                        if self.active_category.is_some() {
                            <Progress
                                gifs={self.gifs.clone()}
                                current_gif={current_gif.path.clone()}
                                category={self.active_category.clone().unwrap()}
                                {end}
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
    /// Set GIFs with respect to passed in category.
    fn set_gifs(&mut self, category: Option<&Category>) {
        let mut gifs: Vec<GifWrapper> = vec![];
        if let Some(category) = category {
            let mut gif_store = category.get_gifs();
            gifs.append(&mut gif_store);

            let filtered_gifs = match category {
                // The random category takes 10 unplayed GIFs at random.
                Category::Random => {
                    let mut gifs = gifs
                        .into_iter()
                        .filter(|gif| !self.played_gifs.contains(&gif.path))
                        .map(RefCell::new)
                        .map(Rc::new)
                        .collect::<Vec<_>>();
                    gifs.shuffle(&mut thread_rng());
                    gifs.into_iter().take(10).collect::<Vec<_>>()
                }
                // Otherwise take plain category GIFs.
                _ => {
                    let mut gifs = gifs
                        .into_iter()
                        .map(|mut gif| {
                            if self.played_gifs.contains(&gif.path) {
                                gif.played = true;
                            }
                            gif
                        })
                        .map(RefCell::new)
                        .map(Rc::new)
                        .collect::<Vec<_>>();
                    gifs.shuffle(&mut thread_rng());
                    gifs
                }
            };
            self.gifs = filtered_gifs;
        } else {
            // If no category is set, set gifs to the placeholder.
            let empty_gif = EMPTY_PLACEHOLDER.into();
            gifs.push(empty_gif);
        }
    }
}

/// Handles all key events triggered by the user.
fn handle_keypress(e: KeyboardEvent) -> Option<Msg> {
    let key = e.key();
    if key == "Backspace" {
        Some(Msg::Backspace)
    } else if key == "ArrowRight" {
        Some(Msg::Forward(ForwardType::Skip))
    } else if key == "ArrowLeft" {
        Some(Msg::Backward)
    } else if key == "Escape" {
        Some(Msg::Escape)
    } else if key == "Enter" {
        Some(Msg::Forward(ForwardType::Success))
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
