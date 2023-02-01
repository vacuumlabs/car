use seed::{prelude::*, *};

const RELATIONS: &str = "relations";
const DIRECTIONS: &str = "directions";

pub mod directions;
pub mod relations;

#[derive(Debug)]
pub enum Page {
    Relations(relations::Model),
    Directions(directions::Model),
}

impl Page {
    pub fn init(mut url: Url, orders: &mut impl Orders<Msg>) -> Option<Page> {
        log("analysis");

        match url.next_path_part() {
            Some(RELATIONS) => match url.next_path_part() {
                Some(slug) => {
                    orders.skip().send_msg(Msg::Relations(relations::Msg::Load));

                    Some(Page::Relations(relations::Model {
                        address: String::from(slug),
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
    Relations(relations::Msg),
    Directions(directions::Msg),
}

pub fn update(msg: Msg, page: &mut Page, ctx: &mut crate::Context, orders: &mut impl Orders<Msg>) {
    match (msg, page) {
        (Msg::Relations(sub_msg), Page::Relations(model)) => {
            relations::update(sub_msg, model, ctx, &mut orders.proxy(Msg::Relations));
        }
        (Msg::Directions(sub_msg), Page::Directions(model)) => {
            directions::update(sub_msg, model, ctx, &mut orders.proxy(Msg::Directions));
        }
        _ => {}
    }
}

struct_urls!();
impl<'a> Urls<'a> {
    pub fn relations(self, address: String) -> Url {
        self.base_url()
            .add_path_part(RELATIONS)
            .add_path_part(address)
    }
    pub fn directions(self, address: String) -> Url {
        self.base_url()
            .add_path_part(DIRECTIONS)
            .add_path_part(address)
    }
}

pub fn view(page: &Page, ctx: &crate::Context) -> Node<Msg> {
    match page {
        Page::Relations(sub) => relations::view(&sub, ctx).map_msg(Msg::Relations),
        Page::Directions(sub) => directions::view(&sub, ctx).map_msg(Msg::Directions),
    }
}
