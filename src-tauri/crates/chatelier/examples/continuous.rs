use std::{error::Error, time::Duration};

use chatelier::continuous::ContinuousSystem;
use chem_eq::Equation;
use egui::{
    plot::{Line, Plot, PlotPoints},
    Style, Visuals,
};

const EQUATION: &str = "H2 + I2 <-> 2HI";
const TIME_STEP: f64 = 0.001;

fn main() -> Result<(), Box<dyn Error>> {
    let native_opts = eframe::NativeOptions::default();
    eframe::run_native(
        EQUATION,
        native_opts,
        Box::new(|cc| Box::new(EguiApp::new(cc))),
    )?;

    Ok(())
}

struct EguiApp {
    sys: ContinuousSystem,
    plot: Vec<Vec<f32>>,
}

impl EguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let style = Style {
            visuals: Visuals::light(),
            ..Style::default()
        };
        cc.egui_ctx.set_style(style);

        let mut eq = Equation::new(EQUATION).unwrap();
        eq.set_concentrations(&[0.12, 0.3, 0.0]).unwrap();
        let mut sys = ContinuousSystem::new(eq, 1.0, 0.2);

        let plot = sys.solve(Duration::from_millis(10), Duration::from_secs(50));

        for (cnc, name) in plot.iter().zip(sys.eq().compound_names()) {
            println!("{name}: {}", cnc.last().unwrap());
        }
        Self { sys, plot }
    }
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            Plot::new(EQUATION)
                .clamp_grid(true)
                .legend(egui::plot::Legend::default())
                .show(ui, |plot_ui| {
                    for (i, cmp) in self.plot.iter().enumerate() {
                        let points: PlotPoints = cmp
                            .iter()
                            .enumerate()
                            .map(|(i, &c)| [i as f64 * TIME_STEP, c as f64])
                            .collect();
                        let line =
                            Line::new(points).name(self.sys.eq().compound_names().nth(i).unwrap());
                        plot_ui.line(line);
                    }
                })
        });
    }
}
