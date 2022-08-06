use gtk::glib;
use std::future::Future;

fn with_thread_context<R>(func: impl FnOnce(glib::MainContext) -> R) -> R {
    let ctx = glib::MainContext::thread_default().unwrap_or_else(glib::MainContext::new);
    func(ctx)
}

pub fn block_on<F>(future: F) -> F::Output
where
    F: Future,
{
    with_thread_context(|ctx| ctx.block_on(future))
}
