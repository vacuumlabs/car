use crate::Context;
use seed::{prelude::*, *};

#[derive(Default, Debug)]
pub struct Model {
    pub slug: String,
    pub edit: bool,
    pub chain: Option<shared::Chain>,
    pub saved: Option<bool>,
}

#[derive(Debug)]
pub enum Msg {
    Load,
    ChainFetched(fetch::Result<shared::Chain>),
    EditToggle,
    ChainTitleChanged(String),

    Save,
}

pub fn update(
    msg: Msg,
    model: &mut Model,
    ctx: &mut crate::Context,
    orders: &mut impl Orders<Msg>,
) {
    match msg {
        Msg::Load => {
            if let Ok(id) = model.slug.parse::<i32>() {
                orders.perform_cmd(async move {
                    Msg::ChainFetched(crate::request::chain::detail(id).await)
                });
            }
        }
        Msg::ChainFetched(Ok(chain)) => {
            model.edit = false;
            model.chain = Some(chain);
        }
        Msg::EditToggle => {
            model.edit = !model.edit;
        }
        Msg::ChainTitleChanged(value) => {
            if let Some(chain) = &mut model.chain {
                log(&value);
                chain.title = value;
                model.saved = Some(false);
            }
        }
        Msg::Save => {
            if let Some(chain) = model.chain.clone() {
                orders.perform_cmd(async move {
                    Msg::ChainFetched(crate::request::chain::save(chain).await)
                });
            }
        }
        _ => {}
    }
}

pub fn view(model: &Model, ctx: &Context) -> Node<Msg> {
    if let Some(chain) = &model.chain {
        div![
            C!["container"],
            div![
                C!["panel panel-default"],
                div![
                    C!["panel-heading"],
                    div![
                        C!["row"],
                        style! {
                            St::Display => "flex",
                            St::AlignItems => "center",
                        },
                        div![
                            C!["col-xs-11"],
                            h3![
                                C!["heading-title"],
                                style! {
                                    // Bootstrap has a default margin on headers,
                                    // removed to align with the button
                                    St::MarginTop => "0px",
                                    St::MarginBottom => "0px",
                                },
                                "Chain Detail"
                            ],
                        ],
                        IF!(ctx.edit =>
                        div![
                            C!["col-xs-1"],
                            div![
                                C!["btn", "btn-primary"],
                                ev(Ev::Click, |_| Msg::EditToggle),
                                "Edit"
                            ],
                        ])
                    ],
                ],
                div![
                    C!["panel-body"],
                    if !model.edit {
                        form![
                            div![
                                C!["form-group"],
                                label![attrs! {At::For => "chain-create-id"}, "Chain ID"],
                                p![
                                    attrs! {At::Id => "chain-create-id"},
                                    chain.id.unwrap().to_string()
                                ],
                            ],
                            div![
                                C!["form-group"],
                                label![attrs! {At::For => "chain-create-title"}, "Title"],
                                p![
                                    attrs![At::Id => "chain-create-title"],
                                    chain.title.clone() //input_ev(Ev::Input, |value| Msg::ChainNewTitleChanged(value)),
                                ],
                            ],
                        ]
                    } else {
                        div![
                            div![
                                C!["form-group"],
                                label![attrs! {At::For => "chain-create-id"}, "#"],
                                span![
                                    C!["form-control"],
                                    attrs! {At::Id => "chain-create-id"},
                                    chain.id.unwrap().to_string()
                                ],
                            ],
                            div![
                                C!["form-group"],
                                label![attrs! {At::For => "chain-create-title"}, "Title"],
                                input![
                                    C!["form-control"],
                                    attrs! {
                                        At::Id => "chain-create-title",
                                        At::Value => chain.title.clone()
                                    },
                                    input_ev(Ev::Input, |value| Msg::ChainTitleChanged(value)),
                                ],
                            ],
                            button![
                                C!["btn", "btn-primary"],
                                "Save",
                                ev(Ev::Click, |_| Msg::Save),
                            ]
                        ]
                    }
                ]
            ]
        ]
    } else {
        h3![C!["text-center"], "Chain ID not found",]
    }
}
