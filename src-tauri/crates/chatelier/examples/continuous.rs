use std::{error::Error, time::Duration};

use crate::validator::ValText;
use chatelier::continuous::ContinuousSystem;
use chem_eq::{Direction, Equation};
use egui::{
    plot::{Line, Plot, PlotPoints},
    Color32, ComboBox, RichText, Style, Vec2, Visuals,
};

const LEFT_EQUATION: &str = "H2 + I2";
const RIGHT_EQUATION: &str = "2HI";
const EQUATION_DIR: Direction = Direction::Right;
const TIME_STEP: f64 = 0.001;
const DEJA_VU_SANS: &[u8] = include_bytes!("assets/DejaVuSans.ttf");

fn main() -> Result<(), Box<dyn Error>> {
    let native_opts = eframe::NativeOptions::default();
    eframe::run_native(
        "Reaction Kinematics",
        native_opts,
        Box::new(|cc| Box::new(EguiApp::new(cc))),
    )?;

    Ok(())
}

struct EguiApp {
    sys: ContinuousSystem,
    plot: Vec<Vec<f32>>,
    modify_equation: bool,
    params: SimulationParams,
    default_params: SimulationParams,
}

impl EguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let style = Style {
            visuals: Visuals::light(),
            ..Style::default()
        };
        cc.egui_ctx.set_style(style);
        // add font that supports the unicode characters: ← → ⇌
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "DejaVuSans".to_owned(),
            egui::FontData::from_static(DEJA_VU_SANS),
        );
        // make highest priority
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "DejaVuSans".to_owned());
        cc.egui_ctx.set_fonts(fonts);

        let params = SimulationParams {
            eq_left: LEFT_EQUATION.to_string(),
            eq_right: RIGHT_EQUATION.to_string(),
            eq_dir: EQUATION_DIR,
            initial_cnc: vec![0.3, 0.12, 0.0],
            k_f: 1.0,
            k_r: 0.2,
            time_step: Duration::from_millis(10),
            stop: Duration::from_secs(50),
        };
        let (sys, plot) = recalculate_curve(&params);

        let default_params = params.clone();
        Self {
            sys,
            plot,
            modify_equation: true,
            params,
            default_params,
        }
    }
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu-bar")
            .min_height(30.0)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.menu_button("Continuous", |ui| {
                            if ui.button("Quit").clicked() {
                                frame.close();
                            }
                        });
                        ui.menu_button("Simulation", |ui| {
                            if ui.button("Modify Simulation").clicked() {
                                self.modify_equation = true;
                                ui.close_menu();
                            }
                            if ui.button("Reset").clicked() {
                                self.params = self.default_params.clone();
                                let (sys, plot) = recalculate_curve(&self.params);
                                self.sys = sys;
                                self.plot = plot;
                                self.modify_equation = true;
                                ui.close_menu();
                            }
                        });
                    });
                });
            });

        if self.modify_equation {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.style_mut().spacing.item_spacing = Vec2::new(10.0, 10.0);
                ui.heading("Customize Simulation");

                ui.horizontal(|ui| {
                    ui.label("Forward reaction rate");
                    let mut validator: ValText<f32, _> =
                        ValText::with_validator(|t| t.parse().ok());
                    validator.set_val(self.params.k_f);
                    let text_edit = egui::TextEdit::singleline(&mut validator).desired_width(70.0);
                    ui.add(text_edit);
                    self.params.k_f = validator.get_val().unwrap_or(self.params.k_f);
                });

                ui.horizontal(|ui| {
                    ui.label("Reverse reaction rate");
                    let mut validator: ValText<f32, _> =
                        ValText::with_validator(|t| t.parse().ok());
                    validator.set_val(self.params.k_r);
                    let text_edit = egui::TextEdit::singleline(&mut validator).desired_width(70.0);
                    ui.add(text_edit);
                    self.params.k_r = validator.get_val().unwrap_or(self.params.k_r);
                });

                ui.horizontal(|ui| {
                    ui.label("Equation:");
                    let text_edit = egui::TextEdit::singleline(&mut self.params.eq_left).clip_text(false).desired_width(50.0);
                    ui.add(text_edit);
                    ComboBox::new("dir-selector", "").width(10.0).selected_text(self.params.dir_str()).show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.params.eq_dir, Direction::Left, "←");
                        ui.selectable_value(&mut self.params.eq_dir, Direction::Right, "→");
                        ui.selectable_value(&mut self.params.eq_dir, Direction::Reversible, "⇌");
                    });
                    let text_edit = egui::TextEdit::singleline(&mut self.params.eq_right).clip_text(false).desired_width(50.0);
                    ui.add(text_edit);
                });

                ui.label("Concentrations:");
                match Equation::new(&self.params.build_str()) {
                    Ok(mut eq) if eq.is_balanced() => {
                        if eq.num_compounds() > self.params.initial_cnc.len() {
                            self.params.initial_cnc.extend(
                                &[0.0].repeat(eq.num_compounds() - self.params.initial_cnc.len()),
                            );
                        }
                        while eq.num_compounds() < self.params.initial_cnc.len() {
                            self.params.initial_cnc.pop();
                        }
                        eq.set_concentrations(&self.params.initial_cnc).unwrap();
                        let names: Vec<_> = eq.compound_names().map(ToString::to_string).collect();
                        for (cmp, name) in eq.iter_compounds_mut().zip(names.into_iter()) {
                            ui.horizontal(|ui| {
                                ui.label(name);
                                ui.add(egui::Slider::new(&mut cmp.concentration, 0.0..=0.5));
                                ui.label("mol/L");
                            });
                        }
                        self.params.initial_cnc = eq.get_concentrations();

                        match eq.direction() {
                            Direction::Left => {
                                if eq.right().iter().any(|c| c.concentration == 0.0) {
                                    ui.label(RichText::new("Equation has zero concentration reactants and thus won't react").color(Color32::RED));
                                }
                            }
                            Direction::Right => {
                                if eq.left().iter().any(|c| c.concentration == 0.0) {
                                    ui.label(RichText::new("Equation has zero concentration reactants and thus won't react").color(Color32::RED));
                                }
                            }
                            Direction::Reversible => {
                                if eq.left().iter().any(|c| c.concentration == 0.0) && eq.right().iter().any(|c| c.concentration == 0.0) {
                                    ui.label(RichText::new("Equation has zero concentration reactants and thus won't react").color(Color32::RED));
                                }
                            }
                        }
                    }
                    Err(e) => {
                        ui.label(RichText::new(format!("Error: {e}")).color(Color32::RED));
                    }
                    Ok(eq) => {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Equation is not balanced").color(Color32::RED));
                            if let Ok(eq) = eq.to_balancer().balance() {
                                if ui.button("Balance").clicked() {
                                    let delim = eq.direction().to_string();
                                    let eq_str = eq.equation();
                                    let (left, right) = eq_str.split_once(&delim).unwrap();
                                    let (left, right) = (left.trim(), right.trim());

                                    self.params.eq_left = left.to_string();
                                    self.params.eq_right = right.to_string();
                                }
                            } else {
                                ui.label(RichText::new("Equation can't be balanced").color(Color32::RED));
                            }
                        });
                    }
                }

                ui.horizontal(|ui| {
                    ui.label("Time Step");
                    let mut validator: ValText<u64, _> =
                        ValText::with_validator(|t| t.parse().ok());
                    validator.set_val(self.params.time_step.as_millis().try_into().unwrap());
                    let text_edit = egui::TextEdit::singleline(&mut validator).desired_width(70.0);
                    ui.add(text_edit);
                    self.params.time_step = validator
                        .get_val()
                        .map(Duration::from_millis)
                        .unwrap_or(self.params.time_step);
                    ui.label("(ms)");
                });

                ui.horizontal(|ui| {
                    ui.label("Stop Time");
                    let mut validator: ValText<u64, _> =
                        ValText::with_validator(|t| t.parse().ok());
                    validator.set_val(self.params.stop.as_millis().try_into().unwrap());
                    let text_edit = egui::TextEdit::singleline(&mut validator).desired_width(70.0);
                    ui.add(text_edit);
                    self.params.stop = validator
                        .get_val()
                        .map(Duration::from_millis)
                        .unwrap_or(self.params.stop);
                    ui.label("(ms)");
                });

                if ui.button("Simulate").clicked() {
                    let Ok(mut eq) = Equation::new(&self.params.build_str()) else {
                        return;
                    };
                    let Ok(_) = eq.set_concentrations(&self.params.initial_cnc) else { return };
                    if !eq.is_balanced() {
                        return;
                    }

                    match eq.direction() {
                        Direction::Left => {
                            if eq.right().iter().any(|c| c.concentration == 0.0) {
                                return;
                            }
                        }
                        Direction::Right => {
                            if eq.left().iter().any(|c| c.concentration == 0.0) {
                                return;
                            }
                        }
                        Direction::Reversible => {
                            if eq.left().iter().any(|c| c.concentration == 0.0) && eq.right().iter().any(|c| c.concentration == 0.0) {
                                return;
                            }
                        }
                    }

                    self.modify_equation = false;
                    let (sys, plot) = recalculate_curve(&self.params);
                    self.sys = sys;
                    self.plot = plot;
                }
            });
            return;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            Plot::new(self.params.build_str())
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

