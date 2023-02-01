use crate::{Msg, Urls};
use seed::{prelude::*, *};

pub mod address;
pub mod analysis;
pub mod chain;
pub mod home;
pub mod list;
pub mod not_found;
pub mod service;
pub mod tag;

pub fn get_address() -> String {
    match document().get_element_by_id("address") {
        Some(el) => match el.get_attribute("value") {
            Some(value) => value,
            _ => {
                log("Value");
                String::new()
            }
        },
        _ => {
            log("Element not found");
            String::new()
        }
    }
}

pub fn pagination<Ms: 'static + std::fmt::Debug>(
    pagination: &crate::Pagination,
    length: usize,
    page_size: usize,
    msg: impl FnOnce(usize) -> Ms + 'static + Clone,
) -> Node<Ms> {
    let pred_msg = msg.clone();
    let post_msg = msg.clone();

    nav![
        attrs! {At::AriaLabel => "Page navigation"},
        ul![
            C!["pagination", "pagination-sm"],
            li![a![
                //attrs!{At::Href => "#", At::AriaLabel=>"Start"},
                "Start",
                ev(Ev::Click, |_| pred_msg(0)),
            ]],
            (1..(length / page_size)).map(move |i| {
                let start = i * page_size;
                let msg = msg.clone();
                li![
                    IF!(start == pagination.start => C!["active"]),
                    a![
                        span![format!("{} - {}", start, start + page_size)],
                        ev(Ev::Click, move |_| msg(start)),
                    ]
                ]
            }),
            li![a![
                attrs! {At::AriaLabel => "End"},
                "End",
                ev(Ev::Click, move |_| post_msg(std::cmp::max(
                    2,
                    length - page_size,
                )))
            ]],
        ]
    ]
}

pub fn view_header(url: &Url, model: &crate::Model) -> Node<Msg> {
    let address = model.address.clone();
    nav![
        C!["navbar", "navbar-default",],
        div![
            C!["container-fluid"],
            div![
                C!["navbar-header"],
                button![
                    C!["navbar-toggle", "collapsed"],
                    attrs! {
                        At::Type => "button",
                        At::from("data-toggle") => "collapse",
                        At::from("data-target") => "#navbar",
                        At::AriaExpanded => "false"
                    },
                    span![C!["sr-only"], "Toggle navigation"],
                    span![C!["icon-bar"]],
                    span![C!["icon-bar"]],
                    span![C!["icon-bar"]],
                ],
                a![
                    C!["navbar-brand"],
                    attrs! {At::Href => Urls::new(url).home()},
                    "CAR!!!"
                ]
            ],
            div![
                C!["collapse navbar-collapse"],
                attrs! {At::Id => "navbar"},
                ul![
                    C!["nav", "navbar-nav"],
                    li![
                        if let crate::Page::Chain(_) = &model.page {
                            C!["active"]
                        } else {
                            C![]
                        },
                        a![attrs! {At::Href => Urls::new(url).chain().list()}, "Chains"]
                    ],
                    li![
                        if let crate::Page::Tag(_) = &model.page {
                            C!["active"]
                        } else {
                            C![]
                        },
                        a![attrs! {At::Href => Urls::new(url).tag().list()}, "Tags"]
                    ],
                    li![
                        if let crate::Page::Service(_) = &model.page {
                            C!["active"]
                        } else {
                            C![]
                        },
                        a![
                            attrs! {At::Href => Urls::new(url).service().list()},
                            "Services"
                        ]
                    ],
                    li![
                        if let crate::Page::List(_) = &model.page {
                            C!["active"]
                        } else {
                            C![]
                        },
                        a![attrs! {At::Href => Urls::new(url).list().list()}, "Lists"]
                    ],
                    /*
                    li![a![
                        attrs! {At::Href => Urls::new(url).task().list()},
                        "Tasks"
                    ]],
                    li![a![
                        attrs! {At::Href => Urls::new(url).transaction().list()},
                        "Transactions"
                    ]]
                    */
                ],
                div![
                    C!["navbar-form", "navbar-left"],
                    div![
                        C!["form-group"],
                        input![
                            attrs! {At::Type => "text", At::Placeholder => "Address", At::Id => "address"},
                            C!["form-control"],
                            input_ev(Ev::Input, |value| crate::Msg::AddressChanged(value))
                        ]
                    ],
                    button![
                        //attrs! {At::Type => "submit"},
                        C!["btn", "btn-default"],
                        "Find",
                        ev(Ev::Click, move |_| { Msg::AddressGo }),
                    ]
                ],
                div![
                    C!["navbar-right"],
                    ul![
                        C!["nav", "navbar-nav"],
                        li![a![
                            attrs! {At::Href => "#"},
                            ev(Ev::Click, |_| Msg::ExternalAddress(String::from("/docs"))),
                            "Docs"
                        ]]
                    ]
                ]
            ]
        ]
    ]
}
