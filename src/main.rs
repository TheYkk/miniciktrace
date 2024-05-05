use std::fmt::format;
use std::future::ready;
use std::future::Future;
use std::future::Ready;
use std::pin::Pin;

use actix_web::dev::Service;
use actix_web::web;
use futures::future::try_join_all;
#[cfg(not(debug_assertions))]
use human_panic::setup_panic;
use tokio::join;
use tokio::select;
use tokio::spawn;
use tokio::time::sleep;

#[cfg(debug_assertions)]
extern crate better_panic;

use std::collections::BTreeMap;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

use actix_web::dev::forward_ready;
use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceResponse;
use actix_web::dev::Transform;
use actix_web::get;
use actix_web::App;
use actix_web::Error;
use actix_web::HttpServer;
use actix_web::Responder;
use futures::executor::block_on;
use futures::future::join_all;
use logcall::logcall;
use minitrace::collector::Config;
use minitrace::collector::SpanContext;
use minitrace::prelude::*;
use minitrace_jaeger::JaegerReporter;
use tokio::task;
use tokio::task::JoinSet;

pub struct SayHiMiddleware<S> {
    /// The next service to call
    service: S,
}
type LocalBoxFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

// `S`: type of the wrapped service
// `B`: type of the body - try to be generic over the body where possible
impl<S, B> Service<ServiceRequest> for SayHiMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<Result<Self::Response, Self::Error>>;

    // This service is ready when its next service is ready
    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let parent = SpanContext::random();
        let root = Span::root(format!("HTTP {} : {}", req.method(), req.path()), parent)
            .with_property(|| ("aa", "bb"));
        let _guard = root.set_local_parent();

        println!("Hi from start. You requested: {}", req.path());

        // A more complex middleware, could return an error or an early response here.

        let fut = self.service.call(req).in_span(root);

        Box::pin(
            async move {
                let res = fut.await?;
                let _span = LocalSpan::enter_with_local_parent("a child span");
                println!("Hi from respoanse");
                Ok(res)
            }
            .in_span(Span::enter_with_local_parent("box")),
        )
    }
}

pub struct SayHi;

// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for SayHi
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = SayHiMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SayHiMiddleware { service }))
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    // Human Panic. Only enabled when *not* debugging.
    #[cfg(not(debug_assertions))]
    {
        setup_panic!();
    }

    // Better Panic. Only enabled *when* debugging.
    #[cfg(debug_assertions)]
    {
        better_panic::Settings::debug()
            .most_recent_first(false)
            .lineno_suffix(true)
            .verbosity(better_panic::Verbosity::Full)
            .install();
    }

    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let reporter = JaegerReporter::new("127.0.0.1:6831".parse().unwrap(), "asynchronous").unwrap();

    minitrace::set_reporter(reporter, Config::default().report_before_root_finish(false));

    HttpServer::new(|| App::new().service(greet).wrap(SayHi))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await?;

    minitrace::flush();
    Ok(())
}

#[trace]
// #[logcall("info")]
async fn func2(i: u64) {
    Event::add_to_local_parent("event in root", || [("key".into(), "value".into())]);
    println!("girdim {}", i);
    sleep(std::time::Duration::from_millis(i)).await;
    Event::add_to_local_parent("printing", || [("int_val".into(), i.to_string().into())]);
    println!("ciktim {}", i)
}

#[get("/hello/{name}")]
#[trace]
#[logcall("info")]
async fn greet(name: web::Path<String>) -> impl Responder {
    let do_something_future = do_something_async(125);

    let func2_futures = (0..15).map(|i| func2(i));
    let _ = spawn(async { do_something_future.await });
    join_all(func2_futures).await;
    // join!(join_all(func2_futures), do_something_future());

    // do_something_future.await;

    // async {
    //     do_something_async(125).await;
    // }.in_span(Span::enter_with_local_parent("aync_job"));
    //
    //
    // for i in 0..15 {
    //     func2(i).await;
    // }
    format!("Hello {name}!")
}

#[trace]
#[logcall("debug")]
async fn do_something_async(i: u64) {
    println!("wait 1");
    sleep(std::time::Duration::from_millis(i / 3)).await;
    println!("wait 2");
    sleep(std::time::Duration::from_millis(i / 3)).await;
    println!("wait 3");
    sleep(std::time::Duration::from_millis(i / 3)).await;
    println!("wait done");
}
