use crate::{
    api::user_from_session,
    models::{UserOrError, UserState},
    pages::{
        CategoryListPage, ContactPage, HomePage, LoginOrRegister, LoginOrRegisterPage,
        NotFoundPage, PostListPage, PostPage,
    },
    route::Route,
};
use gloo_net::http::Request;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    let commit = option_env!("COMMIT_REF")
        .map(|hash| html! { <p class="commit">{"(version: "} { hash } {")"}</p> });
    html! {
    <footer>
        <Link<Route> to={Route::Home}>{ "Startside" }</Link<Route>>
        {" | "}
        <Link<Route> to={Route::Contact}>{ "Kontakt" }</Link<Route>>
        <p>{"volvo240.dk"}</p>
        {commit}
    </footer>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct HeaderProps {
    user: UserState,
}

#[function_component(Header)]
fn header(props: &HeaderProps) -> Html {
    let user = props.user.clone();
    let logout_click = {
        let user = user.clone();
        move |event: MouseEvent| {
            event.prevent_default();
            let user = user.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let user = user.clone();
                let response = Request::post("/api/users/logout").send().await.unwrap();
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

fn switch(routes: Route, user: UserState) -> Html {
    match routes {
        Route::CategoryList => {
            html! { <CategoryListPage /> }
        }
        Route::Login => {
            html! { <LoginOrRegisterPage variant={LoginOrRegister::Login} user={user} /> }
        }
        Route::Register => {
            html! { <LoginOrRegisterPage variant={LoginOrRegister::Register} user={user} /> }
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
    let user = use_state(|| None);

    {
        let user = user.clone();
        use_effect_with((), move |_| {
            let user = user.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = user_from_session().await;
                user.set(Some(response.data));
            });
            || ()
        });
    };

    html! {
        <>
            <BrowserRouter>
                <Header user={user.clone()} />
                <hr />
                <main>
                        <Switch<Route> render={move |route| switch(route, user.clone())} />
                </main>
                <hr />
                <Footer />
            </BrowserRouter>
        </>
    }
}
