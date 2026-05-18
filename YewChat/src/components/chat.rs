use yew::prelude::*;
use crate::User;

#[function_component(Chat)]
pub fn chat() -> Html {
    let user = use_context::<User>().expect("No context found.");
    let username = user.username.borrow().clone();

    html! {
        <div style="
            font-family: 'Poppins', sans-serif;
            min-height: 100vh;
            width: 100vw;
            background: linear-gradient(135deg, #CFF5E7 0%, #BCEAD5 25%, #E8F3D6 50%, #FDF6BD 75%, #FFD6A5 100%);
            display: flex;
            align-items: center;
            justify-content: center;
            padding: 32px;
            box-sizing: border-box;
            color: #2f3e46;
        ">
            <main style="
                width: 100%;
                max-width: 840px;
                height: 78vh;
                background: rgba(255, 255, 255, 0.82);
                border-radius: 36px;
                box-shadow: 0 30px 70px rgba(47, 62, 70, 0.14);
                border: 1px solid rgba(255, 255, 255, 0.75);
                overflow: hidden;
                display: flex;
                flex-direction: column;
                backdrop-filter: blur(18px);
            ">
                <header style="
                    padding: 26px 34px;
                    border-bottom: 1px solid rgba(82, 121, 111, 0.16);
                    background: rgba(255, 255, 255, 0.58);
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                ">
                    <div>
                        <h1 style="
                            margin: 0;
                            font-size: 26px;
                            font-weight: 800;
                            color: #2f3e46;
                        ">
                            {"Alice"}
                        </h1>

                        <p style="
                            margin: 6px 0 0 0;
                            font-size: 13px;
                            font-weight: 700;
                            color: #52796f;
                        ">
                            {"Signed in as "}<strong>{username.clone()}</strong>
                        </p>
                    </div>

                    <div style="
                        background: #BCEAD5;
                        color: #2f3e46;
                        padding: 10px 16px;
                        border-radius: 999px;
                        font-size: 12px;
                        font-weight: 800;
                        letter-spacing: 0.05em;
                    ">
                        {"ONLINE"}
                    </div>
                </header>

                <section style="
                    flex: 1;
                    padding: 34px;
                    display: flex;
                    flex-direction: column;
                    gap: 24px;
                    overflow-y: auto;
                    background: rgba(255, 255, 255, 0.26);
                ">
                    <div style="
                        display: flex;
                        justify-content: flex-start;
                    ">
                        <div style="
                            max-width: 62%;
                        ">
                            <p style="
                                margin: 0 0 8px 4px;
                                font-size: 12px;
                                color: #52796f;
                                font-weight: 800;
                            ">
                                {"Alice"}
                            </p>

                            <div style="
                                background: white;
                                padding: 16px 18px;
                                border-radius: 22px 22px 22px 6px;
                                box-shadow: 0 12px 28px rgba(47, 62, 70, 0.08);
                                font-size: 15px;
                                font-weight: 500;
                                line-height: 1.6;
                            ">
                                {"Hi!"}
                            </div>
                        </div>
                    </div>

                    <div style="
                        display: flex;
                        justify-content: flex-end;
                    ">
                        <div style="
                            max-width: 62%;
                            text-align: right;
                        ">
                            <p style="
                                margin: 0 4px 8px 0;
                                font-size: 12px;
                                color: #52796f;
                                font-weight: 800;
                            ">
                                {username.clone()}
                            </p>

                            <div style="
                                background: #2f3e46;
                                color: white;
                                padding: 16px 18px;
                                border-radius: 22px 22px 6px 22px;
                                box-shadow: 0 12px 28px rgba(47, 62, 70, 0.16);
                                font-size: 15px;
                                font-weight: 500;
                                line-height: 1.6;
                                text-align: left;
                            ">
                                {"Hello!"}
                            </div>
                        </div>
                    </div>

                    <div style="
                        display: flex;
                        justify-content: flex-start;
                    ">
                        <div style="
                            max-width: 62%;
                        ">
                            <p style="
                                margin: 0 0 8px 4px;
                                font-size: 12px;
                                color: #52796f;
                                font-weight: 800;
                            ">
                                {"Alice"}
                            </p>

                            <div style="
                                background: white;
                                padding: 16px 18px;
                                border-radius: 22px 22px 22px 6px;
                                box-shadow: 0 12px 28px rgba(47, 62, 70, 0.08);
                                font-size: 15px;
                                font-weight: 500;
                                line-height: 1.6;
                            ">
                                {"How are you?"}
                            </div>
                        </div>
                    </div>

                    <div style="
                        display: flex;
                        justify-content: flex-end;
                    ">
                        <div style="
                            max-width: 62%;
                            text-align: right;
                        ">
                            <p style="
                                margin: 0 4px 8px 0;
                                font-size: 12px;
                                color: #52796f;
                                font-weight: 800;
                            ">
                                {username.clone()}
                            </p>

                            <div style="
                                background: #2f3e46;
                                color: white;
                                padding: 16px 18px;
                                border-radius: 22px 22px 6px 22px;
                                box-shadow: 0 12px 28px rgba(47, 62, 70, 0.16);
                                font-size: 15px;
                                font-weight: 500;
                                line-height: 1.6;
                                text-align: left;
                            ">
                                {"I'm good."}
                            </div>
                        </div>
                    </div>
                </section>

                <footer style="
                    padding: 22px 28px;
                    background: rgba(255, 255, 255, 0.62);
                    border-top: 1px solid rgba(82, 121, 111, 0.16);
                ">
                    <div style="
                        display: flex;
                        align-items: center;
                        gap: 12px;
                        background: white;
                        border-radius: 22px;
                        padding: 10px;
                        box-shadow: 0 10px 28px rgba(47, 62, 70, 0.08);
                    ">
                        <input
                            style="
                                flex: 1;
                                border: none;
                                outline: none;
                                padding: 14px 16px;
                                font-family: 'Poppins', sans-serif;
                                font-size: 14px;
                                font-weight: 500;
                                color: #2f3e46;
                            "
                            placeholder="Write a message..."
                        />

                        <button style="
                            border: none;
                            border-radius: 16px;
                            background: #2f3e46;
                            color: white;
                            padding: 14px 22px;
                            font-family: 'Poppins', sans-serif;
                            font-weight: 800;
                            font-size: 14px;
                        ">
                            {"SEND"}
                        </button>
                    </div>
                </footer>
            </main>
        </div>
    }
}