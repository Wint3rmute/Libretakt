use libretakt::engine::{Engine, Voice};
use libretakt::sample_provider::SampleProvider;
use libretakt::sequencer::Sequencer;
use macroquad::prelude::*;

use macroquad::ui::{hash, root_ui, widgets};
use rodio::{OutputStream, Sink};
use std::sync::{Arc, RwLock};

#[macroquad::main("LibreTakt")]
async fn main() {
    let provider = Arc::new(SampleProvider::default());

    let sequencer = Arc::new(RwLock::new(Sequencer::new()));
    let voice = Voice::new(&provider);
    let engine = Engine {
        sequencer: sequencer.clone(),
        voices: vec![voice],
    };

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    sink.append(engine);
    sink.play();

    let mut sample = 0.0;

    loop {
        clear_background(BLACK);

        {
            // controller.mutate_default_param(0, Parameters::Sample, sample as u8);
            let mut sequencer = sequencer.write().unwrap();

            let current_pattern = &mut sequencer.tracks[0].patterns[0]; // Hardcoded
            let num_of_steps = current_pattern.steps.len();

            let fps = get_fps();
            root_ui().button(None, format!("{fps}"));

            widgets::Window::new(
                hash!(),
                vec2(0.0, 0.0),
                vec2(screen_width() / 2.0, screen_height()),
            )
            .label("whatever")
            .ui(&mut *root_ui(), |ui| {
                for i in 0..num_of_steps {
                    ui.slider(hash!(), "[-10 .. 10]", 0f32..10f32, &mut sample);
                    if ui.button(
                        None,
                        if current_pattern.steps[i].is_some() {
                            "X"
                        } else {
                            " "
                        },
                    ) {
                        if current_pattern.steps[i].is_some() {
                            // controller.set_step(0, 0, i);
                            println!("TODO set step");
                        } else {
                            // controller.remove_step(0, 0, i);
                            println!("TODO remove step");
                        }
                    }
                }
            });
        }
        next_frame().await;
    }
}
