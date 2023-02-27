use crate::{pages::pagination, Context, Urls};
use seed::{prelude::*, *};

#[derive(Default, Debug)]
pub struct Model {
    filter: String,
    pagination: crate::Pagination,
    new_tag: Option<shared::Tag>,
}

#[derive(Debug)]
pub enum Msg {
    Load,
    Pagination(usize),
    TagsFetched(fetch::Result<Vec<shared::Tag>>),
    FilterChanged(String),
    TagNew,
    TagNewTitleChanged(String),
    TagCreate,
    TagCreated(fetch::Result<shared::Tag>),
    TagDelete(i32),
    TagDeleted(fetch::Result<i32>),
}

pub fn update(msg: Msg, model: &mut Model, ctx: &mut Context, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Pagination(start) => {
            log(format!("Pagination: {}", start));
            model.pagination.start = start;
        }

        Msg::FilterChanged(value) => {
            model.filter = value;
        }
        Msg::TagNew => {
            if model.new_tag.is_none() {
                model.new_tag = Some(shared::Tag {
                    title: String::new(),
                    id: None,
                });
            } else {
                model.new_tag = None;
            }
        }
        Msg::TagNewTitleChanged(value) => {
            if let Some(tag) = &mut model.new_tag {
                tag.title = value.clone();
            }
        }
        Msg::TagCreate => {
            if let Some(tag) = &model.new_tag {
                let tag = tag.clone();
                orders.perform_cmd(async move {
                    Msg::TagCreated(crate::request::tag::create(tag.clone()).await)
                });
            }
        }
        Msg::TagCreated(Ok(tag)) => {
            log("NEW CHAIN CREATED");
            model.new_tag = None;
            ctx.tags.insert(tag.id.unwrap(), tag);
        }

        Msg::TagCreated(Err(_)) => {
            log("Not created");
        }

        Msg::TagDelete(id) => {
            orders.perform_cmd(async move {
                Msg::TagDeleted(crate::request::tag::delete(id.clone()).await)
            });
        }

        Msg::TagDeleted(Ok(id)) => {
            ctx.tags.remove(&id);
        }

        Msg::TagDeleted(Err(_)) => {
            log("Error");
        }
        _ => {
            log(msg);
        }
    }
}

pub fn view(model: &Model, ctx: &Context) -> Node<Msg> {
    let filtered_tags = ctx
        .tags
        .values()
        .into_iter()
        .filter(|s| s.title.to_lowercase().contains(&model.filter))
        .map(|s| s.clone())
        .collect::<Vec<shared::Tag>>();

    let size = filtered_tags.len();
    let start = if model.pagination.start > size {
        size
    } else {
        model.pagination.start
    };
    let end = if model.pagination.start + ctx.page_size > size {
        size
    } else {
        model.pagination.start + ctx.page_size
    };

    let tags: Vec<shared::Tag> = filtered_tags[start..end]
        .iter()
        .map(|c| c.clone())
        .collect();

    div![
        C!["container"],
        h2!["Tags"],
        IF!(ctx.edit =>
        div![
            C!["text-right"],
            span![
                C!["btn", "btn-primary", "right"],
                attrs! {
                    At::Type => "button",
                    At::AriaExpanded => "false"
                },
                "Create tag",
                ev(Ev::Click, |_| Msg::TagNew),
            ],
        ]),
        if let Some(tag) = &model.new_tag {
            div![
                C!["panel", "panel-default"],
                attrs![At::Id => "service-create"],
                div![C!["panel-heading"], h3![C!["panel-title"], "Create TAG"]],
                div![C!["panel-body"],
                    div![
                        attrs![At::Id => "tag-create"],
                        div![
                            C!["form-group"],
                            label![attrs! {At::For => "tag-create-id"}, "#"],
                            span![
                                C!["form-control"],
                                attrs! {At::Id => "tag-create-id"},
                                "Auto"
                            ],
                        ],
                        div![
                            C!["form-group"],
                            label![attrs! {At::For => "tag-create-title"}, "Title"],
                            input![
                                C!["form-control"],
                                attrs![At::Id => "tag-create-title", At::Value => tag.title.clone()],
                                input_ev(Ev::Input, |value| Msg::TagNewTitleChanged(value)),
                            ],
                        ],
                        input![
                            C!["btn", "btn-primary"],
                            attrs! {At::Type => "submit", At::Value => "Create"},
                            ev(Ev::Click, |_| Msg::TagCreate),
                        ],
                    ]
                ]
            ]
        } else {
            div![]
        },
        div![
            C!["card"],
            form![div![
                C!["form-group"],
                label![attrs! {At::For => "tag-filter-text"}, "Filter"],
                input![
                    C!["form-control"],
                    attrs! {At::Type => "text", At::Id => "tag-filter-text"},
                    input_ev(Ev::Input, |value| Msg::FilterChanged(value))
                ],
            ],],
            table![
                C!["table", "table-striped", "table-hover"],
                thead![tr![th!["#"], th!["Title"], th!["Links"], th![C!["text-right"], "Action"]]],
                tbody![
                    tags
                    .iter()
                    .map(|ch| {
                        let id = ch.id.unwrap();
                        tr![
                            td![
                                a![
                                    attrs!{At::Href => Urls::new(ctx.base_url.clone()).tag().detail(id.clone())},
                                    id.to_string()
                                ],
                                ],
                            td![ch.title.clone()],
                            td![
                                a![
                                    "Addresses",
                                    attrs!{At::Href => Urls::new(ctx.base_url.clone()).address().list_by_tag(id.clone())}
                                ]
                            ],
                            td![
                                C!["text-right"],
                                IF!(ctx.edit =>
                                div![
                                    C!["btn", "btn-primary"],
                                    ev(Ev::Click, move |_| Msg::TagDelete(id)),
                                    "DELETE"
                                ])
                            ]
                        ]
                    })
                    .collect::<Vec<Node<Msg>>>()]
            ],
        ],
        IF!(size > 0 => pagination::<Msg>(&model.pagination, size, ctx.page_size.clone(), Msg::Pagination))
    ]
}
