use gloo::utils::document;
use yew::prelude::*;

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    use_effect(|| {
        document().set_title(&format!("Konnektoren - {}", "404 - Page Not Found"));
        || ()
    });
    html! {
        <div class="not-found-page">
            <h1>{ "404 - Page Not Found" }</h1>
            <p>{ "The page you are looking for does not exist." }</p>
            <a href="/">{ "Go to Home" }</a>
        </div>
    }
}
