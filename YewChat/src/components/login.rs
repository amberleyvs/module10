use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();

        Callback::from(move |_| {
            *user.username.borrow_mut() = (*username).clone();
        })
    };

    html! {
        <div style="
            font-family: 'Poppins', sans-serif;
            min-height: 100vh;
            width: 100vw;
            background: linear-gradient(135deg, #CFF5E7 0%, #BCEAD5 25%, #E8F3D6 50%, #FDF6BD 75%, #FFD6A5 100%);
            display: flex;
            align-items: center;
            justify-content: center;
            color: #2f3e46;
        ">
            <main style="
                width: 100%;
                max-width: 520px;
                padding: 24px;
            ">
                <div style="
                    background: rgba(255, 255, 255, 0.72);
                    border-radius: 28px;
                    padding: 44px;
                    box-shadow: 0 30px 70px rgba(47, 62, 70, 0.14);
                    border: 1px solid rgba(255, 255, 255, 0.75);
                    text-align: center;
                    backdrop-filter: blur(18px);
                ">
                    <div style="
                        width: 72px;
                        height: 72px;
                        margin: 0 auto 24px auto;
                        border-radius: 24px;
                        background: linear-gradient(135deg, #BCEAD5, #FFD6A5);
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        font-size: 32px;
                        font-weight: 800;
                        color: #2f3e46;
                        box-shadow: 0 16px 32px rgba(47, 62, 70, 0.12);
                    ">
                        {"C"}
                    </div>

                    <h1 style="
                        font-size: 34px;
                        line-height: 1.2;
                        font-weight: 800;
                        margin: 0 0 12px 0;
                        color: #2f3e46;
                    ">
                        {"Start chatting!"}
                    </h1>

                    <p style="
                        font-size: 15px;
                        line-height: 1.7;
                        font-weight: 500;
                        color: #52796f;
                        margin: 0 0 32px 0;
                    ">
                        {"Enter your nickname"}
                    </p>

                    <form style="
                        width: 100%;
                        display: flex;
                        flex-direction: column;
                        gap: 18px;
                        text-align: left;
                    ">
                        <label style="
                            font-size: 12px;
                            font-weight: 800;
                            letter-spacing: 0.08em;
                            color: #52796f;
                            margin-left: 4px;
                        ">
                            {"NICKNAME"}
                        </label>

                        <input
                            {oninput}
                            style="
                                width: 100%;
                                box-sizing: border-box;
                                border: none;
                                outline: none;
                                border-radius: 18px;
                                padding: 18px 20px;
                                font-family: 'Poppins', sans-serif;
                                font-size: 15px;
                                font-weight: 500;
                                color: #2f3e46;
                                background: rgba(255, 255, 255, 0.88);
                                box-shadow: inset 0 0 0 1px rgba(82, 121, 111, 0.18);
                            "
                            placeholder="Type your nickname..."
                        />

                        <Link<Route> to={Route::Chat}>
                            <button
                                {onclick}
                                disabled={username.len() < 1}
                                style="
                                    width: 100%;
                                    border: none;
                                    border-radius: 18px;
                                    padding: 18px 24px;
                                    background: #2f3e46;
                                    color: white;
                                    font-family: 'Poppins', sans-serif;
                                    font-size: 15px;
                                    font-weight: 800;
                                    cursor: pointer;
                                    box-shadow: 0 18px 36px rgba(47, 62, 70, 0.16);
                                "
                            >
                                {"JOIN CHAT  >"}
                            </button>
                        </Link<Route>>
                    </form>

                    <div style="
                        margin-top: 32px;
                        color: #52796f;
                        font-size: 12px;
                        font-weight: 700;
                        letter-spacing: 0.05em;
                    ">
                        {""}
                    </div>
                </div>
            </main>
        </div>
    }
}