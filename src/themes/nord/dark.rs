//! Nord Light theme
//! <https://www.nordtheme.com/>

use crate::Aesthetix;

/// Dark theme, with rounded buttons, and ample margin. Adapted from the Nord light
/// color scheme.
pub struct NordDark;

impl Aesthetix for NordDark {
    fn name(&self) -> &str {
        "Nord Dark"
    }

    fn primary_accent_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(94, 129, 172)
    }

    fn secondary_accent_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(129, 161, 193)
    }

    fn bg_primary_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(46, 52, 64)
    }

    fn bg_secondary_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(59, 66, 82)
    }

    fn bg_triage_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(59, 66, 82)
    }

    fn bg_auxiliary_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(59, 66, 82)
    }

    fn bg_contrast_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(59, 66, 82)
    }

    fn fg_primary_text_color_visuals(&self) -> Option<egui::Color32> {
        Some(egui::Color32::from_rgb(216, 222, 233))
    }

    fn fg_success_text_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(80, 250, 123)
    }

    fn fg_warn_text_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(255, 215, 64)
    }

    fn fg_error_text_color_visuals(&self) -> egui::Color32 {
        egui::Color32::from_rgb(255, 121, 121)
    }

    fn dark_mode_visuals(&self) -> bool {
        true
    }
}
