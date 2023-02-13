use crate::Context;
use seed::{prelude::*, *};

#[derive(Default, Debug)]
pub struct Model {
    pub slug: String,
    pub edit: bool,
    pub service: Option<shared::Service>,
    pub saved: Option<bool>,
}

#[derive(Debug)]
pub enum Msg {
    Load,
    ServiceFetched(fetch::Result<shared::Service>),
    EditToggle,
    ServiceTitleChanged(String),

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
                    Msg::ServiceFetched(crate::request::service::detail(id).await)
                });
            }
        }
        Msg::ServiceFetched(Ok(service)) => {
            model.edit = false;
            model.service = Some(service);
        }
        Msg::EditToggle => {
            model.edit = !model.edit;
        }
        Msg::ServiceTitleChanged(value) => {
            if let Some(service) = &mut model.service {
                log(&value);
                service.title = value;
                model.saved = Some(false);
            }
        }
        Msg::Save => {
            if let Some(service) = model.service.clone() {
                orders.perform_cmd(async move {
                    Msg::ServiceFetched(crate::request::service::save(service).await)
                });
            }
        }
        _ => {}
    }
}

pub fn view(model: &Model, ctx: &Context) -> Node<Msg> {
    if let Some(service) = &model.service {
        div![
            C!["container"],
            IF!(ctx.edit =>
            div![
                C!["text-right"],
                div![
                    C!["btn", "btn-primary", "right"],
                    ev(Ev::Click, |_| Msg::EditToggle),
                    "Edit"
                ],
            ]),
            if !model.edit {
                div![
                    div![
                        C!["form-group"],
                        label![attrs! {At::For => "service-create-id"}, "#"],
                        span![
                            attrs! {At::Id => "service-create-id"},
                            service.id.unwrap().to_string()
                        ],
                    ],
                    div![
                        C!["form-group"],
                        label![attrs! {At::For => "service-create-title"}, "Title"],
                        span![
                            attrs![At::Id => "service-create-title"],
                            service.title.clone() //input_ev(Ev::Input, |value| Msg::ServiceNewTitleChanged(value)),
                        ],
                    ],
                ]
            } else {
                div![
                    div![
                        C!["form-group"],
                        label![attrs! {At::For => "service-create-id"}, "#"],
                        span![
                            C!["form-control"],
                            attrs! {At::Id => "service-create-id"},
                            service.id.unwrap().to_string()
                        ],
                    ],
                    div![
                        C!["form-group"],
                        label![attrs! {At::For => "service-create-title"}, "Title"],
                        input![
                            C!["form-control"],
                            attrs! {
                                At::Id => "service-create-title",
                                At::Value => service.title.clone()
                            },
                            input_ev(Ev::Input, |value| Msg::ServiceTitleChanged(value)),
                        ],
                    ],
                    button![
                        C!["btn", "btn-primary"],
                        "Save",
                        ev(Ev::Click, |_| Msg::Save),
                    ]
                ]
            },
            div![ul![
                C!["list-group"],
                li![
                    C!["list-group-item"],
                    a![
                        attrs! {At::Href => crate::Urls::new(ctx.base_url.clone()).address().list_by_service(service.id.unwrap().clone())},
                        "Addresses"
                    ]
                ],
            ]]
        ]
    } else {
        div!["Not found"]
    }
}
