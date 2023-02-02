use crate::{model::Address, Context, Urls};
use seed::{prelude::*, *};

#[derive(Debug)]
pub enum PageType {
    Address(String),
    Tag(i32),
    Service(i32),
    Chain(i32),
}

#[derive(Debug)]
pub struct Model {
    pub page_type: PageType,
    pub filter: String,
    pub addresses: Vec<Address>,
    pub new_address: Option<Address>,
}

#[derive(Debug)]
pub enum Msg {
    LoadByAddress(String),
    LoadByTag(i32),
    LoadByService(i32),
    LoadByIds(Vec<i64>),
    AddresssFetched(fetch::Result<Vec<crate::model::Address>>),
    AddressNew,
    AddressNewTitleChanged(String),
    AddressCreate,
    AddressCreated(fetch::Result<crate::model::Address>),
    AddressDelete(i64),
    AddressDeleted(fetch::Result<i64>),
}

pub fn update(msg: Msg, model: &mut Model, ctx: &mut Context, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::LoadByAddress(address) => {
            orders.skip().perform_cmd(async move {
                Msg::AddresssFetched(crate::request::address::list_by_address(address).await)
            });
        }
        Msg::LoadByTag(tag) => {
            orders.skip().perform_cmd(async move {
                Msg::AddresssFetched(crate::request::address::list_by_tag(tag).await)
            });
        }
        Msg::LoadByService(service) => {
            orders.skip().perform_cmd(async move {
                Msg::AddresssFetched(crate::request::address::list_by_service(service).await)
            });
        }
        Msg::AddresssFetched(Ok(addresses)) => {
            model.addresses = addresses;
        }
        Msg::AddressNew => {
            if model.new_address.is_none() {
                model.new_address = Some(Address {
                    title: Some(String::new()),
                    id: None,
                    hash: String::new(),
                    services: Vec::new(),
                    tags: Vec::new(),
                    chain: 1,
                });
            } else {
                model.new_address = None;
            }
        }
        Msg::AddressNewTitleChanged(value) => {
            if let Some(address) = &mut model.new_address {
                address.title = Some(value.clone());
            }
        }
        Msg::AddressCreate => {
            if let Some(address) = &model.new_address {
                let address = address.clone();
                orders.perform_cmd(async move {
                    Msg::AddressCreated(crate::request::address::create(address.clone()).await)
                });
            }
        }
        Msg::AddressCreated(Ok(address)) => {
            log("NEW CHAIN CREATED");
            model.new_address = None;
            model.addresses.push(address);
        }

        Msg::AddressCreated(Err(_)) => {
            log("Not created");
        }

        Msg::AddressDelete(id) => {
            orders.perform_cmd(async move {
                Msg::AddressDeleted(crate::request::address::delete(id.clone()).await)
            });
        }

        Msg::AddressDeleted(Ok(id)) => {
            model.addresses = model
                .addresses
                .iter()
                .filter(|ch| ch.id.unwrap() != id)
                .map(|ch| ch.clone())
                .collect();
        }

        Msg::AddressDeleted(Err(_)) => {
            log("Error");
        }
        _ => {
            log(msg);
        }
    }
}

pub fn view(model: &Model, ctx: &Context) -> Node<Msg> {
    let addresses = model.addresses.clone();

    div![
        C!["container"],
        div![
            C!["text-right"],
            span![
                C!["btn", "btn-primary", "right"],
                attrs!{
                    At::Type => "button",
                    At::AriaExpanded => "false"
                },
                "Create address",
                ev(Ev::Click, |_| Msg::AddressNew),
            ],
        ],
        if let Some(address) = &model.new_address {
            div![
                C!["border-bottom"],            
                attrs![At::Id => "address-create"],
                div![
                    C!["form-group"],
                    label![attrs!{At::For => "address-create-id"}, "#"],
                    span![C!["form-control"], attrs!{At::Id => "address-create-id"}, "Auto"],
                ],
                div![
                    C!["form-group"],
                    label![attrs!{At::For => "address-create-title"}, "Title"],
                    input![
                        C!["form-control"], 
                        attrs![At::Id => "address-create-title", At::Value => address.title.clone().unwrap_or(address.hash.clone())],
                        input_ev(Ev::Input, |value| Msg::AddressNewTitleChanged(value)),
                    ],
                ],
                input![
                    C!["btn", "btn-primary"], 
                    attrs!{At::Type => "submit", At::Value => "Create"},
                    ev(Ev::Click, |_| Msg::AddressCreate),
                ],
            ]
        } else {
            div![]
        },
        div![
            C!["card"],
            form![
                div![
                    C!["form-group"],
                    label![attrs!{At::For => "address-filter-text"}, "Filter"],
                    input![
                        C!["form-control"],
                        attrs!{At::Type => "text", At::Id => "address-filter-text"},
                        //input_ev(Ev::Input, |value| Msg::FilterChanged(value))
                    ],
                ],
            ],
            table![
                C!["table", "table-striped", "table-hover"],
                thead![
                    tr![
                        th!["#"], 
                        th!["Chain"],
                        th!["Title"],
                        th!["Hash"],
                        th!["Tags"],
                        th!["Services"],
                        th!["Links"],
                        th!["Actions"],
                    ],
                ],
                tbody![addresses
                    .iter()
                    .map(|a| {
                        let id = a.id.unwrap();
                        tr![
                            td![
                                a![
                                    attrs!{At::Href => Urls::new(ctx.base_url.clone()).address().detail(id.clone())},
                                    id.to_string()
                                ],
                            ],
                            td![ctx.chains.get(&a.chain).unwrap().title.clone()],
                            td![a.title.clone().unwrap_or(a.hash.clone())],
                            td![a.hash.clone()],
                            td![crate::pages::tag_badge(ctx, &a.tags)],
                            td![crate::pages::service_badge(ctx, &a.services)],
                            td![
                                ul![
                                    li![a![attrs!{At::Href => crate::Urls::new(ctx.base_url.clone()).analysis().relations(a.hash.clone())}, "Relation"]],
                                    li![a![attrs!{At::Href => crate::Urls::new(ctx.base_url.clone()).analysis().directions(a.hash.clone())}, "Direction"]],
                                ]
                            ],
                            td![
                                C!["btn", "btn-primary"],
                                ev(Ev::Click, move |_| Msg::AddressDelete(id)),
                                "DELETE"
                            ]
                        ]
                    })
                    .collect::<Vec<Node<Msg>>>()]
            ],
        ]
    ]
}
