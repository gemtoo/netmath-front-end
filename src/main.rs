#![allow(non_snake_case, unused)]

use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    let mut subnet = use_signal(|| "5.4.3.2/31".to_string());
    let mut signal_calc_result = use_signal(|| "Waiting for input...".to_string());
    rsx! {
        head {
            link {
                rel: "stylesheet",
                href: "https://static.gemtoo.dev/assets/style.css"
            }
            title { "Subnet Calculator" }
        }
        body {
            table {
                tbody {
                    tr {
                        td {
                            p {
                                "style": "text-align: center;",
                                img {
                                    src: "https://static.gemtoo.dev/assets/graph.webp",
                                    style: "width: 250px;",
                                } br {} br {}
                                strong { "Subnet Calculator" }
                                br {} br {}
                                "Type in IP addresses, CIDR notations, binary, hex." br {}
                                "For example: 10.13.37.10, 200::/7, 0xBABEFACE, 001011011010." br {} br {}
                                input {
                                    value: "{subnet}",
                                    oninput: move |event| {
                                        let eventval = event.value();
                                        let mut raw_calc_result = Vec::new();
                                        let mut calc_result = String::new();
                                        if is_whitelisted(&eventval) {
                                            subnet.set(eventval.clone());
                                            let cmd_output = std::process::Command::new("subnetcalc")
                                            	.arg(eventval)
                                            	.arg("-nocolor")
                                            	.arg("-n")
                                            	.output()
                                            	.unwrap();
                                            if cmd_output.stdout.len() != 0 {
                                            raw_calc_result = cmd_output.stdout;
                                            } else {
                                            raw_calc_result = cmd_output.stderr;
                                            }
                                            calc_result = String::from_utf8(raw_calc_result).unwrap()
                                            	.replace("\n", "<br>")
                                            	.replace("ERROR: ", "")
                                            	.replace("!", ".")
                                            	.replace("{ ", "")
                                            	.replace(" }", "");
                                            signal_calc_result.set(calc_result.into());
                                        } else {
                                            subnet.set(eventval.clone());
                                            signal_calc_result.set("This pattern is restricted.".into());
                                        }
                                    },
                                }
                                p { dangerous_inner_html: "{signal_calc_result}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

const ALLOWED_CHARS: &str = "0123456789abcdefxABCDEFX.:/";
const ALLOWED_LETTERS: &str = "abcdefxABCDEFX";

fn is_whitelisted(input: &str) -> bool {
    // By default subnetcalc can also query against DNS and make use of the network.
    // This makes frontend sluggish. Any input that contains both dot and a letter, should not be allowed.
    // Because it is going to be checked against DNS and it could never be an IP address.
    if input.len() >= 2 {
        let has_dot = input.chars().any(|c| ".".contains(c));
        let has_allowed_letter = input.chars().any(|c| ALLOWED_LETTERS.contains(c));
        if has_dot == true && has_allowed_letter == true {
            return false;
        }
    }
    // Maximum possible input is 79 chars of length and is as follows:
    // ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff/ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff
    // Anything beyond is restricted for security reasons.
    if input.len() >= 80 {
        return false;
    }
    let are_all_chars_allowed = input.chars().all(|c| ALLOWED_CHARS.contains(c));
    return are_all_chars_allowed;
}
