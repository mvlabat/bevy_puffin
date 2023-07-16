#![deny(missing_docs)]

//! This crate integrates the `puffin` library into Bevy.
//!
//! It provides [`PuffinTracePlugin`] to use as a replacement for the Bevy's default `LogPlugin`
//! plugin and exposes [`PuffinLayer`], which allows users to set up `tracing` manually with
//! `puffin` as a subscriber layer.

pub use puffin;

use bevy::{
    app::{App, Plugin},
    log,
    log::Level,
    utils::tracing::{
        span::{Attributes, Record},
        Id, Subscriber,
    }, prelude::First,
};
use puffin::ThreadProfiler;
use std::{cell::RefCell, collections::VecDeque, panic};
use tracing_log::LogTracer;
use tracing_subscriber::{
    fmt::{format::DefaultFields, FormatFields, FormattedFields},
    layer::Context,
    prelude::*,
    registry::{LookupSpan, Registry},
    EnvFilter, Layer,
};

thread_local! {
    static PUFFIN_SPAN_STACK: RefCell<VecDeque<(Id, usize)>> =
        RefCell::new(VecDeque::with_capacity(16));
}

/// A plugin that sets up `puffin` and configures it as a `tracing-subscriber` layer.
///
/// Note that this plugin can't be used with Bevy's default `LogPlugin`. If you are using the
/// `DefaultPlugins` group, make sure to disable `LogPlugin`.
///
/// Unlike Bevy's `LogPlugin`, this plugin doesn't support Android and it doesn't initialize
/// `tracing-tracy` or `tracing-chrome`. If you need to support either of those, consider
/// initializing `tracing` manually.
pub struct PuffinTracePlugin {
    init_systems: bool,
    init_scopes: bool,
    filter: String,
    level: Level,
}

impl Default for PuffinTracePlugin {
    fn default() -> Self {
        Self {
            init_systems: true,
            init_scopes: true,
            filter: "wgpu=error".to_string(),
            level: Level::INFO,
        }
    }
}

impl PuffinTracePlugin {
    /// Creates `PuffinTracePlugin` with the default configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enables initializing systems. If you want to control in marking `puffin` frames manually,
    /// you might be interested in disabling them. (Systems are enabled by default.)
    pub fn with_systems(self) -> Self {
        Self {
            init_systems: true,
            ..self
        }
    }

    /// Disables initializing systems. If you want to control in marking `puffin` frames manually,
    /// you might be interested in disabling them. (Systems are enabled by default.)
    pub fn without_systems(self) -> Self {
        Self {
            init_systems: false,
            ..self
        }
    }

    /// Enables puffin profiler macros (calls `puffin::set_scopes_on(true)` on building the plugin).
    /// (Enabled by default.)
    pub fn with_scopes_on(self) -> Self {
        Self {
            init_scopes: true,
            ..self
        }
    }

    /// Disables puffin profiler macros (calls `puffin::set_scopes_on(false)` on building the
    /// plugin). (Enabled by default.)
    pub fn with_scopes_off(self) -> Self {
        Self {
            init_scopes: false,
            ..self
        }
    }

    /// Filters logs using the [`EnvFilter`] format.
    pub fn with_filter(self, filter: String) -> Self {
        Self { filter, ..self }
    }

    /// Filters out logs that are "less than" the given level.
    /// This can be further filtered using the `filter` setting.
    pub fn with_level(self, level: Level) -> Self {
        Self { level, ..self }
    }
}

/// Marks a new frame for the puffin profiler.
pub fn new_frame_system() {
    puffin::GlobalProfiler::lock().new_frame();
}

