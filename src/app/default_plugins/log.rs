use bevy::{
    app::PluginGroupBuilder,
    log::{Level, LogPlugin},
    prelude::*,
};

pub(super) fn set_log_plugin<T: PluginGroup>(default_plugins: T) -> PluginGroupBuilder {
    default_plugins.set(LogPlugin {
        filter: log_filter(log_level()).to_string(),
        level: log_level(),
    })
}

const fn log_level() -> Level {
    if cfg!(feature = "tracing") {
        Level::TRACE
    } else if cfg!(feature = "dev") {
        Level::INFO
    } else {
        Level::ERROR
    }
}

const fn log_filter(log_level: Level) -> &'static str {
    match log_level {
        Level::INFO => {
            "wgpu_hal=warn,gfx_backend_metal=warn,wgpu_core=warn,bevy_render=info,lain=debug,\
             bevy_render::render_resource::pipeline_cache=warn,sequence=debug,naga=warn"
        },
        Level::TRACE | Level::DEBUG => {
            "wgpu_hal=warn,gfx_backend_metal=warn,wgpu_core=warn,bevy_render=info,lain=debug,\
             bevy_render::render_resource::pipeline_cache=warn,bevy_app=debug,sequence=debug,naga=warn"
        },
        _ => "",
    }
}
