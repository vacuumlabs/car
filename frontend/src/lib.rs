#![allow(clippy::use_self)]

use seed::{
    prelude::{subs::url_requested::UrlRequest, *},
    *,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

mod model;
mod pages;
mod request;

const LOCAL_STORAGE_KEY: &str = "crypto-address-relation-storage";

const CHAIN: &str = "chain";
const TAG: &str = "tag";
const SERVICE: &str = "service";
const ADDRESS: &str = "address";
const TASK: &str = "task";
const LIST: &str = "list";
const ANALYSIS: &str = "analysis";

#[derive(Debug)]
pub struct Context {
    pub base_url: Url,
    pub page_size: usize,
    pub chains: HashMap<i32, model::Chain>,
    pub tags: HashMap<i32, model::Tag>,
    pub services: HashMap<i32, model::Service>,
    pub lists: HashMap<Uuid, model::StoredList>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            page_size: 10,
            base_url: Url::default(),
            chains: HashMap::new(),
            tags: HashMap::new(),
            services: HashMap::new(),
            lists: LocalStorage::get(LOCAL_STORAGE_KEY).unwrap_or_default()
        }
    }
}

#[derive(Default, Debug)]
pub enum Page {
    #[default]
    Home,
    Chain(pages::chain::Page),
    Tag(pages::tag::Page),
    Service(pages::service::Page),
    Address(pages::address::Page),
    List(pages::list::Page),
    Analysis(pages::analysis::Page),
    NotFound,
}

impl Page {
    fn init(mut url: Url, orders: &mut impl Orders<Msg>) -> Self {
        match url.next_path_part() {
            None => Self::Home,
            Some(CHAIN) => pages::chain::Page::init(url, &mut orders.proxy(Msg::Chain))
                .map_or(Self::NotFound, Self::Chain),

            Some(TAG) => pages::tag::Page::init(url, &mut orders.proxy(Msg::Tag))
                .map_or(Self::NotFound, Self::Tag),

            Some(SERVICE) => pages::service::Page::init(url, &mut orders.proxy(Msg::Service))
                .map_or(Self::NotFound, Self::Service),

            Some(ADDRESS) => pages::address::Page::init(url, &mut orders.proxy(Msg::Address))
                .map_or(Self::NotFound, Self::Address),

            Some(LIST) => pages::list::Page::init(url, &mut orders.proxy(Msg::List))
                .map_or(Self::NotFound, Self::List),

            Some(ANALYSIS) => pages::analysis::Page::init(url, &mut orders.proxy(Msg::Analysis))
                .map_or(Self::NotFound, Self::Analysis),

            Some(_) => Self::NotFound,
        }
    }
}

#[derive(Default, Debug)]
pub struct Model {
    pub ctx: Context,
    pub address: String,
    pub page: Page,
}

#[derive(Default, Debug)]
pub struct Pagination {
    start: usize,
    count: usize,
}

struct_urls!();
impl<'a> Urls<'a> {
    pub fn home(self) -> Url {
        self.base_url()
    }
    pub fn chain(self) -> pages::chain::Urls<'a> {
        pages::chain::Urls::new(self.base_url().add_path_part(CHAIN))
    }
    pub fn tag(self) -> pages::tag::Urls<'a> {
        pages::tag::Urls::new(self.base_url().add_path_part(TAG))
    }
    pub fn service(self) -> pages::service::Urls<'a> {
        pages::service::Urls::new(self.base_url().add_path_part(SERVICE))
    }
    pub fn address(self) -> pages::address::Urls<'a> {
        pages::address::Urls::new(self.base_url().add_path_part(ADDRESS))
    }
    pub fn list(self) -> pages::list::Urls<'a> {
        pages::list::Urls::new(self.base_url().add_path_part(LIST))
    }
    pub fn analysis(self) -> pages::analysis::Urls<'a> {
        pages::analysis::Urls::new(self.base_url().add_path_part(ANALYSIS))
    }

    pub fn task(self) -> pages::address::Urls<'a> {
        pages::address::Urls::new(self.base_url().add_path_part(TASK))
    }

    pub fn docs(self) -> Url {
        Url::new()
    }
}

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    log("Run init");
    orders.subscribe(Msg::UrlChanged);
    orders.send_msg(Msg::LoadContext);
    Model {
        ctx: Context {
            base_url: url.to_base_url(),
            ..Default::default()
        },
        address: String::new(),
        page: Page::init(url, orders),
    }
}

