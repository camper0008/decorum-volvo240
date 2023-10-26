use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;

use crate::models::PostItem;

#[derive(Clone, PartialEq, Properties)]
pub struct PostPageProps {
    pub post_id: String,
    pub category_id: String,
}

#[derive(Clone, PartialEq, Deserialize)]
struct PostItemResponse {
    ok: bool,
    data: PostItem,
}

#[function_component(Post)]
fn post(props: &PostPageProps) -> Html {
    let post = use_state(|| None);
    let post_id = props.post_id.clone();
    let category_id = props.category_id.clone();
    {
        let post = post.clone();
        use_effect_with((), move |_| {
            let post = post.clone();
            let post_id = post_id.clone();
            let category_id = category_id.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_post: PostItemResponse =
                    Request::get(&format!("/api/posts/post_from_id/{category_id}/{post_id}"))
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                post.set(Some(fetched_post));
            });
            || ()
        });
    };

    html! {
        <div class="content-area">
            {
                post
                    .as_ref()
                    .map(|p| p.clone())
                    .map(|p|
                        html!{
                            <>
                            <h2>{ p.data.title }</h2>
                            <p>{ p.data.content }</p>
                            </>
                        }
                )
            }
        </div>
    }
}

#[function_component(PostPage)]
pub fn post_page(props: &PostPageProps) -> Html {
    let PostPageProps {
        category_id,
        post_id,
    } = props;
    let fallback_fn = html! {<h1>{"waiting..."}</h1>};
    html! {
        <div class="post">
            <Suspense fallback={fallback_fn}>
                <Post category_id={category_id.clone()} post_id={post_id.clone()} />
            </Suspense>
        </div>
    }
}
