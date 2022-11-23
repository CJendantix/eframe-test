fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "eframe test",
        native_options,
        Box::new(|cc| Box::new(eframe_test::TemplateApp::new(cc))),
    );
}