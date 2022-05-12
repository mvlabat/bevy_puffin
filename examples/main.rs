use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_egui::{EguiContext, EguiPlugin};
use bevy_puffin::PuffinTracePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultBevyPlugins)
        .add_plugin(PuffinTracePlugin::new())
        .add_plugin(EguiPlugin)
        .add_system(show_profiler)
        .run();
}

fn show_profiler(mut ctx: ResMut<EguiContext>, mut frame_counter: Local<usize>) {
    puffin::profile_function!();

    let ctx = ctx.ctx_mut();
    puffin_egui::profiler_window(ctx);

    std::thread::Builder::new()
        .name("Other thread".to_owned())
        .spawn(|| {
            sleep_ms(5);
        })
        .unwrap();

    sleep_ms(7);
    if *frame_counter % 49 == 0 {
        puffin::profile_scope!("Spike");
        std::thread::sleep(std::time::Duration::from_millis(10))
    }
    if *frame_counter % 343 == 0 {
        puffin::profile_scope!("Big spike");
        std::thread::sleep(std::time::Duration::from_millis(25))
    }

    for _ in 0..1000 {
        puffin::profile_scope!("very thin");
    }

    *frame_counter += 1;
}

fn sleep_ms(ms: usize) {
    puffin::profile_function!();
    match ms {
        0 => {}
        1 => std::thread::sleep(std::time::Duration::from_millis(1)),
        _ => {
            sleep_ms(ms / 2);
            sleep_ms(ms - (ms / 2));
        }
    }
}

pub struct DefaultBevyPlugins;

impl PluginGroup for DefaultBevyPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(bevy::core::CorePlugin::default());
        group.add(bevy::transform::TransformPlugin::default());
        group.add(bevy::hierarchy::HierarchyPlugin::default());
        group.add(bevy::diagnostic::DiagnosticsPlugin::default());
        group.add(bevy::input::InputPlugin::default());
        group.add(bevy::window::WindowPlugin::default());
        group.add(bevy::asset::AssetPlugin::default());
        group.add(bevy::scene::ScenePlugin::default());

        // bevy/debug_asset_server
        // group.add(bevy::asset::debug_asset_server::DebugAssetServerPlugin::default());

        // bevy/bevy_winit
        group.add(bevy::winit::WinitPlugin::default());

        // bevy/bevy_render
        group.add(bevy::render::RenderPlugin::default());

        // bevy/bevy_core_pipeline
        group.add(bevy::core_pipeline::CorePipelinePlugin::default());

        // bevy/bevy_sprite
        group.add(bevy::sprite::SpritePlugin::default());

        // bevy/bevy_text
        group.add(bevy::text::TextPlugin::default());

        // bevy/bevy_ui
        group.add(bevy::ui::UiPlugin::default());

        // bevy/bevy_pbr
        group.add(bevy::pbr::PbrPlugin::default());

        // bevy/bevy_gltf
        group.add(bevy::gltf::GltfPlugin::default());

        // bevy/bevy_audio
        group.add(bevy::audio::AudioPlugin::default());

        // bevy/bevy_gilrs
        group.add(bevy::gilrs::GilrsPlugin::default());

        // bevy/bevy_animation
        group.add(bevy::animation::AnimationPlugin::default());
    }
}
