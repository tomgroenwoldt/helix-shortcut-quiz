use serde::Serialize;
use strum::{EnumIter, IntoEnumIterator};
use strum_macros::Display;
use yew::{classes, html, Callback, Component, Properties};

use crate::{
    constants::{
        NORMAL_MODE_CHANGES, NORMAL_MODE_GOTO_MODE, NORMAL_MODE_MATCH_MODE, NORMAL_MODE_MOVEMENT,
        NORMAL_MODE_SEARCH, NORMAL_MODE_SELECT, NORMAL_MODE_VIEW_MODE, NORMAL_MODE_WINDOW_MODE,
    },
    gif::GifWrapper,
};

/// Component which displays nearly all categories of helix editor modes.
pub struct Categories;

#[derive(Properties, Clone, PartialEq)]
pub struct CategoriesProps {
    /// Nearly all available categories. By default categories consist
    /// of all possible `Category` enum values.
    pub active_category: Option<Category>,
    /// Callback which handles the click on a category.
    pub on_category_click: Callback<Category>,
    pub on_reset_click: Callback<Category>,
}

/// Nearly all possible mode categories mentioned by the helix editor docs.
#[derive(Display, Clone, PartialEq, EnumIter, Serialize)]
pub enum Category {
    #[strum(serialize = "Normal Mode - Movement")]
    NormalModeMovement,
    #[strum(serialize = "Normal Mode - Changes")]
    NormalModeChanges,
    #[strum(serialize = "Normal Mode - Select")]
    NormalModeSelect,
    #[strum(serialize = "Normal Mode - Search")]
    NormalModeSearch,
    #[strum(serialize = "View Mode")]
    ViewMode,
    #[strum(serialize = "Goto Mode")]
    GotoMode,
    #[strum(serialize = "Match Mode")]
    MatchMode,
    #[strum(serialize = "Window Mode")]
    WindowMode,
    #[strum(serialize = "Space Mode")]
    SpaceMode,
    #[strum(serialize = "Insert Mode")]
    InsertMode,
    #[strum(serialize = "Select Mode")]
    SelectMode,
    Picker,
    Prompt,
    Random,
}

impl Component for Categories {
    type Message = ();

    type Properties = CategoriesProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let categories = Category::iter();

        let reset_button = if let Some(category) = &ctx.props().active_category {
            if category.eq(&Category::Random) {
                html! {
                    <div class="disabled-reset">
                        <div>{"Reset this category"}</div>
                    </div>
                }
            } else {
                let category_clone = category.clone();
                let callback_clone = ctx.props().on_reset_click.clone();
                let onclick = Callback::from(move |_| {
                    let c = category_clone.clone();
                    callback_clone.emit(c);
                });
                html! {
                    <div class="reset">
                        <div {onclick}>{"Reset this category"}</div>
                    </div>
                }
            }
        } else {
            html! {
                <div class="disabled-reset">
                    <div>{"Reset this category"}</div>
                </div>
            }
        };

        html! {
            <div class="sidebox">
                <div class="categories">
                    <div class="title">{"Choose a category:"}</div>
                    {for categories.map(|category| {
                        let first_category_clone = category.clone();
                        let class = if ctx.props().active_category.eq(&Some(category)) {
                            "active"
                        } else if first_category_clone.is_disabled() {
                            "disabled"
                        } else {
                            "inactive"
                        };
                        let callback = ctx.props().on_category_click.clone();
                        let category_clone = first_category_clone.clone();
                        let onclick = Callback::from(move |_| {
                            let c = &category_clone;
                            callback.emit(c.clone());
                        });
                        html! {
                            <div class={classes!(class)} {onclick}>
                                {first_category_clone.to_string()}
                            </div>
                        }
                    }
                    )}
                </div>
                {reset_button}
            </div>
        }
    }
}

impl Category {
    // TODO: Remove this method after all GIFs are generated.
    /// Indicates whether the category is disabled. A category is disabled
    /// if the GIFs aren't generated yet.
    pub fn is_disabled(&self) -> bool {
        match self {
            Category::NormalModeMovement => false,
            Category::NormalModeChanges => false,
            Category::NormalModeSelect => false,
            Category::NormalModeSearch => false,
            Category::ViewMode => false,
            Category::GotoMode => false,
            Category::MatchMode => false,
            Category::WindowMode => false,
            Category::SpaceMode => true,
            Category::InsertMode => true,
            Category::SelectMode => true,
            Category::Picker => true,
            Category::Prompt => true,
            Category::Random => false,
        }
    }

    pub fn get_gifs(&self) -> Vec<GifWrapper> {
        match self {
            Category::NormalModeMovement => NORMAL_MODE_MOVEMENT
                .iter()
                .map(|&gif| gif.into())
                .collect::<Vec<_>>(),
            Category::NormalModeChanges => NORMAL_MODE_CHANGES
                .iter()
                .map(|&gif| gif.into())
                .collect::<Vec<_>>(),
            Category::NormalModeSelect => NORMAL_MODE_SELECT
                .iter()
                .map(|&gif| gif.into())
                .collect::<Vec<_>>(),
            Category::NormalModeSearch => NORMAL_MODE_SEARCH
                .iter()
                .map(|&gif| gif.into())
                .collect::<Vec<_>>(),
            Category::ViewMode => NORMAL_MODE_VIEW_MODE
                .iter()
                .map(|&gif| gif.into())
                .collect::<Vec<_>>(),
            Category::GotoMode => NORMAL_MODE_GOTO_MODE
                .iter()
                .map(|&gif| gif.into())
                .collect::<Vec<_>>(),
            Category::MatchMode => NORMAL_MODE_MATCH_MODE
                .iter()
                .map(|&gif| gif.into())
                .collect::<Vec<_>>(),
            Category::WindowMode => NORMAL_MODE_WINDOW_MODE
                .iter()
                .map(|&gif| gif.into())
                .collect::<Vec<_>>(),
            Category::SpaceMode => vec![],
            Category::InsertMode => vec![],
            Category::SelectMode => vec![],
            Category::Picker => vec![],
            Category::Prompt => vec![],
            Category::Random => Category::iter()
                .filter(|c| !c.eq(&Category::Random))
                .flat_map(|c| c.get_gifs())
                .collect::<Vec<_>>(),
        }
    }

    pub fn base_path(&self) -> String {
        match self {
            Category::NormalModeMovement => String::from("normal-mode/movement"),
            Category::NormalModeChanges => String::from("normal-mode/changes"),
            Category::NormalModeSelect => String::from("normal-mode/select"),
            Category::NormalModeSearch => String::from("normal-mode/search"),
            Category::ViewMode => String::from("normal-mode/minor-modes/view-mode"),
            Category::GotoMode => String::from("normal-mode/minor-modes/goto-mode"),
            Category::MatchMode => String::from("normal-mode/minor-modes/match-mode"),
            Category::WindowMode => String::from("normal-mode/minor-modes/window-mode"),
            Category::SpaceMode => todo!(),
            Category::InsertMode => todo!(),
            Category::SelectMode => todo!(),
            Category::Picker => todo!(),
            Category::Prompt => todo!(),
            Category::Random => String::from("random"),
        }
    }
}
