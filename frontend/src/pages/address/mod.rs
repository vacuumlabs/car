use seed::{prelude::*, *};

const LIST_BY_ADDRESS: &str = "list_by_address";
const LIST_BY_TAG: &str = "list_by_tag";
const LIST_BY_SERVICE: &str = "list_by_service";
const LIST_BY_IDS: &str = "list_by_ids";
const DETAIL: &str = "detail";

pub mod detail;
pub mod list;

#[derive(Debug)]
pub enum Page {
    List(list::Model),
    Detail(detail::Model),
}

impl Page {
    pub fn init(mut url: Url, orders: &mut impl Orders<Msg>) -> Option<Page> {
        match url.next_path_part() {
            Some(LIST_BY_ADDRESS) => match url.next_path_part() {
                Some(address) => {
                    orders
                        .skip()
                        .send_msg(Msg::List(list::Msg::LoadByAddress(address.to_string())));

                    Some(Page::List(list::Model {
                        filter: String::new(),
                        page_type: list::PageType::Address(String::from(address)),
                        addresses: Vec::new(),
                        new_address: None,
                        modal_address: None,
                        show_modal: false,
                        selected_list_id: None,
                    }))
                }
                _ => None,
            },
            Some(LIST_BY_TAG) => match url.next_path_part() {
                Some(tag) => {
                    let tag_id = tag.parse::<i32>().unwrap();
                    orders
                        .skip()
                        .send_msg(Msg::List(list::Msg::LoadByTag(tag_id)));

                    Some(Page::List(list::Model {
                        filter: String::new(),
                        page_type: list::PageType::Tag(tag_id),
                        addresses: Vec::new(),
                        new_address: None,
                        modal_address: None,
                        show_modal: false,
                        selected_list_id: None,
                    }))
                }
                _ => None,
            },
            Some(LIST_BY_SERVICE) => match url.next_path_part() {
                Some(service) => {
                    let service_id = service.parse::<i32>().unwrap();
                    orders
                        .skip()
                        .send_msg(Msg::List(list::Msg::LoadByService(service_id)));

                    Some(Page::List(list::Model {
                        filter: String::new(),
                        page_type: list::PageType::Service(service_id),
                        addresses: Vec::new(),
                        new_address: None,
                        modal_address: None,
                        show_modal: false,
                        selected_list_id: None,
                    }))
                }
                _ => None,
            },
            Some(DETAIL) => match url.next_path_part() {
                Some(slug) => {
                    orders.skip().send_msg(Msg::Detail(detail::Msg::Load));

                    Some(Page::Detail(detail::Model {
                        slug: slug.to_string(),
                        edit: false,
                        ..Default::default()
                    }))
                }
                _ => None,
            },
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum Msg {
    List(list::Msg),
    Detail(detail::Msg),
}

pub fn update(msg: Msg, page: &mut Page, ctx: &mut crate::Context, orders: &mut impl Orders<Msg>) {
    match (msg, page) {
        (Msg::List(sub_msg), Page::List(model)) => {
            list::update(sub_msg, model, ctx, &mut orders.proxy(Msg::List));
        }
        (Msg::Detail(sub_msg), Page::Detail(model)) => {
            detail::update(sub_msg, model, ctx, &mut orders.proxy(Msg::Detail));
        }
        _ => {}
    }
}

struct_urls!();
impl<'a> Urls<'a> {
    pub fn list_by_address(self, address: String) -> Url {
        self.base_url()
            .add_path_part(LIST_BY_ADDRESS)
            .add_path_part(address)
    }
    pub fn list_by_tag(self, tag: i32) -> Url {
        self.base_url()
            .add_path_part(LIST_BY_TAG)
            .add_path_part(tag.to_string())
    }
    pub fn list_by_service(self, service: i32) -> Url {
        self.base_url()
            .add_path_part(LIST_BY_SERVICE)
            .add_path_part(service.to_string())
    }
    pub fn list_by_ids(self, ids: Vec<i32>) -> Url {
        self.base_url()
            .add_path_part(LIST_BY_IDS)
            .add_hash_path_part(
                ids.iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            )
    }
    pub fn detail(self, id: i64) -> Url {
        self.base_url()
            .add_path_part(DETAIL)
            .add_path_part(id.to_string())
    }
}

pub fn view(page: &Page, ctx: &crate::Context) -> Node<Msg> {
    match page {
        Page::List(sub) => list::view(&sub, ctx).map_msg(Msg::List),
        Page::Detail(sub) => detail::view(&sub, ctx).map_msg(Msg::Detail),
    }
}