fn recalculate_curve(params: &SimulationParams) -> (ContinuousSystem, Vec<Vec<f32>>) {
    let eq_str = params.build_str();
    let SimulationParams {
        initial_cnc,
        k_f,
        k_r,
        time_step,
        stop,
        ..
    } = params;
    let mut eq = Equation::new(&eq_str).unwrap();
    assert!(eq.is_balanced());
    eq.set_concentrations(initial_cnc).unwrap();
    let mut sys = ContinuousSystem::new(eq, *k_f, *k_r);

    let plot = sys.solve(*time_step, *stop);

    (sys, plot)
}

#[derive(Clone)]
struct SimulationParams {
    eq_left: String,
    eq_right: String,
    eq_dir: Direction,
    initial_cnc: Vec<f32>,
    k_f: f32,
    k_r: f32,
    time_step: Duration,
    stop: Duration,
}

impl SimulationParams {
    fn build_str(&self) -> String {
        format!("{} {} {}", self.eq_left, self.eq_dir, self.eq_right)
    }

    fn dir_str(&self) -> &'static str {
        match self.eq_dir {
            Direction::Left => "←",
            Direction::Right => "→",
            Direction::Reversible => "⇌",
        }
    }
}

/// Taken from [BfBB-Clash](https://github.com/BfBB-Clash/BfBB-Clash/blob/ab54cf46c4beb23588b403cf20a0fc06d7d/crates/clash/src/gui/val_text.rs)
#[allow(unused)]
mod validator {
    use std::fmt::Display;

