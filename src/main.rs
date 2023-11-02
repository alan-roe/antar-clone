use leptos::*;
use let_me_talk::web::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <h1 class="bg-red-400">"Let Me Talk"</h1>
        <h2>"Chat"</h2>
        <Messages />
    }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}
