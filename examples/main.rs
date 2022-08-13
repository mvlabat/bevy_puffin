use bevy::{log::LogPlugin, prelude::*};
use bevy_egui::{EguiContext, EguiPlugin};
use bevy_puffin::PuffinTracePlugin;

fn main() {
    App::new()
        .add_plugins_with(DefaultPlugins, |group| group.disable::<LogPlugin>())
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
