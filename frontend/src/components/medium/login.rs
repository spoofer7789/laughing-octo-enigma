use reqwasm::http::Request;
use serde::Serialize;
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen_futures::*;
use yew::events::SubmitEvent;
use serde_json;
#[derive(Debug, Serialize)]
struct LoginData {
    username: String,
    password: String,
}

#[function_component(Loginform)]
pub fn login() -> Html {
    let user = use_state(|| "".to_string());
    let pass = use_state(|| "".to_string());
    let error = use_state(|| "".to_string());

    let onsubmit = {
        let user = user.clone();
        let pass = pass.clone();
        let error = error.clone();
        Callback::from(move |e: SubmitEvent| {
            // Prevent the default behavior of form submission
            e.prevent_default();
    
            let user = user.clone();
            let pass = pass.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp  = Request::post("/backend")
                    .header("Content-Type", "application/json")
                    .body(JsValue::from_str(
                        &serde_json::to_string(&LoginData {
                            username: (*user).to_string(),
                            password: (*pass).to_string(),
                        })
                        .unwrap(),
                    ))
                    .send()
                    .await;
                    
            });
        })
    };

    let user_onchange = {
        let user = user.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            user.set(input.value());
        })
    };

    let pass_onchange = {
        let pass = pass.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            pass.set(input.value());
        })
    };

    html! {
        <>
            <section class="section">
                <div class="container">
                    <div class="columns is-centered">
                        <div class="column is-four-fifths">
                            <div class="card">
                                <header class="card-header">
                                    <p class="card-header-title">
                                        {"Login"}
                                    </p>
                                </header>
                                <div class="card-content">
                                    <div class="content">
                                        if error.len() > 0 {
                                            <div class="notification is-danger">
                                               {(*error).clone()}
                                            </div>
                                        }
                                        <form {onsubmit}>
                                            <div class="field">
                                                <label class="label">{"Username"}</label>
                                                <div class="control">
                                                    <input onchange={user_onchange} value={(*user).clone()} class="input" type="text" placeholder="username" />
                                                </div>
                                            </div>

                                            <div class="field">
                                                <label class="label">{"Password"}</label>
                                                <div class="control">
                                                    <input onchange={pass_onchange} value={(*pass).clone()} class="input" type="password" placeholder="password" />
                                                </div>
                                            </div>

                                            <div class="field">
                                                <div class="control">
                                                    <button class="button is-link" type="submit">{"Login"}</button>
                                                </div>
                                            </div>
                                        </form>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        </>
    }
}