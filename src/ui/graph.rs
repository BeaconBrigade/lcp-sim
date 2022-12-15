use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::ui::UiState;

pub fn graph(mut egui_context: ResMut<EguiContext>, mut ui_state: ResMut<UiState>) {
    egui::SidePanel::left("equation graphs").show(egui_context.ctx_mut(), |ui| {
        ui.heading("Concentrations");
        ui.add_space(10.0);

        let UiState {
            eq_constant,
            eq_res,
            ..
        } = &mut *ui_state;
        let Ok(eq) = &mut eq_res.0 else {
            return;
        };

        // set temperature of reaction
        let mut temp = eq.temperature().unwrap_or(0.0);
        ui.add(egui::Slider::new(&mut temp, 0.0..=200.0).text("Temperature (°C)"));
        eq.set_temperature(temp);

        // set volume of reaction
        let mut vol = eq.volume().unwrap_or(1.0);
        ui.add(egui::Slider::new(&mut vol, 0.0..=100.0).text("Volume (L)"));
        eq.set_volume(vol);

        // set equilibrium constant for reaction
        ui.add(egui::Slider::new(eq_constant, 0.0..=10.0).text("Equilibrium constant"));

        let len = eq.num_compounds();
        // vertical scroll so graphs don't overflow the window
        egui::ScrollArea::vertical().show(ui, |ui| {
            for (i, (name, cmp)) in eq.name_and_concentration_mut().enumerate() {
                use egui::plot::{Line, Plot, PlotPoints};

                ui.label(&name);
                let series: PlotPoints = (0..1000)
                    .map(|i| {
                        let x = i as f64 * 0.01;
                        [x, x.sin()]
                    })
                    .collect();
                let line = Line::new(series);
                Plot::new(i)
                    .view_aspect(2.0)
                    .allow_scroll(false)
                    .show(ui, |plot_ui| plot_ui.line(line));

                ui.add(egui::Slider::new(cmp, 0.0..=2.0));

                // this prevents some weird spazzing in the [`ScrollArea`]
                if i < len - 1 {
                    ui.add_space(20.0);
                }
            }
        });
    });
}
