use gloo_net::http::Request;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yew::{function_component, Html, Properties};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub html: String,
}

#[function_component(SafeHtml)]
pub fn safe_html(props: &Props) -> Html {
    let div = gloo_utils::document().create_element("div").unwrap();
    div.set_inner_html(&props.html.clone());

    Html::VRef(div.into())
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(Home)]
fn home() -> Html {
    let subnet = use_state(|| "5.4.3.2/31".to_string());
    let calc_result = use_state(|| "Waiting for input...".to_string());

    let oninput = {
        let subnet = subnet.clone();
        let calc_result = calc_result.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let eventval = input.value();

            subnet.set(eventval.clone());
            calc_result.set("Calculating...".into());

            let subnet_clone = eventval.clone();
            let calc_result = calc_result.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let body =
                    match serde_json::to_string(&serde_json::json!({ "subnet": subnet_clone })) {
                        Ok(b) => b,
                        Err(e) => {
                            calc_result.set(format!("JSON error: {}", e));
                            return;
                        }
                    };
                let response = match Request::post("https://netmath.gemtoo.dev/api")
                    .header("Content-Type", "application/json")
                    .body(body)
                    .unwrap()
                    .send()
                    .await
                {
                    Ok(resp) => resp,
                    Err(e) => {
                        calc_result.set(format!("Network error: {}", e));
                        return;
                    }
                };

                if !response.ok() {
                    calc_result.set(format!("API error: {}.", response.status()));
                    return;
                }

                match response.text().await {
                    Ok(text) => {
                        web_sys::console::log_1(&format!("API response: {}", text).into());
                        calc_result.set(text);
                    }
                    Err(e) => {
                        calc_result.set(format!("Response error: {}", e));
                    }
                }
            });
        })
    };

    html! {
        <>
            <head>
                <link rel="stylesheet" href="https://static.gemtoo.dev/assets/netmath.css" />
            </head>
            <body>
                <table>
                    <tbody>
                        <tr>
                            <td>
                                <div style="text-align: center;">
                                    <img
                                        src="https://static.gemtoo.dev/assets/graph.webp"
                                        style="width: 270px;"
                                    />
                                    <br /><br />
                                    <strong>{ "Subnet Calculator" }</strong>
                                    <br /><br />
                                    { "Type in IP addresses, CIDR notations, binary, hex." }
                                    <br />
                                    { "For example: 10.13.37.10, 200::/7, 0xBABEFACE, 001011011010." }
                                    <br /><br />
                                    <input
                                        type="text"
                                        value={(*subnet).clone()}
                                        {oninput}
                                    />
                                    <SafeHtml html={(*calc_result).clone()} />
                                </div>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </body>
        </>
    }
}