impl Plugin for PuffinTracePlugin {
    fn build(&self, app: &mut App) {
        if self.init_systems {
            app.add_systems(First, new_frame_system);
        }
        if self.init_scopes {
            puffin::set_scopes_on(true);
        }

        {
            let old_handler = panic::take_hook();
            panic::set_hook(Box::new(move |infos| {
                println!("{}", tracing_error::SpanTrace::capture());
                old_handler(infos);
            }));
        }

        let finished_subscriber;
        let default_filter = format!("{},{}", self.level, self.filter);
        let filter_layer = EnvFilter::try_from_default_env()
            .or_else(|_| EnvFilter::try_new(&default_filter))
            .unwrap();
        let subscriber = Registry::default()
            .with(filter_layer)
            .with(PuffinLayer::new())
            .with(tracing_error::ErrorLayer::default());

        #[cfg(not(target_arch = "wasm32"))]
        {
            let fmt_layer = tracing_subscriber::fmt::Layer::default();
            finished_subscriber = subscriber.with(fmt_layer);
        }

        #[cfg(target_arch = "wasm32")]
        {
            console_error_panic_hook::set_once();
            finished_subscriber = subscriber.with(tracing_wasm::WASMLayer::new(
                tracing_wasm::WASMLayerConfig::default(),
            ));
        }

        let logger_already_set = LogTracer::init().is_err();
        let subscriber_already_set =
            bevy::utils::tracing::subscriber::set_global_default(finished_subscriber).is_err();

        match (logger_already_set, subscriber_already_set) {
            (true, true) => log::warn!(
                "Could not set global logger and tracing subscriber for bevy_puffin as they are already set. Consider disabling LogPlugin or re-ordering plugin initialization."
            ),
            (true, _) => log::warn!("Could not set global logger as it is already set. Consider disabling LogPlugin."),
            (_, true) => log::warn!("Could not set global tracing subscriber as it is already set. Consider disabling LogPlugin."),
            _ => (),
        }
    }
}

/// A tracing layer that collects data for puffin.
pub struct PuffinLayer<F = DefaultFields> {
    fmt: F,
}

impl Default for PuffinLayer<DefaultFields> {
    fn default() -> Self {
        Self::new()
    }
}

impl PuffinLayer<DefaultFields> {
    /// Create a new `PuffinLayer`.
    pub fn new() -> Self {
        Self {
            fmt: DefaultFields::default(),
        }
    }

    /// Use a custom field formatting implementation.
    pub fn with_formatter<F>(self, fmt: F) -> PuffinLayer<F> {
        let _ = self;
        PuffinLayer { fmt }
    }
}

impl<S: Subscriber, F> Layer<S> for PuffinLayer<F>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    F: for<'writer> FormatFields<'writer> + 'static,
{
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        if !puffin::are_scopes_on() {
            return;
        }

        if let Some(span) = ctx.span(id) {
            let mut extensions = span.extensions_mut();
            if extensions.get_mut::<FormattedFields<F>>().is_none() {
                let mut fields = FormattedFields::<F>::new(String::with_capacity(64));
                if self.fmt.format_fields(fields.as_writer(), attrs).is_ok() {
                    extensions.insert(fields);
                }
            }
        }
    }

    fn on_record(&self, id: &Id, values: &Record<'_>, ctx: Context<'_, S>) {
        if let Some(span) = ctx.span(id) {
            let mut extensions = span.extensions_mut();
            if let Some(fields) = extensions.get_mut::<FormattedFields<F>>() {
                let _ = self.fmt.add_fields(fields, values);
            } else {
                let mut fields = FormattedFields::<F>::new(String::with_capacity(64));
                if self.fmt.format_fields(fields.as_writer(), values).is_ok() {
                    extensions.insert(fields);
                }
            }
        }
    }

    fn on_enter(&self, id: &Id, ctx: Context<'_, S>) {
        if !puffin::are_scopes_on() {
            return;
        }

        if let Some(span_data) = ctx.span(id) {
            let metadata = span_data.metadata();
            let name = metadata.name();
            let target = metadata.target();
            let extensions = span_data.extensions();
            let data = extensions
                .get::<FormattedFields<F>>()
                .map(|fields| fields.fields.as_str())
                .unwrap_or_default();

            ThreadProfiler::call(|tp| {
                let start_stream_offset = tp.begin_scope(name, target, data);
                PUFFIN_SPAN_STACK.with(|s| {
                    s.borrow_mut().push_back((id.clone(), start_stream_offset));
                });
            });
        }
    }

    fn on_exit(&self, id: &Id, _ctx: Context<'_, S>) {
        PUFFIN_SPAN_STACK.with(|s| {
            let value = s.borrow_mut().pop_back();
            if let Some((last_id, start_stream_offset)) = value {
                if *id == last_id {
                    ThreadProfiler::call(|tp| tp.end_scope(start_stream_offset));
                } else {
                    s.borrow_mut().push_back((last_id, start_stream_offset));
                }
            }
        });
    }

    fn on_close(&self, id: Id, ctx: Context<'_, S>) {
        if let Some(span) = ctx.span(&id) {
            span.extensions_mut().remove::<FormattedFields<F>>();
        }
    }
}
