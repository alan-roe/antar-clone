use leptos::*;
use let_me_talk::{
    system::{Chat, Colour, Content, PChat, PMessage, PMessages, PSender},
    web::*,
};
use uuid::Uuid;

type WebChat = PChat<Uuid, PMessage<PSender>, PMessages<Uuid, PMessage<PSender>>>;

#[component]
fn App() -> impl IntoView {
    let value = RwSignal::new("".to_string());
    let (chat, set_chat) = create_signal(WebChat::new());
    log::info!("start of App");
    view! {
        <h1 class="bg-red-400">"Let Me Talk"</h1>
        <h2>"Chat name: " {move || chat.with(|chat| chat.name().to_owned())} </h2>
        <Messages messages=chat.with(|chat| chat.messages().read_only())
        />
        <Input 
            value=value
            on_submit={ move |evt| {
                log::info!("submitted {evt}");
                set_chat.update(|chat| {
                    chat.send_message(PMessage::new(PSender::new("New Sender", "", Colour::BLACK), Content::text(value())));
                    *chat.name_mut() = value();
                });
                // value.set("".to_owned());
            }} 
        />
        // <Messages />
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <App/> })
}
