use yew::prelude::*;
use crate::components::Roulette;

#[function_component(SpeechBubble)]
pub fn speech_bubble() -> Html {
    html! {
        <div class="speech-bubble">
            <p>{ "Learn German" }</p>
            <Roulette></Roulette>
        </div>
    }
}