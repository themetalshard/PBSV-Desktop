// RIP:
// TheMetalShard (developer)
// Primative_11 (main tester)
// LunarThePr0t0g3n
// Boltazon
// Kyguy329 (Mac version)
// TheSkout001 (for ideas)
//
// Well, thanks ATTG!

use crate::{
    fetch::{fetch_schedule, Schedule},
    model::ScheduleEvent,
};

use chrono::{Datelike, Duration, Local, TimeZone, Utc};
use eframe::egui;

#[derive(PartialEq)]
enum ViewMode {
    List,
    Calendar,
}

pub struct ScheduleApp {
    schedule: Schedule,
    active_group: String,
    selected_event: Option<ScheduleEvent>,
    show_about: bool,
    view_mode: ViewMode,
}

impl Default for ScheduleApp {
    fn default() -> Self {
        let schedule = fetch_schedule().unwrap_or_default();

        let active_group = schedule
            .keys()
            .next()
            .cloned()
            .unwrap_or_else(|| "PBST".to_string());

        Self {
            schedule,
            active_group,
            selected_event: None,
            show_about: false,
            view_mode: ViewMode::List,
        }
    }
}

impl eframe::App for ScheduleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {

                for group in self.schedule.keys() {
                    let selected = self.active_group == *group && !self.show_about;
                    if ui.selectable_label(selected, group.to_uppercase()).clicked() {
                        self.active_group = group.clone();
                        self.show_about = false;
                        self.selected_event = None;
                    }
                }

                ui.separator();

                egui::ComboBox::from_id_source("view_selector")
                    .selected_text(match self.view_mode {
                        ViewMode::List => "List",
                        ViewMode::Calendar => "Calendar",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.view_mode, ViewMode::List, "List");
                        ui.selectable_value(&mut self.view_mode, ViewMode::Calendar, "Calendar");
                    });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let selected = self.show_about;
                    if ui.selectable_label(selected, "About").clicked() {
                        self.show_about = true;
                        self.selected_event = None;
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {

            if self.show_about {
                show_about(ui);
                return;
            }

            match self.view_mode {
                ViewMode::List => {
                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .show(ui, |ui| {

                            if let Some(events) = self.schedule.get(&self.active_group) {
                                let mut events = events.clone();
                                events.sort_by_key(|e| e.time);

                                for event in events {
                                    if event_card(ui, &event) {
                                        self.selected_event = Some(event);
                                    }
                                    ui.add_space(6.0);
                                }
                            } else {
                                ui.label("No events found.");
                            }
                        });
                }

                ViewMode::Calendar => {
                    show_calendar_view(
                        ui,
                        self.schedule.get(&self.active_group),
                        &mut self.selected_event,
                    );
                }
            }
        });

        if let Some(event) = self.selected_event.clone() {
            let mut open = true;

            egui::Window::new("Event details")
                .collapsible(false)
                .resizable(false)
                .open(&mut open)
                .show(ctx, |ui| {

                    let local = Local.timestamp_opt(event.time, 0).unwrap();
                    let utc = Utc.timestamp_opt(event.time, 0).unwrap();

                    ui.heading(&event.event_type);
                    ui.separator();

                    ui.label(format!("Local start: {}", local));
                    ui.label(format!("UTC start: {}", utc));
                    ui.label(format!("Unix timestamp: {}", event.time));
                    ui.label(format!("Duration: {} minutes", event.duration));

                    ui.separator();

                    if let Some(trainer) = &event.trainer {
                        ui.label(format!("Host: {}", trainer));
                    }

                    if let Some(notes) = &event.notes {
                        ui.label(format!("Notes: {}", notes));
                    }

                    ui.separator();

                    if let Some(uuid) = &event.uuid {
                        ui.label(format!("UUID: {}", uuid));
                    }

                    if let Some(id) = event.trainer_id {
                        ui.label(format!("Trainer ID: {}", id));
                    }

                    if let Some(id) = &event.discord_id {
                        ui.label(format!("Discord ID: {}", id));
                    }
                });

            if !open {
                self.selected_event = None;
            }
        }
    }
}

fn event_card(ui: &mut egui::Ui, event: &ScheduleEvent) -> bool {
    let color = event
        .event_color
        .map(|c| egui::Color32::from_rgb(c[0], c[1], c[2]))
        .unwrap_or(egui::Color32::GRAY);

    let start = Local.timestamp_opt(event.time, 0).unwrap();
    let end = start + Duration::minutes(event.duration);

    let frame = egui::Frame::group(ui.style())
        .fill(ui.visuals().extreme_bg_color)
        .stroke(egui::Stroke::new(
            1.0,
            ui.visuals().widgets.noninteractive.bg_stroke.color,
        ))
        .rounding(egui::Rounding::same(6.0))
        .inner_margin(egui::Margin::same(8.0));

    let response = frame
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("▌").color(color));

                ui.vertical(|ui| {
                    ui.label(format!(
                        "{} – {}",
                        start.format("%H:%M"),
                        end.format("%H:%M")
                    ));

                    ui.strong(&event.event_type);

                    if let Some(trainer) = &event.trainer {
                        ui.label(format!("Host: {}", trainer));
                    }

                    if let Some(notes) = &event.notes {
                        ui.weak(notes);
                    }
                });
            });
        })
        .response
        .interact(egui::Sense::click());

    response.clicked()
}

fn show_calendar_view(
    ui: &mut egui::Ui,
    events: Option<&Vec<ScheduleEvent>>,
    selected_event: &mut Option<ScheduleEvent>,
) {
    let Some(events) = events else {
        ui.label("No events found.");
        return;
    };

    let mut by_day: std::collections::BTreeMap<(i32, u32, u32), Vec<ScheduleEvent>> =
        std::collections::BTreeMap::new();

    for event in events {
        let dt = Local.timestamp_opt(event.time, 0).unwrap();
        by_day
            .entry((dt.year(), dt.month(), dt.day()))
            .or_default()
            .push(event.clone());
    }

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            
            egui::ScrollArea::horizontal()
                .auto_shrink([false, false])
                .show(ui, |ui| {

                    ui.horizontal(|ui| {
                        for ((y, m, d), mut day_events) in by_day {
                            day_events.sort_by_key(|e| e.time);

                            let date =
                                Local.with_ymd_and_hms(y, m, d, 0, 0, 0).unwrap();

                            ui.allocate_ui_with_layout(
                                egui::vec2(220.0, ui.available_height()),
                                egui::Layout::top_down(egui::Align::LEFT),
                                |ui| {
                                    ui.heading(date.format("%a %d/%m").to_string());
                                    ui.separator();

                                    for event in day_events {
                                        if event_card(ui, &event) {
                                            *selected_event = Some(event);
                                        }
                                        ui.add_space(6.0);
                                    }
                                },
                            );
                        }
                    });
                });
        });
}

fn show_about(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("PB Schedule Viewer");
        ui.label("Version: 0.1.0");
        ui.add_space(10.0);

        ui.label("A cross-platform schedule viewer for Pinewood built with Rust and egui.");
        ui.add_space(10.0);

        ui.separator();
        ui.add_space(10.0);

        ui.label("Dependencies:");
        ui.monospace("• eframe / egui");
        ui.monospace("• chrono");
        ui.monospace("• reqwest");
        ui.monospace("• serde");

        ui.add_space(20.0);
        ui.weak("Smiley!");
    });
}

