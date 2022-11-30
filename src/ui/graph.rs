use bevy_egui::egui;

use crate::AppState;

pub fn graph(ui: &mut egui::Ui, app_state: &mut AppState) {
    ui.heading("Concentrations");
    ui.add_space(10.0);
    let Ok(eq) = &mut app_state.eq_res else {
        return;
    };

    for (name, cmp) in eq.name_and_concentration_mut() {
        use egui::plot::{Line, Plot, PlotPoints};

        ui.label(&name);
        let series: PlotPoints = (0..1000)
            .map(|i| {
                let x = i as f64 * 0.01;
                [x, x.sin()]
            })
            .collect();
        let line = Line::new(series);
        Plot::new(name)
            .view_aspect(2.0)
            .show(ui, |plot_ui| plot_ui.line(line));

        ui.add(egui::Slider::new(cmp, 0.0..=20.0));
        ui.add_space(20.0);
    }
}
