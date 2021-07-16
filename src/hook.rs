/// Hook functions with the [`ilhook`](https://docs.rs/ilhook) crate, and never unhook. Should be invoked once when the shared library attaches.
#[macro_export]
macro_rules! hook {
    ($arch:ident, $({ $addr:expr, $hook_type:ident, $hook_fn:ident }),* $(,)?) => {
        vec![
            $(
                ilhook::$arch::Hooker::new(
                    $addr,
                    ilhook::$arch::HookType::$hook_type($hook_fn),
                    ilhook::$arch::CallbackOption::None,
                    ilhook::$arch::HookFlags::empty(),
                )
                .hook(),
            )*
        ]
        .leak();
    }
}
