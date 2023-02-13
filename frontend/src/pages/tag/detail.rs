use crate::Context;
use seed::{prelude::*, *};

#[derive(Default, Debug)]
pub struct Model {
    pub slug: String,
    pub edit: bool,
    pub tag: Option<shared::Tag>,
    pub saved: Option<bool>,
}

#[derive(Debug)]
pub enum Msg {
    Load,
    TagFetched(fetch::Result<shared::Tag>),
    EditToggle,
    TagTitleChanged(String),

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
                    Msg::TagFetched(crate::request::tag::detail(id).await)
                });
            }
        }
        Msg::TagFetched(Ok(tag)) => {
            model.edit = false;
            model.tag = Some(tag);
        }
        Msg::EditToggle => {
            model.edit = !model.edit;
        }
        Msg::TagTitleChanged(value) => {
            if let Some(tag) = &mut model.tag {
                log(&value);
                tag.title = value;
                model.saved = Some(false);
            }
        }
        Msg::Save => {
            if let Some(tag) = model.tag.clone() {
                orders.perform_cmd(
                    async move { Msg::TagFetched(crate::request::tag::save(tag).await) },
                );
            }
        }
        _ => {}
    }
}

pub fn view(model: &Model, ctx: &Context) -> Node<Msg> {
    if let Some(tag) = &model.tag {
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
                        label![attrs! {At::For => "tag-create-id"}, "#"],
                        span![
                            attrs! {At::Id => "tag-create-id"},
                            tag.id.unwrap().to_string()
                        ],
                    ],
                    div![
                        C!["form-group"],
                        label![attrs! {At::For => "tag-create-title"}, "Title"],
                        span![
                            attrs![At::Id => "tag-create-title"],
                            tag.title.clone() //input_ev(Ev::Input, |value| Msg::TagNewTitleChanged(value)),
                        ],
                    ],
                ]
            } else {
                div![
                    div![
                        C!["form-group"],
                        label![attrs! {At::For => "tag-create-id"}, "#"],
                        span![
                            C!["form-control"],
                            attrs! {At::Id => "tag-create-id"},
                            tag.id.unwrap().to_string()
                        ],
                    ],
                    div![
                        C!["form-group"],
                        label![attrs! {At::For => "tag-create-title"}, "Title"],
                        input![
                            C!["form-control"],
                            attrs! {
                                At::Id => "tag-create-title",
                                At::Value => tag.title.clone()
                            },
                            input_ev(Ev::Input, |value| Msg::TagTitleChanged(value)),
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
