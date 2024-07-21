use gloo::storage::{LocalStorage, Storage};
use yew::prelude::*;

const THEME_KEY: &str = "theme";

#[function_component(ThemeToggle)]
pub fn theme_toggle() -> Html {
    let theme =
        use_state(|| LocalStorage::get(THEME_KEY).unwrap_or_else(|_| String::from("light")));

    {
        let theme = theme.clone();
        use_effect(move || {
            let body = gloo::utils::document().body().unwrap();
            body.set_class_name(format!("theme-{}", theme.as_str()).as_str());
            LocalStorage::set(THEME_KEY, theme.as_str()).unwrap();
            || ()
        });
    }

    let toggle_theme = {
        let theme = theme.clone();
        Callback::from(move |_| {
            let new_theme = if *theme == "light" {
                "dark".to_string()
            } else {
                "light".to_string()
            };
            theme.set(new_theme);
        })
    };

    html! {
        <button onclick={toggle_theme}>
            {
                if *theme == "light" {
                    html! { <i class="fas fa-moon"></i> }
                } else {
                    html! { <i class="fas fa-sun"></i> }
                }
            }
        </button>
    }
}