#[derive(Debug)]
pub enum Msg {
    UrlChanged(subs::UrlChanged),
    AddressChanged(String),
    AddressGo,
    ExternalAddress(String),
    ChainsFetched(fetch::Result<Vec<model::Chain>>),
    TagsFetched(fetch::Result<Vec<model::Tag>>),
    ServicesFetched(fetch::Result<Vec<model::Service>>),

    LoadContext,
    Chain(pages::chain::Msg),
    Tag(pages::tag::Msg),
    Service(pages::service::Msg),
    Address(pages::address::Msg),
    List(pages::list::Msg),
    Analysis(pages::analysis::Msg),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            log(url.path());
            model.page = Page::init(url, orders);
        }

        Msg::LoadContext => {
            orders.perform_cmd(async { Msg::ChainsFetched(request::chain::list().await) });
            orders.perform_cmd(async { Msg::ServicesFetched(request::service::list().await) });
            orders.perform_cmd(async { Msg::TagsFetched(request::tag::list().await) });
        }

        Msg::AddressChanged(value) => {
            model.address = value;
            orders.skip();
        }

        Msg::AddressGo => {
            Urls::new(model.ctx.base_url.clone())
                .address()
                .list_by_address(model.address.clone())
                .go_and_load();
        }

        Msg::ExternalAddress(value) => Url::go_and_load_with_str(value),

        Msg::ChainsFetched(Ok(chains)) => {
            model.ctx.chains = chains
                .iter()
                .map(|ch| (ch.id.unwrap(), ch.clone()))
                .collect();
            log(model.ctx.chains.clone());
        }

        Msg::ServicesFetched(Ok(services)) => {
            model.ctx.services = services
                .iter()
                .map(|s| (s.id.unwrap(), s.clone()))
                .collect();
        }

        Msg::TagsFetched(Ok(tags)) => {
            model.ctx.tags = tags.iter().map(|t| (t.id.unwrap(), t.clone())).collect();
        }

        Msg::Chain(sub_msg) => {
            if let Page::Chain(sub_page) = &mut model.page {
                pages::chain::update(
                    sub_msg,
                    sub_page,
                    &mut model.ctx,
                    &mut orders.proxy(Msg::Chain),
                )
            }
        }

        Msg::Tag(sub_msg) => {
            if let Page::Tag(sub_page) = &mut model.page {
                pages::tag::update(
                    sub_msg,
                    sub_page,
                    &mut model.ctx,
                    &mut orders.proxy(Msg::Tag),
                )
            }
        }

        Msg::Service(sub_msg) => {
            if let Page::Service(sub_page) = &mut model.page {
                pages::service::update(
                    sub_msg,
                    sub_page,
                    &mut model.ctx,
                    &mut orders.proxy(Msg::Service),
                )
            }
        }

        Msg::Address(sub_msg) => {
            if let Page::Address(sub_page) = &mut model.page {
                pages::address::update(
                    sub_msg,
                    sub_page,
                    &mut model.ctx,
                    &mut orders.proxy(Msg::Address),
                )
            }
        }

        Msg::List(sub_msg) => {
            if let Page::List(sub_page) = &mut model.page {
                pages::list::update(
                    sub_msg,
                    sub_page,
                    &mut model.ctx,
                    &mut orders.proxy(Msg::List),
                )
            }
        }

        Msg::Analysis(sub_msg) => {
            if let Page::Analysis(sub_page) = &mut model.page {
                pages::analysis::update(
                    sub_msg,
                    sub_page,
                    &mut model.ctx,
                    &mut orders.proxy(Msg::Analysis),
                )
            }
        }
        _ => {}
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl IntoNodes<Msg> {
    nodes![
        pages::view_header(&model.ctx.base_url, model),
        match &model.page {
            Page::Home => pages::home::view(),

            Page::Chain(chain_model) =>
                pages::chain::view(chain_model, &model.ctx).map_msg(Msg::Chain),

            Page::Tag(tag_model) => pages::tag::view(tag_model, &model.ctx).map_msg(Msg::Tag),

            Page::Service(service_model) =>
                pages::service::view(service_model, &model.ctx).map_msg(Msg::Service),

            Page::Address(address_model) =>
                pages::address::view(address_model, &model.ctx).map_msg(Msg::Address),

            Page::List(list_model) => pages::list::view(list_model, &model.ctx).map_msg(Msg::List),

            Page::Analysis(analysis_model) =>
                pages::analysis::view(analysis_model, &model.ctx).map_msg(Msg::Analysis),

            _ => pages::not_found::view(),
        },
    ]
}
// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
