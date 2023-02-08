use std::collections::HashMap;

use crate::{model::StoredList, Context, LOCAL_STORAGE_KEY};
use seed::{prelude::*, *, futures::task::LocalSpawn};
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct Model {
    pub slug: String,
    pub edit: bool,
    pub list: Option<StoredList>,
    pub saved: Option<bool>,
}

#[derive(Debug)]
pub enum Msg {
    Load,
    EditToggle,
    ListDescriptionChanged(String),
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
            let list_map: HashMap<Uuid, StoredList> = LocalStorage::get(LOCAL_STORAGE_KEY).unwrap_or_default();
            ctx.lists = list_map;

            match model.slug.parse::<Uuid>() {
                    Ok(id) => model.list = ctx.lists.get(&id).cloned(),
                    Err(_) => model.list = None,
                };
        }
        Msg::EditToggle => {
            model.edit = !model.edit;
        }
        Msg::ListDescriptionChanged(value) => {
            if let Some(list) = &mut model.list {
                list.description = value;
            }
        }
        Msg::Save => {
            ctx.lists.insert(model.list.as_ref().unwrap().id, model.list.clone().unwrap());        
            LocalStorage::insert(LOCAL_STORAGE_KEY, &ctx.lists);
        }
        _ => {}
    }
}

pub fn view(model: &Model, ctx: &Context) -> Node<Msg> {
    if let Some(list) = &model.list {
        div![
            C!["container"],
            div![
                C!["text-right"],
                div![
                    C!["btn", "btn-primary", "right"],
                    ev(Ev::Click, |_| Msg::EditToggle),
                    "Edit"
                ],
            ],
            if !model.edit {
                div![
                    div![
                        C!["form-group"],
                        label![attrs! {At::For => "list-create-id"}, "#"],
                        span![
                            attrs! {At::Id => "list-create-id"},
                            list.id.to_string()
                        ],
                    ],
                    div![
                        C!["form-group"],
                        label![attrs! {At::For => "list-create-description"}, "Description"],
                        span![
                            attrs![At::Id => "list-create-description"],
                            list.description.clone()
                        ],
                    ],
                    div![
                        C!["form-group"],
                        label![attrs! {At::For => "list-addresses"}, "Addresses"],
                        ul![
                            attrs! {At::Id => "list-addresses"},
                            list.addresses.iter().map(
                                |it| ul![it]
                                )
                        ]
                    ],

                ]
            } else {
                div![
                    div![
                        C!["form-group"],
                        label![attrs! {At::For => "list-create-id"}, "#"],
                        span![
                            C!["form-control"],
                            attrs! {At::Id => "list-create-id"},
                            list.id.to_string()
                        ],
                    ],
                    div![
                        C!["form-group"],
                        label![attrs! {At::For => "list-create-description"}, "Description"],
                        input![
                            C!["form-control"],
                            attrs! {
                                At::Id => "list-create-description",
                                At::Value => list.description.clone()
                            },
                            input_ev(Ev::Input, |value| Msg::ListDescriptionChanged(value)),
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
    } else {
        div!["Not found"]
    }
}
