use crate::Error;
use std::task::{Context, Poll};
use surrealdb::sql::{ Thing, Value };
use pin_project_lite::pin_project;
use futures_lite::future::FutureExt;

pub struct DbRequest {
    cmd: Cmd,
}

pub trait PopRequest<T> {
    fn pop_request(self) -> Cmd;
}

impl PopRequest<DbRequest> for DbRequest {
    fn pop_request(self) -> Cmd {
        self.cmd
    }
}

pin_project! {
    pub struct DbResponse<T> {
        #[pin]
        pub future: T,
    }
}

impl DbRequest {
    pub fn create_table(table: &str, value: Option<Value>) -> Self {
        Self { 
            cmd: Cmd::CreateTable(String::from(table), value)
        }
    }

    pub fn get_record(table: &str, id: &str) -> Self {
        Self {
            cmd: Cmd::Get(String::from(table), String::from(id))
        }
    }

    pub fn set_record(table: &str, id: &str, value: Value) -> Self {
        Self {
            cmd: Cmd::Set(String::from(table), String::from(id), value)
        }
    }

    pub fn query(query: &str) -> Self {
        Self {
            cmd: Cmd::Query(String::from(query))
        }
    }

    pub fn call_fn(name: &str, args: Vec<Value>) -> Self {
        Self {
            cmd: Cmd::CallFn(String::from(name), args)
        }
    }
}

pub enum Cmd {
    CreateTable(String, Option<Value>),
    Get(String, String),
    Set(String, String, Value),
    Query(String),
    CallFn(String, Vec<Value>),
}

use std::{pin::Pin, future::Future};

impl<F> Future for DbResponse<F>
where
    F: Future<Output = Result<Value, Error>>,
{
    type Output = Result<F, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.future.poll(cx) {
            Poll::Ready(response) => Poll::Ready(response),
            Poll::Pending => Poll::Pending,
        }
    }
}