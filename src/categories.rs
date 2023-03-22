use strum::{EnumIter, IntoEnumIterator};
use strum_macros::Display;
use yew::{classes, html, Callback, Component, Properties};

/// Component which displays nearly all categories of helix editor modes.
pub struct Categories;

#[derive(Properties, Clone, PartialEq)]
pub struct CategoriesProps {
    /// Nearly all available categories. By default categories consist
    /// of all possible `Category` enum values.
    pub categories: Vec<Category>,
    /// Callback which handles the click on a category.
    pub callback: Callback<Category>,
}

/// Nearly all possible mode categories mentioned by the helix editor docs.
#[derive(Display, Clone, PartialEq, EnumIter)]
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
        let categories = &ctx.props().categories;
        html! {
            <div class="sidebox">
                <div class="categories">
                    <div class="title">{"Choose categories:"}</div>
                    {for Category::iter().map(|category| {
                        let class = if categories.contains(&category) {
                            "active"
                        } else if category.is_disabled() {
                            "disabled"
                        } else {
                            "inactive"
                        };
                        let callback = ctx.props().callback.clone();
                        let category_clone = category.clone();
                        let onclick = Callback::from(move |_| {
                            let c = &category_clone;
                            callback.emit(c.clone());
                        });
                        html! {
                            <div class={classes!(class)} {onclick}>
                                {category.to_string()}
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
            Category::NormalModeChanges => true,
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

    /// Toggles the category and returns all remaining categories.
    /// If the category is already activated, make it
    /// inactive. If inactive, make it active. If disabled, do nothing.
    pub fn toggle_category(&self, categories: &[Category]) -> Vec<Category> {
        let mut categories = categories.to_owned();
        if self.is_disabled() {
            return categories;
        }
        if categories.contains(self) {
            categories = categories
                .into_iter()
                .filter(|c| c != self)
                .collect::<Vec<_>>();
        } else {
            categories.push(self.clone());
        }
        categories
    }
}
