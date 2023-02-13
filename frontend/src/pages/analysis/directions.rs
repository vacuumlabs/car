use crate::{pages::pagination, Context, Urls};
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
}

pub fn update(msg: Msg, model: &mut Model, ctx: &mut Context, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Load => {}
        Msg::Pagination(start) => {
            log(format!("Pagination: {}", start));
            model.pagination.start = start;
        }
    }
}

pub fn view(model: &Model, ctx: &Context) -> Node<Msg> {
    span![]
}
