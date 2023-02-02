use crate::{
    model::{AddressRef, AddressRelation},
    pages::pagination,
    Context, Pagination, Urls,
};
use seed::{prelude::*, *};
use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Debug)]
pub struct Model {
    pub filter: String,
    pub pagination: crate::Pagination,
    pub address: String,
    pub relations: AddressRelation,
    pub show_empty: bool,
    pub tags: Vec<(i32, usize)>,
    pub services: Vec<(i32, usize)>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            filter: String::new(),
            pagination: Pagination::default(),
            address: String::new(),
            relations: AddressRelation::default(),
            show_empty: false,
            tags: Vec::new(),
            services: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum Msg {
    Load,
    FilterChanged(String),
    RelationsFetched(fetch::Result<AddressRelation>),
    EmptyChange,
}

pub fn update(msg: Msg, model: &mut Model, ctx: &mut Context, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Load => {
            let address = model.address.clone();
            orders.perform_cmd(async move {
                Msg::RelationsFetched(crate::request::analysis::relations(address).await)
            });
        }
        Msg::EmptyChange => {
            model.show_empty = !model.show_empty;
        }
        Msg::RelationsFetched(Ok(relations)) => {
            model.relations = relations;
            model
                .relations
                .inputs
                .sort_by(|a, b| b.quantity.cmp(&a.quantity));
            model
                .relations
                .outputs
                .sort_by(|a, b| b.quantity.cmp(&a.quantity));
            model
                .relations
                .mixed_in
                .sort_by(|a, b| b.quantity.cmp(&a.quantity));
            model
                .relations
                .mixed_out
                .sort_by(|a, b| b.quantity.cmp(&a.quantity));

            let mut tag_quantity: HashMap<i32, usize> = HashMap::new();
            let mut service_quantity: HashMap<i32, usize> = HashMap::new();

            let tag_closure = |sum: &mut HashMap<i32, usize>, source: &Vec<AddressRef>| {
                for reference in source.iter() {
                    for id in &reference.tags {
                        if let Some(t) = sum.get_mut(id) {
                            *t += reference.quantity as usize;
                        } else {
                            sum.insert(id.clone(), reference.quantity.clone() as usize);
                        }
                    }
                }
            };

            let service_closure = |sum: &mut HashMap<i32, usize>, source: &Vec<AddressRef>| {
                for reference in source.iter() {
                    for id in &reference.services {
                        if let Some(t) = sum.get_mut(id) {
                            *t += reference.quantity as usize;
                        } else {
                            sum.insert(id.clone(), reference.quantity.clone() as usize);
                        }
                    }
                }
            };

            tag_closure(&mut tag_quantity, &model.relations.inputs);
            tag_closure(&mut tag_quantity, &model.relations.outputs);
            tag_closure(&mut tag_quantity, &model.relations.mixed_in);
            tag_closure(&mut tag_quantity, &model.relations.mixed_out);

            service_closure(&mut service_quantity, &model.relations.inputs);
            service_closure(&mut service_quantity, &model.relations.outputs);
            service_closure(&mut service_quantity, &model.relations.mixed_in);
            service_closure(&mut service_quantity, &model.relations.mixed_out);

            let mut tag_vec = Vec::from_iter(tag_quantity.iter());
            let mut service_vec = Vec::from_iter(service_quantity.iter());
            tag_vec.sort_by(|a, b| b.1.cmp(&a.1));
            service_vec.sort_by(|a, b| b.1.cmp(&a.1));

            model.tags = tag_vec
                .iter()
                .filter(|t| ctx.tags.contains_key(t.0))
                .take(10)
                .map(|item| (item.0.clone(), item.1.clone()))
                .collect();
            model.services = service_vec
                .iter()
                .filter(|s| ctx.services.contains_key(s.0))
                .take(10)
                .map(|item| (item.0.clone(), item.1.clone()))
                .collect();
        }
        Msg::FilterChanged(value) => {
            model.filter = value;
        }

        _ => {}
    }
}

pub fn view_relation(model: &Model, ctx: &Context, addresses: &Vec<AddressRef>) -> Vec<Node<Msg>> {
    addresses
        .iter()
        .filter(|i| {
            if model.show_empty {
                true
            } else {
                (i.tags.len() + i.services.len()) > 0
            }
        })
        .map(|i| {
            li![
                C!["list-group-item"],
                span![format!("{}X ", i.quantity), style! {St::Color => "red"}],
                a![
                    attrs!{At::Href => crate::Urls::new(ctx.base_url.clone()).address().detail(i.id.clone())},
                    format!("{}...{}", i.hex[0..3].to_string(), i.hex[i.hex.len()-3..].to_string())
                ],
                div![crate::pages::tag_badge(ctx, &i.tags)],
                div![crate::pages::service_badge(ctx, &i.services)],
            ]
        })
        .collect::<Vec<Node<Msg>>>()
}

