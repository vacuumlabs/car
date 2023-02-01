use crate::{model::Chain, pages::pagination, Context, Urls};
use seed::{prelude::*, *};

#[derive(Default, Debug)]
pub struct Model {
    pub filter: String,
    pub pagination: crate::Pagination,
    pub address: String,
}

#[derive(Debug)]
pub enum Msg {
    Load,
    Pagination(usize),
    FilterChanged(String),
}

pub fn update(msg: Msg, model: &mut Model, ctx: &mut Context, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Load => {}
        Msg::Pagination(start) => {
            log(format!("Pagination: {}", start));
            model.pagination.start = start;
        }
        Msg::FilterChanged(value) => {
            model.filter = value;
        }
    }
}

pub fn view(model: &Model, ctx: &Context) -> Node<Msg> {
    div!["Relations"]
}
