use serde::Serialize;
use strum::{EnumIter, IntoEnumIterator};
use strum_macros::Display;
use yew::{classes, html, Callback, Component, Properties};

use crate::constants::{NORMAL_MODE_CHANGES, NORMAL_MODE_MOVEMENT};

/// Component which displays nearly all categories of helix editor modes.
pub struct Categories;

#[derive(Properties, Clone, PartialEq)]
pub struct CategoriesProps {
    /// Nearly all available categories. By default categories consist
    /// of all possible `Category` enum values.
    pub active_category: Option<Category>,
    /// Callback which handles the click on a category.
    pub callback: Callback<Category>,
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
}

impl Component for Categories {
    type Message = ();

    type Properties = CategoriesProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let categories = Category::iter();
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
                        let callback = ctx.props().callback.clone();
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
            Category::NormalModeSelect => true,
            Category::NormalModeSearch => true,
            Category::ViewMode => true,
            Category::GotoMode => true,
            Category::MatchMode => true,
            Category::WindowMode => true,
            Category::SpaceMode => true,
            Category::InsertMode => true,
            Category::SelectMode => true,
            Category::Picker => true,
            Category::Prompt => true,
        }
    }

    pub fn get_gifs(&self) -> &[(&str, &[&str], &str)] {
        match self {
            Category::NormalModeMovement => NORMAL_MODE_MOVEMENT,
            Category::NormalModeChanges => NORMAL_MODE_CHANGES,
            Category::NormalModeSelect => todo!(),
            Category::NormalModeSearch => todo!(),
            Category::ViewMode => todo!(),
            Category::GotoMode => todo!(),
            Category::MatchMode => todo!(),
            Category::WindowMode => todo!(),
            Category::SpaceMode => todo!(),
            Category::InsertMode => todo!(),
            Category::SelectMode => todo!(),
            Category::Picker => todo!(),
            Category::Prompt => todo!(),
        }
    }
}
