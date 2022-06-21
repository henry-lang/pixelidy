mod library;

use eframe::{
    egui::{
        style::{WidgetVisuals, Widgets},
        Button, CentralPanel, Color32, Context, FontData, FontDefinitions, FontFamily, RichText,
        Rounding, Stroke, Visuals,
    },
    App, CreationContext,
};
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::{BufReader, Cursor};

struct Pixelidy {
    sink: Sink,
}

impl Pixelidy {
    fn new(sink: Sink) -> Self {
        Self { sink }
    }
}

impl eframe::App for Pixelidy {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            if ui.add(Button::new(RichText)).clicked() {
                println!("Clicked!");
            }
        });
    }
}

impl Pixelidy {
    fn setup(mut self, cc: &CreationContext) -> Self {
        self.configure_fonts(&cc.egui_ctx);

        self
    }

    fn configure_fonts(&self, ctx: &Context) {
        let mut fonts = FontDefinitions::default();

        fonts.font_data.insert(
            "pixeloid".into(),
            FontData::from_static(include_bytes!("fonts/pixeloid.ttf")),
        );

        fonts.font_data.insert(
            "pixeloid-bold".into(),
            FontData::from_static(include_bytes!("fonts/pixeloid-bold.ttf")),
        );

        fonts.families.insert(
            FontFamily::Proportional,
            vec!["pixeloid".into(), "pixeloid-bold".into()],
        );

        ctx.set_fonts(fonts);
    }
}

fn main() {
    let options = eframe::NativeOptions::default();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let visuals = Visuals {
        widgets: Widgets {
            inactive: WidgetVisuals {
                bg_fill: Color32::WHITE,
                bg_stroke: Stroke::new(2.0, Color32::BLACK),
                fg_stroke: Stroke::new(1.0, Color32::BLACK),
                rounding: Rounding::none(),
                expansion: 0.0,
            },
            noninteractive: WidgetVisuals {
                bg_fill: Color32::WHITE,
                bg_stroke: Stroke::new(2.0, Color32::BLACK),
                fg_stroke: Stroke::new(1.0, Color32::BLACK),
                rounding: Rounding::none(),
                expansion: 0.0,
            },
            hovered: WidgetVisuals {
                bg_fill: Color32::WHITE,
                bg_stroke: Stroke::new(2.0, Color32::BLACK),
                fg_stroke: Stroke::new(1.0, Color32::BLACK),
                rounding: Rounding::none(),
                expansion: 0.0,
            },
            active: WidgetVisuals {
                bg_fill: Color32::BLACK,
                bg_stroke: Stroke::new(2.0, Color32::BLACK),
                fg_stroke: Stroke::new(1.0, Color32::WHITE),
                rounding: Rounding::none(),
                expansion: 0.0,
            },
            open: WidgetVisuals {
                bg_fill: Color32::WHITE,
                bg_stroke: Stroke::new(2.0, Color32::BLACK),
                fg_stroke: Stroke::new(1.0, Color32::BLACK),
                rounding: Rounding::none(),
                expansion: 0.0,
            },

            ..Default::default()
        },
        ..Default::default()
    };

    eframe::run_native(
        "pixelidy",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(visuals);
            Box::new(Pixelidy::new(sink).setup(cc))
        }),
    )
}
