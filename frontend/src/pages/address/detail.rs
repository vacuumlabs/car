use crate::{model::Address, Context};
use seed::{prelude::*, *};

#[derive(Default, Debug)]
pub struct Model {
    pub slug: String,
    pub edit: bool,
    pub filter_tag: String,
    pub filter_service: String,
    pub address: Option<Address>,
    pub saved: Option<bool>,
}

#[derive(Debug)]
pub enum Msg {
    Load,
    AddressFetched(fetch::Result<crate::model::Address>),
    TagFilterChanged(String),
    ServiceFilterChanged(String),
    AddressTitleChanged(String),
    AddressServiceAdd(i32),
    AddressServiceRemove(i32),
    AddressTagAdd(i32),
    AddressTagRemove(i32),

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
                    Msg::AddressFetched(crate::request::address::detail(id).await)
                });
            }
        }
        Msg::AddressFetched(Ok(mut address)) => {
            model.edit = false;
            model.address = Some(address);
        }
        Msg::AddressTitleChanged(value) => {
            if let Some(address) = &mut model.address {
                log(&value);
                address.title = Some(value);
                model.saved = Some(false);
            }
        }
        Msg::TagFilterChanged(value) => {
            model.filter_tag = value;
        }
        Msg::ServiceFilterChanged(value) => {
            model.filter_service = value;
        }
        Msg::AddressTagAdd(tag) => {
            if let Some(address) = &mut model.address {
                address.tags.push(tag);
                address.tags = address
                    .tags
                    .iter()
                    .filter(|t| ctx.tags.contains_key(t))
                    .map(|t| t.clone())
                    .collect();
            }
        }
        Msg::AddressTagRemove(tag) => {
            if let Some(address) = &mut model.address {
                address.tags = address
                    .tags
                    .iter()
                    .filter(|t| **t != tag)
                    .map(|t| t.clone())
                    .collect();
            }
        }

        Msg::AddressServiceAdd(service) => {
            if let Some(address) = &mut model.address {
                address.services.push(service);
                address.services = address
                    .services
                    .iter()
                    .filter(|s| ctx.services.contains_key(s))
                    .map(|s| s.clone())
                    .collect();
            }
        }

        Msg::AddressServiceRemove(service) => {
            if let Some(address) = &mut model.address {
                address.services = address
                    .services
                    .iter()
                    .filter(|s| **s != service)
                    .map(|s| s.clone())
                    .collect();
            }
        }
        Msg::Save => {
            if let Some(address) = model.address.clone() {
                orders.perform_cmd(async move {
                    Msg::AddressFetched(crate::request::address::save(address).await)
                });
            }
        }
        _ => {}
    }
}

pub fn view(model: &Model, ctx: &Context) -> Node<Msg> {
    if let Some(address) = &model.address {
        div![
            C!["container"],
            div![
                C!["panel", "panel-default"],
                attrs![At::Id => "address-edit"],
                div![C!["panel-heading"], h3![C!["panel-title"], "Edit address"]],
                div![
                    C!["panel-body"],
                    div![
                        C!["form-group"],
                        label![attrs! {At::For => "address-edit-id"}, "#"],
                        span![
                            C!["form-control"],
                            attrs! {At::Id => "address-edit-id"},
                            address.id.clone().unwrap().to_string()
                        ],
                    ],
                    div![
                        C!["form-group"],
                        label![attrs! {At::For => "address-edit-title"}, "Title"],
                        input![
                            C!["form-control"],
                            attrs![At::Id => "address-create-title", At::Value => address.title.clone().unwrap_or(address.hash.clone())],
                            input_ev(Ev::Input, |value| Msg::AddressTitleChanged(value)),
                        ],
                    ],
                    div![
                        C!["form-group"],
                        label![attrs! {At::For => "address-edit-hash"}, "Hash"],
                        span![
                            C!["form-control"],
                            attrs![At::Id => "address-edit-hash"],
                            address.hash.clone()
                        ],
                    ],
                    div![
                        C!["form-group"],
                        label![attrs! {At::For => "address-edit-tags"}, "Tags"],
                        div![
                            C!["form-control"],
                            address
                                .tags
                                .iter()
                                .filter(|t| ctx.tags.contains_key(t))
                                .map(|t| {
                                    let tag = ctx.tags.get(t).unwrap();
                                    let id = tag.id.clone().unwrap();
                                    span![
                                        C!["badge"],
                                        tag.title.clone(),
                                        ev(Ev::Click, move |_| Msg::AddressTagRemove(id))
                                    ]
                                })
                                .collect::<Vec<Node<Msg>>>()
                        ],
                    ],
                    div![
                        C!["form-group"],
                        label![attrs! {At::For => "address-edit-services"}, "Services"],
                        div![
                            C!["form-control"],
                            address
                                .services
                                .iter()
                                .filter(|s| ctx.services.contains_key(s))
                                .map(|s| {
                                    let service = ctx.services.get(s).unwrap();
                                    let id = service.id.clone().unwrap();
                                    span![
                                        C!["badge"],
                                        service.title.clone(),
                                        ev(Ev::Click, move |_| Msg::AddressServiceRemove(id))
                                    ]
                                })
                                .collect::<Vec<Node<Msg>>>()
                        ],
                    ],
                    input![
                        C!["btn", "btn-primary"],
                        attrs! {At::Type => "submit", At::Value => "Save"},
                        ev(Ev::Click, |_| Msg::Save),
                    ],
                ]
            ],
            div![
                C!["container"],
                h3!["Tags"],
                div![
                    C!["form-group"],
                    label![attrs! {At::For => "address-tag-filter"}, "Tag filter"],
                    input![
                        C!["form-control"],
                        attrs! {At::Value => model.filter_tag.clone()},
                        input_ev(Ev::Input, |value| Msg::TagFilterChanged(value))
                    ],
                    div![div![ctx
                        .tags
                        .values()
                        .into_iter()
                        .filter(|t| model.filter_tag.len() > 0
                            && !address.tags.contains(&t.id.unwrap())
                            && t.title.to_lowercase().contains(&model.filter_tag))
                        .map(|t| {
                            let id = t.id.unwrap();
                            span![
                                C!["badge"],
                                t.title.clone(),
                                ev(Ev::Click, move |_| Msg::AddressTagAdd(id))
                            ]
                        })
                        .collect::<Vec<Node<Msg>>>()]],
                ],
            ],
            div![
                C!["container"],
                h3!["Services"],
                div![
                    C!["form-group"],
                    label![
                        attrs! {At::For => "address-service-filter"},
                        "Service filter"
                    ],
                    input![
                        C!["form-control"],
                        attrs! {At::Value => model.filter_service.clone()},
                        input_ev(Ev::Input, |value| Msg::ServiceFilterChanged(value))
                    ],
                    div![div![ctx
                        .services
                        .values()
                        .into_iter()
                        .filter(|s| model.filter_service.len() > 0
                            && !address.services.contains(&s.id.unwrap())
                            && s.title.to_lowercase().contains(&model.filter_service))
                        .map(|s| {
                            let id = s.id.unwrap();
                            span![
                                C!["badge"],
                                s.title.clone(),
                                ev(Ev::Click, move |_| Msg::AddressServiceAdd(id))
                            ]
                        })
                        .collect::<Vec<Node<Msg>>>()]],
                ],
            ]
        ]
    } else {
        div!["Not found"]
    }
}
