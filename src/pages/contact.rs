use yew::prelude::*;

#[function_component(ContactPage)]
pub fn contact_page() -> Html {
    html! {
        <div class="contact">
            <h1>{"Kontakt"}</h1>
            <ul>
                <li><p>{"Kontakt <email0@example.org>, hvis du har opdaget en fejl, med volvo240."}</p></li>
                <li><p>{"Kontakt <email1@example.org>, for sponsoring af volvo240.dk m.v."}</p></li>
            </ul>
        </div>
    }
}
