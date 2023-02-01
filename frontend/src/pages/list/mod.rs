use seed::{prelude::*, *};

pub mod detail;
pub mod list;

const LIST: &str = "list";
const DETAIL: &str = "detail";

#[derive(Debug)]
pub enum Page {
    List(list::Model),
    Detail(detail::Model),
}

impl Page {
    pub fn init(mut url: Url, orders: &mut impl Orders<Msg>) -> Option<Page> {
        match url.next_path_part() {
            Some(LIST) => Some(Page::List(list::Model::default())),
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
    pub fn default(self) -> Url {
        self.list()
    }

    pub fn list(self) -> Url {
        self.base_url().add_path_part(LIST)
    }
    pub fn detail(self, id: String) -> Url {
        self.base_url().add_path_part(DETAIL).add_path_part(id)
    }
}

pub fn view(page: &Page, ctx: &crate::Context) -> Node<Msg> {
    match page {
        Page::List(sub) => list::view(&sub, ctx).map_msg(Msg::List),
        Page::Detail(sub) => detail::view(&sub, ctx).map_msg(Msg::Detail),
    }
}