pub fn view(model: &Model, ctx: &Context) -> Node<Msg> {
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
                            "Relations of ",
                            model.address.clone()
                        ],
                    ]
                ],
            ],
            div![
                C!["panel-body"],
                div![
                    C!["form-group"],
                    label![attrs! {At::For => "relations-hex"}, "HEX"],
                    p![
                        attrs! {At::Id => "relations-hex"},
                        a![                            
                            model.relations.hex.clone(),
                            attrs!{At::Href => crate::Urls::new(ctx.base_url.clone()).address().list_by_address(model.relations.hex.clone())}
                        ]
                    ],
                ],
                div![
                    C!["form-group"],
                    label![attrs! {At::For => "relations-human"}, "Human"],
                    p![
                        attrs! {At::Id => "relations-human"},
                        model.relations.human.clone()
                    ],
                ],
                div![
                    C!["form-group"],
                    label![attrs! {At::For => "relations-tags"}, "Tags"],
                    p![
                        attrs! {At::Id => "relations-tags"},
                        model
                            .relations
                            .tags
                            .iter()
                            .filter(|t| ctx.tags.contains_key(t))
                            .map(|t| span![C!["badge"], ctx.tags.get(t).unwrap().title.clone()])
                            .collect::<Vec<Node<Msg>>>()
                    ],
                ],
                div![
                    C!["form-group"],
                    label![attrs! {At::For => "relations-services"}, "Services"],
                    p![
                        attrs! {At::Id => "relations-services"},
                        model
                            .relations
                            .services
                            .iter()
                            .filter(|t| ctx.services.contains_key(t))
                            .map(|t| span![C!["badge"], ctx.services.get(t).unwrap().title.clone()])
                            .collect::<Vec<Node<Msg>>>()
                    ],
                ],
                span![
                    "Empty: ",
                    input![
                        attrs! {At::Type => "checkbox", At::Checked => model.show_empty.as_at_value()}
                    ],
                    ev(Ev::Click, |_| Msg::EmptyChange),
                ]
            ]
        ],
        div![
            C!["col-md-6"],
            div![
                C!["panel panel-default"],
                div![C!["panel-heading"], h3![C!["panel-title"], "Top TAGs"]],
                div![
                    C!["panel-body"],
                    ul![
                        C!["list-group"],
                        model.tags.iter().map(|t| li![
                            C!["list-group-item"],
                            span![style! {St::Color => "red"}, format!("{}x ", t.1)],
                            a![
                                C!["badge"],
                                attrs!{At::Href => crate::Urls::new(ctx.base_url.clone()).tag().detail(t.0.clone())},
                                ctx.tags[&t.0].title.clone()
                            ]
                        ])
                    ]
                ]
            ],
        ],
        div![
            C!["col-md-6"],
            div![
                C!["panel panel-default"],
                div![C!["panel-heading"], h3![C!["panel-title"], "Top Services"]],
                div![
                    C!["panel-body"],
                    ul![
                        C!["list-group"],
                        model.services.iter().map(|t| li![
                            C!["list-group-item"],
                            span![style! {St::Color => "red"}, format!("{}x ", t.1)],
                            a![
                                C!["badge"],
                                attrs!{At::Href => crate::Urls::new(ctx.base_url.clone()).service().detail(t.0.clone())},
                                ctx.services[&t.0].title.clone()
                            ]
                        ])
                    ]
                ]
            ],
        ],
        div![
            C!["col-md-6"],
            div![
                C!["panel panel-default"],
                div![C!["panel-heading"], h3![C!["panel-title"], "Inputs"]],
                div![
                    C!["panel-body"],
                    style! {St::Height => "300px", St::OverflowY => "scroll"},
                    ul![
                        C!["list-group"],
                        view_relation(model, ctx, &model.relations.inputs)
                    ]
                ]
            ],
        ],
        div![
            C!["col-md-6"],
            div![
                C!["panel panel-default"],
                div![C!["panel-heading"], h3![C!["panel-title"], "Outputs"]],
                div![
                    C!["panel-body"],
                    style! {St::Height => "300px", St::OverflowY => "scroll"},
                    view_relation(model, ctx, &model.relations.outputs),
                ]
            ]
        ],
        div![
            C!["col-md-6"],
            div![
                C!["panel panel-default"],
                div![C!["panel-heading"], h3![C!["panel-title"], "Mixed IN"]],
                div![
                    C!["panel-body"],
                    style! {St::Height => "300px", St::OverflowY => "scroll"},
                    ul![
                        C!["list-group"],
                        view_relation(model, ctx, &model.relations.mixed_in)
                    ]
                ]
            ],
        ],
        div![
            C!["col-md-6"],
            div![
                C!["panel panel-default"],
                div![C!["panel-heading"], h3![C!["panel-title"], "Mixed OUT"]],
                div![
                    C!["panel-body"],
                    style! {St::Height => "300px", St::OverflowY => "scroll"},
                    view_relation(model, ctx, &model.relations.mixed_out),
                ]
            ]
        ]
    ]
}
