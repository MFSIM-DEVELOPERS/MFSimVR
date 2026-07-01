use mfsimvr_common::MFSIMVR_VERSION;
use eframe::egui::{RichText, Ui};

pub fn about_tab_ui(ui: &mut Ui) {
    ui.label(RichText::new(format!("MFSimVR streamer v{}", *MFSIMVR_VERSION)).size(30.0));
}
