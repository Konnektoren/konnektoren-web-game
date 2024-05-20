use crate::{
    components::Navigation,
    pages::{about::AboutPage, home::HomePage, map::MapPage},
    route::Route,
    utils::translation::{translations, LANGUAGES},
};
use yew::prelude::*;
use yew_i18n::I18nProvider;

use yew_router::{BrowserRouter, Switch};

fn switch_main(route: Route) -> Html {
    match route {
        Route::About => html! {<AboutPage /> },
        Route::Home => html! {<HomePage />},
        Route::Map => html! {<MapPage />},
    }
}

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let supported_languages = LANGUAGES.to_vec();
        let translations = translations();
        html! {
            <I18nProvider supported_languages={supported_languages} translations={translations} >
            <div>
            <BrowserRouter>
                <Navigation />
                <Switch<Route> render={switch_main} />
            </BrowserRouter>
            </div>
            </I18nProvider>
        }
    }
}