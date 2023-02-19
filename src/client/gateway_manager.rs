use crate::login::login_conn;
use crate::models::token;
use jwt;
use jwt::{Header, Token};
use log::info;
use std::borrow::Borrow;
use std::ops::Deref;

pub fn add_gateway() {
    let jwt_str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJSdW5JZCI6ImI4YzUxOTI1LTViYzktNGRkMy05YTM1LWYzMjAyNDMxYWEwOCIsIkhvc3QiOiJndW9uZWkubmF0LWNsb3VkLmNvbSIsIlRjcFBvcnQiOjM0MzIwLCJLY3BQb3J0IjozNDMyMCwiVGxzUG9ydCI6MzQzMjEsIkdycGNQb3J0IjozNDMyMiwiVURQQXBpUG9ydCI6MzQzMjEsIktDUEFwaVBvcnQiOjM0MzIyLCJQZXJtaXNzaW9uIjpbImdhdGV3YXkiXSwiVHh0cyI6e30sImV4cCI6MjAxNjc4Mjg0NDk4LCJuYmYiOjE2NzgyNTU2OTh9.iMxk4COineumzJbHQJ7p07jO-jq7Y1CDILpxm6Ilkbs".to_string();
    let jwt_decoded: Token<Header, token::TokenClaim, _> =
        jwt::Token::parse_unverified(&jwt_str).unwrap();
    info!("jwt_decoded:{:?}", jwt_decoded.claims());
    // 连接服务器并登录服务器
    login_conn::login_conn(jwt_str.clone(), jwt_decoded.claims().deref()).expect("login err");
}
