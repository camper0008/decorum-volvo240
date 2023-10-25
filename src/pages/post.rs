use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct PostPageProps {
    pub post_id: String,
    pub category_id: String,
}

#[derive(Clone, PartialEq, Deserialize)]
struct PostItem {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

#[function_component(Post)]
fn post(props: &PostPageProps) -> Html {
    let post = use_state(|| None);
    {
        let post = post.clone();
        use_effect_with((), move |_| {
            let post = post.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_videos: Vec<PostItem> = Request::get("/tutorial/data.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                post.set(Some(fetched_videos));
            });
            || ()
        });
    };

    html! {
        <div class="content-area">
            { post.as_ref().map(|f| f.iter().map(|f| html!{ <>{ f.id } {"-"} { f.clone().title } <br /></> }).collect::<Html>()) }
        </div>
    }
}

#[function_component(PostPage)]
pub fn post(props: &PostPageProps) -> Html {
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
