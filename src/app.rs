use crate::{
    pages::{CategoryListPage, HomePage, NotFoundPage, PostListPage, PostPage, LoginOrRegisterPage, LoginOrRegister},
    route::Route,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
    <footer>
        <Link<Route> to={Route::Home}>{ "Startside" }</Link<Route>>
        {" | "}
        <Link<Route> to={Route::NotFound}>{ "Kontakt" }</Link<Route>>
        <p>{"volvo240.dk"}</p>
    </footer>
    }
}

#[function_component(Header)]
pub fn header() -> Html {
    html! {
    <header>
        <nav>
            <Link<Route> to={Route::Home}>{ "Startside" }</Link<Route>>
            <Link<Route> to={Route::CategoryList}>{ "Kategori" }</Link<Route>>
            <Link<Route> to={Route::Login}>{ "Log ind" }</Link<Route>>
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
