use crate::{Context, Urls, pages::pagination};
use seed::{prelude::*, *};

#[derive(Default, Debug)]
pub struct Model {
    filter: String,
    pagination: crate::Pagination,
    new_service: Option<shared::Service>,
}

#[derive(Debug)]
pub enum Msg {
    FilterChanged(String),
    Pagination(usize),
    ServiceNew,
    ServiceNewTitleChanged(String),
    ServiceCreate,
    ServiceCreated(fetch::Result<shared::Service>),
    ServiceDelete(i32),
    ServiceDeleted(fetch::Result<i32>),
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
            model.pagination.start = 0;
            model.filter = value;
        }
        Msg::ServiceNew => {
            if model.new_service.is_none() {
                model.new_service = Some(shared::Service{title: String::new(), id: None});

            } else {
                model.new_service = None;
            }
        }
        Msg::ServiceNewTitleChanged(value) => {
            if let Some(service) = &mut model.new_service {
                service.title = value.clone();
            }
        }
        Msg::ServiceCreate => {
            if let Some(service) = &model.new_service {
                let service = service.clone();
                orders.perform_cmd(
                    async move {
                        Msg::ServiceCreated(crate::request::service::create(service.clone()).await)
                    }
                );
            }
        }
        Msg::ServiceCreated(Ok(service)) => {

            log("NEW CHAIN CREATED");
            model.new_service = None;
            ctx.services.insert(service.id.unwrap(), service);        
        }

        Msg::ServiceCreated(Err(_)) => {
            log("Not created");
        }

        Msg::ServiceDelete(id) => {
            orders.perform_cmd(
                async move {
                    Msg::ServiceDeleted(crate::request::service::delete(id.clone()).await)
                }
            );
        }

        Msg::ServiceDeleted(Ok(id)) => {
            ctx.services.remove(&id);
        }

        Msg::ServiceDeleted(Err(_)) => {
            log("Error");
        }
        _ => {
            log(msg);
        }
    }
}

pub fn view(model: &Model, ctx: &Context) -> Node<Msg> {
      

    let filtered_services = ctx
                                    .services
                                    .values()
                                    .filter(|s| s.title.to_lowercase().contains(&model.filter));
                                    

    let size = filtered_services.clone().count();

    let services: Vec<shared::Service> = filtered_services.skip(model.pagination.start).take(ctx.page_size)
                        .map(|c| c.clone()).collect();
    
    div![
        C!["container"],
        h2!["Services"],
        IF!(ctx.edit =>
        div![
            C!["text-right"],
            span![
                C!["btn", "btn-primary", "right"],
                attrs!{
                    At::Type => "button",
                    At::AriaExpanded => "false"
                },
                "Create service",
                ev(Ev::Click, |_| Msg::ServiceNew),
            ],
        ]),
        if let Some(service) = &model.new_service {
            div![
                C!["panel", "panel-default"],                            
                attrs![At::Id => "service-create"],
                div![C!["panel-heading"], h3![C!["panel-title"], "Create service"]],
                div![C!["panel-body"],
                    div![
                        C!["form-group"],
                        label![attrs!{At::For => "service-create-id"}, "#"],
                        span![C!["form-control"], attrs!{At::Id => "service-create-id"}, "Auto"],
                    ],
                    div![
                        C!["form-group"],
                        label![attrs!{At::For => "service-create-title"}, "Title"],
                        input![
                            C!["form-control"], 
                            attrs![At::Id => "service-create-title", At::Value => service.title.clone()],
                            input_ev(Ev::Input, |value| Msg::ServiceNewTitleChanged(value)),                        
                        ],
                    ],
                    input![
                        C!["btn", "btn-primary"], 
                        attrs!{At::Type => "submit", At::Value => "Create"},
                        ev(Ev::Click, |_| Msg::ServiceCreate),
                    ],
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
                    label![attrs!{At::For => "service-filter-text"}, "Filter"],
                    input![
                        C!["form-control"],
                        attrs!{At::Type => "text", At::Id => "service-filter-text"},
                        input_ev(Ev::Input, |value| Msg::FilterChanged(value))
                    ],
                ],
            ],
            table![
                C!["table", "table-striped", "table-hover"],
                thead![tr![th!["#"], th!["Title"], th!["Links"], th![C!["text-right"], "Action"]]],
                tbody![services
                    .iter()
                    .map(|ch| {
                        let id = ch.id.unwrap();
                        tr![
                            td![
                                a![
                                    attrs!{At::Href => Urls::new(ctx.base_url.clone()).service().detail(id.clone())},
                                    id.to_string()
                                ],
                                ],
                            td![ch.title.clone()],
                            td![
                                a![
                                    "Addresses",
                                    attrs!{At::Href => Urls::new(ctx.base_url.clone()).address().list_by_service(id.clone())}
                                ]
                            ],
                            
                            td![
                                C!["text-right"],                                
                                IF!(ctx.edit => 
                                div![
                                    C!["btn", "btn-primary"],
                                    ev(Ev::Click, move |_| Msg::ServiceDelete(id)),
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


