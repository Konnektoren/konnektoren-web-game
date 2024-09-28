use konnektoren_yew::storage::{ProfileStorage, Storage};
use std::sync::Arc;
use yew::prelude::*;
use yew_chat::prelude::{ChatApp, MessageHandler, RequestMessageHandler};

const API_URL: &str = "https://api.konnektoren.help/api/v1/chat";

#[derive(Properties, Clone, PartialEq)]
pub struct ChatProps {
    pub challenge_id: String,
}

#[function_component(Chat)]
pub fn chat(props: &ChatProps) -> Html {
    let profile = ProfileStorage::default().get("").unwrap_or_default();
    let channel = format!("challenge-{}", props.challenge_id);
    let handler = Arc::new(RequestMessageHandler {
        host: API_URL.to_string(),
    }) as Arc<dyn MessageHandler>;

    let expanded = use_state(|| false);

    let on_toggle = {
        let expanded = expanded.clone();
        Callback::from(move |_| {
            expanded.set(!*expanded);
        })
    };

    html! {
        <div class={classes!("chat-popup", if *expanded { "expanded" } else { "" })}>
            <div class="chat-bubble" onclick={on_toggle.clone()}>
                <span class="chat-icon">{"💬"}</span>
                <span class="chat-text">{"Chat"}</span>
            </div>
            if *expanded {
                <div class="chat-content">
                    <button class="close-button" onclick={on_toggle}>{"×"}</button>
                    <ChatApp user={profile.name.clone()} channel={channel.clone()} handler={handler.clone()} />
                </div>
            }
        </div>
    }
}