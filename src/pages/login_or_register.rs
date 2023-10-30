use crate::{api::user_from_session, models::UserState, route::Route};
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq)]
pub enum LoginOrRegister {
    Login,
    Register,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub variant: LoginOrRegister,
    pub user: UserState,
}

#[derive(Deserialize)]
struct Response {
    ok: bool,
    data: String,
}

#[derive(Serialize, Default)]
struct User {
    pub username: String,
    pub password: String,
}

#[function_component(LoginOrRegisterPage)]
pub fn login_or_register_page(props: &Props) -> Html {
    let status_response = use_state(|| None::<Response>);
    let (text, alt_text, route, endpoint, refresh_session_on_success) = match props.variant {
        LoginOrRegister::Register => ("Opret konto", "Log ind", Route::Login, "register", false),
        LoginOrRegister::Login => ("Log ind", "Opret konto", Route::Register, "login", true),
    };
    let username = use_node_ref();
    let password = use_node_ref();
    let user = props.user.clone();
    let click = {
        let status_response = status_response.clone();
        let username = username.clone();
        let password = password.clone();
        let user = user.clone();

        Callback::from(move |ev: MouseEvent| {
            ev.prevent_default();
            let status_response = status_response.clone();
            let username = username.clone();
            let password = password.clone();
            let user = user.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let response = {
                    let response = (|| async {
                        let username: HtmlInputElement = username
                            .clone()
                            .cast()
                            .ok_or_else(|| "internal error: username: option = none".to_string())?;
                        let password: HtmlInputElement = password
                            .clone()
                            .cast()
                            .ok_or_else(|| "internal error: password: option = none".to_string())?;
                        let user = User {
                            username: username.value(),
                            password: password.value(),
                        };
                        let body = serde_json::to_string(&user).map_err(|err| err.to_string())?;
                        let response = Request::post(&format!("/api/users/{endpoint}"))
                            .header("Content-Type", "application/json")
                            .body(body)
                            .map_err(|data| data.to_string())?;
                        let response = response.send().await.map_err(|data| data.to_string())?;

                        response.json().await.map_err(|err| err.to_string())
                    })()
                    .await;

                    let response = match response {
                        Ok(response) => response,
                        Err(data) => Response { data, ok: false },
                    };
                    response
                };
                if refresh_session_on_success && response.ok {
                    let response = user_from_session().await;
                    user.set(Some(response.data));
                };
                status_response.set(Some(response));
            });
        })
    };
    html! {
        <div class="login_or_register">
            { status_response.as_ref().map(|response| {
                html! {
                    <div class={if response.ok {"success"} else {"error"}}>{&response.data}</div>
                }
            }) }
            <h1>{text}</h1>
            <form>
                <label for="username">{"Brugernavn"}</label>
                <input ref={username} type="text" name="username" id="username" />
                <label for="password">{"Adgangskode"}</label>
                <input ref={password} type="password" id="password" name="password" />
                <input type="submit" value = {text} onclick={click}/>
                <h3><Link<Route> to={route}>{alt_text}{" i stedet?"}</Link<Route>></h3>
            </form>
        </div>
    }
}
