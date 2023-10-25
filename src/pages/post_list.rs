use crate::route::Route;
use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Deserialize)]
struct CategoryPostsResponse {
    ok: bool,
    data: Vec<PostItem>,
}

#[derive(Clone, PartialEq, Deserialize)]
struct PostItem {
    pub id: String,
    pub category_id: String,
    pub title: String,
    pub content: String,
    pub creator_id: String,
    pub deleted: bool,
    pub locked: bool,
    pub date_created: String,
    pub date_edited: Option<String>,
}

#[derive(Clone, PartialEq, Deserialize)]
pub enum Permission {
    Banned,
    Unverified,
    User,
    Admin,
    Root,
}

#[derive(Clone, PartialEq, Properties)]
pub struct PostsProps {
    pub category_id: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct PostListProps {
    pub category_id: String,
    pub category_title: String,
}

#[function_component(Posts)]
fn posts(props: &PostsProps) -> Html {
    let posts = use_state(|| Vec::new());
    let category_id = props.category_id.clone();

    {
        let posts = posts.clone();
        use_effect_with((), move |_| {
            let posts = posts.clone();
            let category_id = category_id.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response: CategoryPostsResponse = Request::get(&format!(
                    "/api/posts/posts_from_category/{}",
                    category_id.clone()
                ))
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
                posts.set(response.data);
            });
            || ()
        });
    };

    html! {
        posts
            .iter()
            .map(|post| {
                let post = post.clone();
                html!{
                    <div class="list-item">
                        <div class="list-item-title">
                            <Link<Route> to={Route::Post { post_title: post.title.clone(), category_id: props.category_id.clone(), post_id: post.id }}>{ post.title }</Link<Route>>
                        </div>
                    </div>
                }
            })
            .collect::<Html>()
    }
}

#[function_component(PostListPage)]
pub fn post_list_page(props: &PostListProps) -> Html {
    html! {<>
        <h2>{props.category_title.clone()}</h2>
        <div class="forum_list">
            <div class="list">
                <div class="list-item list-header">
                    <div class="list-item-title">
                        {"Indlæg"}
                    </div>
                </div>

                <Posts category_id={props.category_id.clone()}/>
            </div>
        </div>
    </>}
}
