use crate::{model::StoredList, Context, Urls, pages::pagination, LOCAL_STORAGE_KEY};
use seed::{prelude::*, *};
use uuid::Uuid;

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
    StoredListNewDescriptionChanged(String),
    StoredListNewAddressAdd(String),
    StoredListNewAddressDel(String),
    StoredListCreate,
    StoredListCreated(fetch::Result<crate::model::StoredList>),
    StoredListDelete(Uuid),
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
                model.new_list = Some(StoredList{id: Uuid::new_v4(), description: String::new(), addresses: Vec::new()});
            } else {
                model.new_list = None;
            }
        }
        Msg::StoredListNewDescriptionChanged(value) => {
            if let Some(list) = &mut model.new_list {
                list.description = value.clone();
            }
        }
        Msg::StoredListCreate => {
            if model.new_list.is_none() { return; }

            ctx.lists.insert(model.new_list.as_ref().unwrap().id, model.new_list.clone().unwrap());        
            LocalStorage::insert(LOCAL_STORAGE_KEY, &ctx.lists);

            // Reset the creation field
            model.new_list = None;
            orders.send_msg(Msg::StoredListNew);
        }
        Msg::StoredListCreated(Ok(list)) => {

            log("NEW LIST CREATED");
            model.new_list = None;
            //ctx.lists.insert(list.id.unwrap(), list);        
        }

        Msg::StoredListCreated(Err(_)) => {
            log("Not created");
        }

        Msg::StoredListDelete(id) => {
            ctx.lists.remove(&id);
            LocalStorage::insert(LOCAL_STORAGE_KEY, &ctx.lists);
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
    let filtered_lists = ctx.lists.values()
                                    .filter(|l| Uuid::to_string(&l.id).to_lowercase().contains(&model.filter) || l.description.to_lowercase().contains(&model.filter))
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
        h2!["Stored Lists"],
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
                        label![attrs!{At::For => "list-create-id"}, "Id"],
                        span![C!["form-control"], attrs!{At::Id => "list-create-id"}, "Auto"],
                    ],
                    div![
                        C!["form-group"],
                        label![attrs!{At::For => "list-create-description"}, "Description"],
                        input![
                            C!["form-control"], 
                            attrs![At::Id => "list-create-description", At::Value => list.description.clone()],
                            input_ev(Ev::Input, |value| Msg::StoredListNewDescriptionChanged(value)),                        
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
                thead![tr![th!["#"], th!["Description"], th!["Links"], th![C!["text-right"], "Action"]]],
                tbody![lists
                    .iter()
                    .map(|ch| {
                        let id = ch.id.clone();
                        tr![
                            td![
                                a![
                                    attrs!{At::Href => Urls::new(ctx.base_url.clone()).list().detail(Uuid::to_string(&id.clone()))},
                                    id.to_string()
                                ],
                                ],
                            td![ch.description.clone()],
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


