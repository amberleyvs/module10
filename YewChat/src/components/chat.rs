use yew::prelude::*;
use crate::User;

#[function_component(Chat)]
pub fn chat() -> Html {
    let user = use_context::<User>().expect("No context found.");
    let username = user.username.borrow().clone();

    html! {
        <div class="bg-gray-900 text-white w-screen h-screen flex flex-col justify-center items-center">
            <h1 class="text-4xl font-bold mb-4">{"Chat Page 💬"}</h1>
            <p class="text-xl">{"Welcome, "}{username}</p>
            <p class="text-gray-400 mt-4">{"WebSocket chat logic will be added next."}</p>
        </div>
    }
}
