use crate::{
    models::User,
    pages::{
        CategoryListPage, ContactPage, HomePage, LoginOrRegister, LoginOrRegisterPage,
        NotFoundPage, PostListPage, PostPage,
    },
    route::Route,
};
use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Deserialize, PartialEq, Clone)]
#[serde(untagged)]
enum UserOrError {
    Error(String),
    User(User),
}

#[derive(Clone, PartialEq, Deserialize)]
struct UserResponse {
    ok: bool,
    data: UserOrError,
}

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
    <footer>
        <Link<Route> to={Route::Home}>{ "Startside" }</Link<Route>>
        {" | "}
        <Link<Route> to={Route::Contact}>{ "Kontakt" }</Link<Route>>
        <p>{"volvo240.dk"}</p>
    </footer>
    }
}

#[function_component(Header)]
pub fn header() -> Html {
    let user = use_state(|| None);

    {
        let user = user.clone();
        use_effect_with((), move |_| {
            let user = user.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response: UserResponse = Request::get("/api/users/user_from_session")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                user.set(Some(response.data));
            });
            || ()
        });
    };

    let logout_click = {
        let user = user.clone();
        move |event: MouseEvent| {
            event.prevent_default();
            let user = user.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let user = user.clone();
                let response = Request::get("/api/users/logout").send().await.unwrap();
                if response.status() == 200 {
                    user.set(None);
                }
            });
        }
    };

    html! {
    <header>
        <nav>
            <Link<Route> to={Route::Home}>{ "Startside" }</Link<Route>>
            <Link<Route> to={Route::CategoryList}>{ "Kategori" }</Link<Route>>
            {
                match *user {
                    Some(UserOrError::User(ref user)) =>
                        html!{ <>
                            <Link<Route> to={Route::NotFound}>{ &user.username }</Link<Route>>
                            <a href="/logout" onclick={logout_click}>{ "Log ud" }</a>
                        </> },
                    _ => html!{ <Link<Route> to={Route::Login}>{ "Log ind" }</Link<Route>> },
                }
            }
        </nav>
        <div class="header-list">
            <img src="/static/logo.jpg" />
            <div class="header-search">
                <input />
                <button>{"Søg"}</button>
            </div>
        </div>
    </header>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::CategoryList => {
            html! { <CategoryListPage /> }
        }
        Route::Login => {
            html! { <LoginOrRegisterPage variant={LoginOrRegister::Login} /> }
        }
        Route::Register => {
            html! { <LoginOrRegisterPage variant={LoginOrRegister::Register} /> }
        }
        Route::Home => {
            html! { <HomePage /> }
        }
        Route::Contact => {
            html! { <ContactPage /> }
        }
        Route::PostList {
            category_id,
            category_title,
        } => {
            html! { <PostListPage category_title={ category_title } category_id={ category_id } /> }
        }
        Route::NotFound => {
            html! { <NotFoundPage /> }
        }
        Route::Post {
            post_id,
            category_id,
            ..
        } => {
            html! { <PostPage post_id={post_id} category_id={category_id} /> }
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <BrowserRouter>
                <Header />
                <hr />
                <main>
                        <Switch<Route> render={switch} />
                </main>
                <hr />
                <Footer />
            </BrowserRouter>
        </>
    }
}
