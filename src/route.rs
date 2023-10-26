use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/contact")]
    Contact,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/categories")]
    CategoryList,
    #[at("/category/:category_id/:category_title")]
    PostList {
        category_id: String,
        category_title: String,
    },
    #[at("/post/:category_id/:post_id/:post_title")]
    Post {
        post_id: String,
        post_title: String,
        category_id: String,
    },
    #[not_found]
    #[at("/404")]
    NotFound,
}
