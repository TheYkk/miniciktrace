#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::fmt::format;
use std::future::ready;
use std::future::Future;
use std::future::Ready;
use std::pin::Pin;

use actix_web::dev::forward_ready;
use actix_web::dev::Service;
use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceResponse;
use actix_web::dev::Transform;
use actix_web::get;
use actix_web::web;
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
impl<S, B> Service<ServiceRequest> for SayHiMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<Result<Self::Response, Self::Error>>;
    #[inline]
    fn poll_ready(
        &self,
        cx: &mut ::core::task::Context<'_>,
    ) -> ::core::task::Poll<Result<(), Self::Error>> {
        self.service
            .poll_ready(cx)
            .map_err(::core::convert::Into::into)
    }
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let parent = SpanContext::random();
        let root = Span::root(
            {
                let res =
                    ::alloc::fmt::format(format_args!("HTTP {0} : {1}", req.method(), req.path()));
                res
            },
            parent,
        )
        .with_property(|| ("aa", "bb"));
        let _guard = root.set_local_parent();
        {
            ::std::io::_print(format_args!(
                "Hi from start. You requested: {0}\n",
                req.path()
            ));
        };
        let fut = self.service.call(req).in_span(root);
        Box::pin(
            async move {
                let res = fut.await?;
                let _span = LocalSpan::enter_with_local_parent("a child span");
                {
                    ::std::io::_print(format_args!("Hi from response\n"));
                };
                Ok(res)
            }
            .in_span(Span::enter_with_local_parent("box")),
        )
    }
}
pub struct SayHi;
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
fn main() -> std::io::Result<()> {
    let body = async {
        env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .init();
        let reporter =
            JaegerReporter::new("127.0.0.1:6831".parse().unwrap(), "asynchronous").unwrap();
        minitrace::set_reporter(reporter, Config::default().report_before_root_finish(false));
        HttpServer::new(|| App::new().service(greet).wrap(SayHi))
            .bind(("127.0.0.1", 8080))?
            .run()
            .await?;
        minitrace::flush();
        Ok(())
    };
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
async fn func2(i: u64) {
    {
        let __span__ = minitrace::Span::enter_with_local_parent({
            fn f() {}
            fn type_name_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let name = type_name_of(f);
            let name = &name[..name.len() - 3];
            name.trim_end_matches("::{{closure}}")
        });
        minitrace::future::FutureExt::in_span(
            async move {
                {
                    Event::add_to_local_parent("event in root", || {
                        [("key".into(), "value".into())]
                    });
                    futures_timer::Delay::new(std::time::Duration::from_millis(i)).await;
                    Event::add_to_local_parent("printing", || {
                        [("int_val".into(), i.to_string().into())]
                    });
                    {
                        ::std::io::_print(format_args!("asd {0}\n", i));
                    }
                }
            },
            __span__,
        )
    }
    .await
}
#[allow(non_camel_case_types, missing_docs)]
pub struct greet;
impl ::actix_web::dev::HttpServiceFactory for greet {
    fn register(self, __config: &mut actix_web::dev::AppService) {
        async fn greet(name: web::Path<String>) -> impl Responder {
            async move {
                let __ret_value = async move {
                    {
                        {
                            let __span__ = minitrace::Span::enter_with_local_parent({
                                fn f() {}
                                fn type_name_of<T>(_: T) -> &'static str {
                                    std::any::type_name::<T>()
                                }
                                let name = type_name_of(f);
                                let name = &name[..name.len() - 3];
                                name.trim_end_matches("::{{closure}}")
                            });
                            minitrace::future::FutureExt::in_span(
                                async move {
                                    {
                                        let do_something_future = do_something_async(125);
                                        let func2_futures = (0..15).map(|i| func2(i));
                                        func2_futures.join_all(func2_futures).await;
                                        do_something_future.await;
                                        {
                                            let res = ::alloc::fmt::format(format_args!(
                                                "Hello {0}!",
                                                name
                                            ));
                                            res
                                        }
                                    }
                                },
                                __span__,
                            )
                        }
                        .await
                    }
                }
                .await;
                {
                    let lvl = ::log::Level::Info;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            format_args!("{0}() => {1:?}", "greet", &__ret_value),
                            lvl,
                            &("trace_test", "trace_test", "src/main.rs"),
                            106u32,
                            (),
                        );
                    }
                };
                __ret_value
            }
            .await
        }
        let __resource = ::actix_web::Resource::new("/hello/{name}")
            .name("greet")
            .guard(::actix_web::guard::Get())
            .to(greet);
        ::actix_web::dev::HttpServiceFactory::register(__resource, __config);
    }
}
async fn do_something_async(i: u64) {
    async move {
        let __ret_value = async move {
            {
                {
                    let __span__ = minitrace::Span::enter_with_local_parent({
                        fn f() {}
                        fn type_name_of<T>(_: T) -> &'static str {
                            std::any::type_name::<T>()
                        }
                        let name = type_name_of(f);
                        let name = &name[..name.len() - 3];
                        name.trim_end_matches("::{{closure}}")
                    });
                    minitrace::future::FutureExt::in_span(
                        async move {
                            {
                                futures_timer::Delay::new(std::time::Duration::from_millis(i))
                                    .await;
                            }
                        },
                        __span__,
                    )
                }
                .await
            }
        }
        .await;
        {
            let lvl = ::log::Level::Debug;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api::log(
                    format_args!("{0}() => {1:?}", "do_something_async", &__ret_value),
                    lvl,
                    &("trace_test", "trace_test", "src/main.rs"),
                    131u32,
                    (),
                );
            }
        };
        __ret_value
    }
    .await
}
