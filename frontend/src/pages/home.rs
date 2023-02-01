use seed::{prelude::*, *};

pub fn view() -> Node<crate::Msg> {
    div![
        C!["container"],
        h1!["Crypto address relation"],
        div!["Lorem ipsum!"],
    ]
}
