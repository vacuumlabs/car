use std::collections::HashMap;

use crate::{Context, Urls, LOCAL_STORAGE_KEY};
use seed::{prelude::*, *};
use shared::{Address, StoredList};
use uuid::Uuid;

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
    pub modal_address: Option<Address>,
    pub show_modal: bool,
    pub selected_list_id: Option<Uuid>,
}

#[derive(Debug)]
pub enum Msg {
    LoadByAddress(String),
    LoadByTag(i32),
    LoadByService(i32),
    LoadByIds(Vec<i64>),
    AddressFetched(fetch::Result<Vec<shared::Address>>),
    AddressNew,
    AddressNewTitleChanged(String),
    AddressCreate,
    AddressCreated(fetch::Result<shared::Address>),
    AddressDelete(i64),
    AddressDeleted(fetch::Result<i64>),
    OpenModal(i64),
    SaveToList(),
    SelectValueChanged(String),
    CloseModal(),
}

pub fn update(msg: Msg, model: &mut Model, ctx: &mut Context, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::LoadByAddress(address) => {
            orders.skip().perform_cmd(async move {
                Msg::AddressFetched(crate::request::address::list_by_address(address).await)
            });
        }
        Msg::LoadByTag(tag) => {
            orders.skip().perform_cmd(async move {
                Msg::AddressFetched(crate::request::address::list_by_tag(tag).await)
            });
        }
        Msg::LoadByService(service) => {
            orders.skip().perform_cmd(async move {
                Msg::AddressFetched(crate::request::address::list_by_service(service).await)
            });
        }
        Msg::AddressFetched(Ok(addresses)) => {
            let list_map: HashMap<Uuid, StoredList> =
                LocalStorage::get(LOCAL_STORAGE_KEY).unwrap_or_default();
            ctx.lists = list_map;

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

        Msg::OpenModal(id) => {
            model.modal_address = model.addresses.iter().find(|&a| a.id == Some(id)).cloned();
            model.show_modal = true;
        }

        Msg::CloseModal() => {
            model.show_modal = false;
            model.modal_address = None;
            model.selected_list_id = None;
        }

        Msg::SelectValueChanged(event) => {
            model.selected_list_id = match event.parse::<Uuid>() {
                Ok(uuid) => Some(uuid),
                Err(_) => None,
            };
        }

        Msg::SaveToList() => {
            if model.selected_list_id.is_none() || model.modal_address.is_none() {
                log("Tried to call save with None!");
            }

            if ctx
                .lists
                .get(&model.selected_list_id.unwrap())
                .unwrap()
                .addresses
                .contains(&model.modal_address.clone().unwrap().id.unwrap())
            {
                orders.send_msg(Msg::CloseModal());
                return;
            }
            ctx.lists
                .get_mut(&model.selected_list_id.unwrap())
                .unwrap()
                .addresses
                .push(model.modal_address.clone().unwrap().id.unwrap());

            LocalStorage::insert(LOCAL_STORAGE_KEY, &ctx.lists);
            orders.send_msg(Msg::CloseModal());
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
        if model.show_modal {
            div![
                C!["modal show"],
                attrs!{
                    At::Id => "listModal",
                    At::from("data-bs-keyboard") => "false",
                    At::from("tabindex") => "-1",
                },
                div![
                    C!["modal-dialog"],
                    div![
                        C!["modal-content"],
                        div![
                            C!["modal-header"],
                            attrs!{
                                At::Id => "listModalLabel",
                            },
                            h1![
                                C!["modal-title fs-5"],
                                "Add to List"
                            ],
                        ],
                    div![
                        C!["modal-body"],
                        select![
                            C!["form-select"],
                            input_ev(Ev::Input, Msg::SelectValueChanged),
                            option![
                                style!{
                                    St::Display => "none"
                                },
                                attrs!(
                                    At::Value => "",
                                    At::Selected => true,
                                    At::Disabled => true,
                                )
                            ],
                            ctx.lists
                                .values()
                                .map(|list|
                                     option![
                                     attrs!(
                                         At::Value => list.id.to_string(),
                                         ),
                                     list.id.to_string()

                                ])
                        ]
                    ],
                    div![
                        C!["modal-footer"],
                        button![
                            C!["btn btn-primary"],
                            ev(Ev::Click, |_| Msg::SaveToList()),
                            "Save"
                        ],
                        button![
                            C!["btn btn-secondary"],
                            ev(Ev::Click, |_| Msg::CloseModal()),
                            "Close"
                        ],
                    ],
                    ]
                ]
            ]
        } else {
            div![]
        },
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
                                ul![
                                    C!["list-group"],
                                    button![
                                        C!["list-group-item list-group-item-action"],
                                        ev(Ev::Click, move |_| Msg::AddressDelete(id)),
                                        "DELETE"
                                    ],
                                    button![
                                        C!["list-group-item list-group-item-action"],
                                        attrs!{
                                            At::from("data-bs-toggle") => "modal",
                                            At::from("data-bs-target") => "#listModal",
                                        },
                                        ev(Ev::Click, move |_| Msg::OpenModal(id)),
                                        "Add to List"
                                    ],
                                ]
                            ],
                        ]
                    })
                    .collect::<Vec<Node<Msg>>>()]
            ],
        ]
    ]
}
