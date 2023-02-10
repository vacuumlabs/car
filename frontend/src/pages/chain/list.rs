use crate::{Context, Urls, pages::pagination};
use seed::{prelude::*, *};

#[derive(Default, Debug)]
pub struct Model {
    filter: String,
    pagination: crate::Pagination,
    new_chain: Option<shared::Chain>,
}

#[derive(Debug)]
pub enum Msg {
    Load,
    Pagination(usize),
    ChainsFetched(fetch::Result<Vec<shared::Chain>>),
    FilterChanged(String),
    ChainNew,
    ChainNewTitleChanged(String),
    ChainCreate,
    ChainCreated(fetch::Result<shared::Chain>),
    ChainDelete(i32),
    ChainDeleted(fetch::Result<i32>),
}


pub fn update(
    msg: Msg,
    model: &mut Model,
    ctx: &mut Context,
    orders: &mut impl Orders<Msg>,
) {
    match msg {
        Msg::Pagination(start) => {
            log(format!("Pagination: {}", start));
            model.pagination.start = start;
        }
        Msg::FilterChanged(value) => {
            model.filter = value;
        }
        Msg::ChainNew => {
            if model.new_chain.is_none() {
                model.new_chain = Some(shared::Chain{title: String::new(), ..Default::default()});

            } else {
                model.new_chain = None;
            }
        }
        Msg::ChainNewTitleChanged(value) => {
            if let Some(chain) = &mut model.new_chain {
                chain.title = value.clone();
            }
        }
        Msg::ChainCreate => {
            if let Some(chain) = &model.new_chain {
                let chain = chain.clone();
                orders.perform_cmd(
                    async move {
                        Msg::ChainCreated(crate::request::chain::create(chain.clone()).await)
                    }
                );
            }
        }
        Msg::ChainCreated(Ok(chain)) => {

            log("NEW CHAIN CREATED");
            model.new_chain = None;
            ctx.chains.insert(chain.id.unwrap().clone(), chain);
        }

        Msg::ChainCreated(Err(_)) => {
            log("Not created");
        }

        Msg::ChainDelete(id) => {
            orders.perform_cmd(
                async move {
                    Msg::ChainDeleted(crate::request::chain::delete(id.clone()).await)
                }
            );
        }

        Msg::ChainDeleted(Ok(id)) => {
            ctx.chains.remove(&id);
        }

        Msg::ChainDeleted(Err(_)) => {
            log("Error");
        }
        _ => {
            log(msg);
        }
    }
}

pub fn view(model: &Model, ctx: &Context) -> Node<Msg> {
    let filtered_chains = ctx
                                    .chains
                                    .values()
                                    .into_iter()
                                    .filter(|s| s.title.to_lowercase().contains(&model.filter))
                                    .map(|s| s.clone())
                                    .collect::<Vec<shared::Chain>>();

    let size = filtered_chains.len();
    let start = if model.pagination.start > size { size } else { model.pagination.start };
    let end = if model.pagination.start + ctx.page_size > size { size } else { model.pagination.start + ctx.page_size};

    let mut chains: Vec<shared::Chain> = filtered_chains
                        [start..end]
                        .iter()
                        .map(|c| c.clone()).collect();

    // sort pagination
    chains.sort_by_key(|ch| ch.id);

    div![
        C!["container"],
        h2!["Chains"],
        div![
            C!["text-right"],
            span![
                C!["btn", "btn-primary", "right"],
                attrs!{
                    At::Type => "button",
                    At::AriaExpanded => "false"
                },
                "Create chain",
                ev(Ev::Click, |_| Msg::ChainNew),
            ],
        ],
        if let Some(chain) = &model.new_chain {
            div![
                C!["panel", "panel-default"],                            
                attrs![At::Id => "service-create"],
                div![C!["panel-heading"], h3![C!["panel-title"], "Create chain"]],
                div![C!["panel-body"],
                    div![         
                        attrs![At::Id => "chain-create"],
                        div![
                            C!["form-group"],
                            label![attrs!{At::For => "chain-create-id"}, "#"],
                            span![C!["form-control"], attrs!{At::Id => "chain-create-id"}, "Auto"],
                        ],
                        div![
                            C!["form-group"],
                            label![attrs!{At::For => "chain-create-title"}, "Title"],
                            input![
                                C!["form-control"], 
                                attrs![At::Id => "chain-create-title", At::Value => chain.title.clone()],
                                input_ev(Ev::Input, |value| Msg::ChainNewTitleChanged(value)),                        
                            ],
                        ],
                        input![
                            C!["btn", "btn-primary"], 
                            attrs!{At::Type => "submit", At::Value => "Create"},
                            ev(Ev::Click, |_| Msg::ChainCreate),
                        ],
                    ]
                ]
            ]
        } else {
            div![]
        },
        div![
            C!["card"],
            form![
                div![
                    C!["form-group"],
                    label![attrs!{At::For => "chain-filter-text"}, "Filter"],
                    input![
                        C!["form-control"],
                        attrs!{At::Type => "text", At::Id => "chain-filter-text"},
                        input_ev(Ev::Input, |value| Msg::FilterChanged(value))
                    ],
                ],
            ],
            table![
                C!["table", "table-striped", "table-hover"],
                thead![tr![th!["#"], th!["Title"], th![C!["text-right"],"Action"]],],
                tbody![chains
                    .iter()
                    .map(|ch| {
                        let id = ch.id.unwrap();
                        tr![
                            td![
                                a![
                                    attrs!{At::Href => Urls::new(ctx.base_url.clone()).chain().detail(id.clone())},
                                    id.to_string()
                                ],
                                ],
                            td![ch.title.clone()],
                            td![
                                C!["text-right"],
                                span![
                                    C!["btn", "btn-primary"],
                                    ev(Ev::Click, move |_| Msg::ChainDelete(id)),
                                    "DELETE"
                                ],
                            ],
                        ]
                    })
                    .collect::<Vec<Node<Msg>>>()]
            ],
        ],
        IF!(size > 0 => pagination::<Msg>(&model.pagination, size, ctx.page_size.clone(), Msg::Pagination))
    ]
}