    use egui::TextBuffer;

    pub struct ValText<T: 'static, F> {
        text: String,
        val: Option<T>,
        validator: F,
    }

    impl<T: Copy, F: Fn(&str) -> Option<T>> ValText<T, F> {
        pub fn with_validator(validator: F) -> Self {
            Self {
                text: Default::default(),
                val: Default::default(),
                validator,
            }
        }

        pub fn get_val(&self) -> Option<T> {
            self.val
        }

        pub fn is_valid(&self) -> bool {
            self.val.is_some()
        }
    }

    impl<T: Display, F> ValText<T, F> {
        pub fn set_val(&mut self, val: T) {
            self.text = val.to_string();
            self.val = Some(val);
        }
    }

    impl<T, F> TextBuffer for ValText<T, F>
    where
        F: Fn(&str) -> Option<T>,
    {
        fn is_mutable(&self) -> bool {
            true
        }

        fn as_str(&self) -> &str {
            self.text.as_str()
        }

        fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
            let n = self.text.insert_text(text, char_index);
            self.val = (self.validator)(&self.text);
            n
        }

        fn delete_char_range(&mut self, char_range: std::ops::Range<usize>) {
            self.text.delete_char_range(char_range);
            self.val = (self.validator)(&self.text);
        }
    }
}
