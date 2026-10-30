//! Shared Adotme mark, decoded once per egui context and reused at every size.
use egui::{Color32, Id, Rect, TextureHandle, TextureOptions, Ui, pos2};
const RGBA: &[u8] = include_bytes!("../../../assets/app-icon/adotme-mark.rgba");
fn texture(ui: &Ui) -> TextureHandle {
    let id = Id::unique("adotme-brand-mark");
    if let Some(texture) = ui.ctx().data(|data| data.get_temp::<TextureHandle>(id)) {
        return texture;
    }
    let pixels = egui::ColorImage::from_rgba_unmultiplied([128, 128], RGBA);
    let texture = ui.ctx().load_texture("adotme-brand-mark", pixels, TextureOptions::LINEAR);
    ui.ctx().data_mut(|data| data.insert_temp(id, texture.clone()));
    texture
}
pub fn paint_mark(ui: &Ui, rect: Rect) {
    let uv = Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0));
    ui.painter().image(texture(ui).id(), rect, uv, Color32::WHITE);
}
pub fn heading(ui: &mut Ui, name: &str, color: Color32) {
    ui.horizontal(|ui| {
        let (rect, _) = ui.allocate_exact_size(egui::vec2(48.0, 48.0), egui::Sense::hover());
        paint_mark(ui, rect);
        ui.label(egui::RichText::new(name).size(38.0).strong().color(color));
    });
}
