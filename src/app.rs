use crate::components::Sidenav;
use crate::pages::{LeaderboardPage, NotFoundPage, ResultsPage};
use crate::utils::translation::LANGUAGE_KEY;
use crate::{
    components::Navigation,
    pages::{AboutPage, ChallengePage, HomePage, MapPage, ProfilePage},
    route::Route,
    utils::translation::{translations, LANGUAGES},
};
use gloo::storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_i18n::I18nProvider;
use yew_router::prelude::{BrowserRouter, Switch};

fn switch_main(route: Route) -> Html {
    match route {
        Route::About => html! {<AboutPage /> },
        Route::Challenge { id } => html! {<ChallengePage {id} />},
        Route::Home => html! {<HomePage />},
        Route::Leaderboard => html! {<LeaderboardPage />},
        Route::Map => html! {<MapPage />},
        Route::Profile => html! {<ProfilePage />},
        Route::Results { code } => html! {<ResultsPage { code } />},
        Route::NotFound => html! { <NotFoundPage /> },
        #[cfg(feature = "yew-preview")]
        Route::YewPreview => html! {<crate::pages::preview::PreviewPage />},
    }
}

fn update_language(query: &String) {
    let query = query.trim_start_matches('?');

    let lang = if !query.is_empty() {
        query
            .split('&')
            .find(|part| part.starts_with("lang="))
            .and_then(|lang_part| lang_part.split('=').nth(1))
    } else {
        None
    };

    if let Some(lang) = lang {
        LocalStorage::set(LANGUAGE_KEY, lang).unwrap_or_else(|err| {
            log::error!("Error setting language in local storage: {:?}", err);
        });
    }
}

#[function_component(App)]
pub fn app() -> Html {
    use_effect_with((), |_| {
        let query = web_sys::window().unwrap().location().search().unwrap();
        update_language(&query);
    });
    let supported_languages = LANGUAGES.to_vec();
    let translations = translations();

    html! {
        <I18nProvider supported_languages={supported_languages} translations={translations} >
            <BrowserRouter>
                <Sidenav />
                <Navigation />
                <Switch<Route> render={switch_main} />
            </BrowserRouter>
        </I18nProvider>
    }
}
