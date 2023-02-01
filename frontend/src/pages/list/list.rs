use crate::{model::StoredList, Context, Urls, pages::pagination};
use seed::{prelude::*, *};

#[derive(Default, Debug)]
pub struct Model {
    filter: String,
    pagination: crate::Pagination,
    lists: Vec<crate::model::StoredList>,
    new_list: Option<StoredList>,
}

#[derive(Debug)]
pub enum Msg {
    FilterChanged(String),
    Pagination(usize),
    StoredListNew,
    StoredListNewIdChanged(String),
    StoredListNewDescriptionChanged(String),
    StoredListNewAddressAdd(String),
    StoredListNewAddressDel(String),
    StoredListCreate,
    StoredListCreated(fetch::Result<crate::model::StoredList>),
    StoredListDelete(String),
    StoredListDeleted(fetch::Result<i32>),
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
        Msg::StoredListNew => {
            if model.new_list.is_none() {
                log("Creating list");
                model.new_list = Some(StoredList{id: String::new(), description: String::new(), addresses: Vec::new()});

            } else {
                model.new_list = None;
            }
        }
        Msg::StoredListNewIdChanged(value) => {
            if let Some(list) = &mut model.new_list {
                list.id = value.clone();
            }
        }
        Msg::StoredListCreate => {
            if let Some(list) = &model.new_list {
                let list = list.clone();
                
            }
        }
        Msg::StoredListCreated(Ok(list)) => {

            log("NEW CHAIN CREATED");
            model.new_list = None;
            //ctx.lists.insert(list.id.unwrap(), list);        
        }

        Msg::StoredListCreated(Err(_)) => {
            log("Not created");
        }

        Msg::StoredListDelete(id) => {
            
        }

        Msg::StoredListDeleted(Ok(id)) => {
            //ctx.lists.remove(&id);
        }

        Msg::StoredListDeleted(Err(_)) => {
            log("Error");
        }
        _ => {
            log(msg);
        }
    }
}

pub fn view(model: &Model, ctx: &Context) -> Node<Msg> {
      

    let filtered_lists = model.lists.iter()
                                    .filter(|l| l.id.to_lowercase().contains(&model.filter))
                                    .map(|l| l.clone())
                                    .collect::<Vec<StoredList>>();

    let size = filtered_lists.len();
    let start = if model.pagination.start > size { size } else { model.pagination.start };
    let end = if model.pagination.start + ctx.page_size > size { size } else { model.pagination.start + ctx.page_size};

    let lists: Vec<StoredList> = filtered_lists
                        [start..end]
                        .iter()
                        .map(|c| c.clone()).collect();
    
    div![
        C!["container"],
        h2!["StoredLists"],
        div![
            C!["text-right"],
            span![
                C!["btn", "btn-primary", "right"],
                attrs!{
                    At::Type => "button",
                    At::AriaExpanded => "false"
                },
                "Create list",
                ev(Ev::Click, |_| Msg::StoredListNew),
            ],
        ],
        if let Some(list) = &model.new_list {
            div![
                C!["panel", "panel-default"],                            
                attrs![At::Id => "list-create"],
                div![C!["panel-heading"], h3![C!["panel-title"], "Create list"]],
                div![C!["panel-body"],
                    div![
                        C!["form-group"],
                        label![attrs!{At::For => "list-create-id"}, "#"],
                        span![C!["form-control"], attrs!{At::Id => "list-create-id"}, "Auto"],
                    ],
                    div![
                        C!["form-group"],
                        label![attrs!{At::For => "list-create-title"}, "Id"],
                        input![
                            C!["form-control"], 
                            attrs![At::Id => "list-create-title", At::Value => list.id.clone()],
                            input_ev(Ev::Input, |value| Msg::StoredListNewIdChanged(value)),                        
                        ],
                    ],
                    input![
                        C!["btn", "btn-primary"], 
                        attrs!{At::Type => "submit", At::Value => "Create"},
                        ev(Ev::Click, |_| Msg::StoredListCreate),
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
                    label![attrs!{At::For => "list-filter-text"}, "Filter"],
                    input![
                        C!["form-control"],
                        attrs!{At::Type => "text", At::Id => "list-filter-text"},
                        input_ev(Ev::Input, |value| Msg::FilterChanged(value))
                    ],
                ],
            ],
            table![
                C!["table", "table-striped", "table-hover"],
                thead![tr![th!["#"], th!["Title"], th!["Links"], th![C!["text-right"], "Action"]]],
                tbody![lists
                    .iter()
                    .map(|ch| {
                        let id = ch.id.clone();
                        tr![
                            td![
                                a![
                                    attrs!{At::Href => Urls::new(ctx.base_url.clone()).list().detail(id.clone())},
                                    id.to_string()
                                ],
                                ],
                            td![ch.id.clone()],
                            td![
                                a![
                                    "Addresses",
                                    //attrs!{At::Href => Urls::new(ctx.base_url.clone()).address().list_by_list(id.clone())}
                                ]
                            ],
                            td![
                                C!["text-right"],                                
                                span![
                                    C!["btn", "btn-primary"],
                                    ev(Ev::Click, move |_| Msg::StoredListDelete(id)),
                                    "DELETE"
                                ]
                            ]
                        ]
                    })
                    .collect::<Vec<Node<Msg>>>()]                    
            ],            
        ],
        IF!(size > 0 => pagination::<Msg>(&model.pagination, size, ctx.page_size.clone(), Msg::Pagination))
    ]
}


