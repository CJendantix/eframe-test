fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Calculator",
        native_options,
        Box::new(|cc| Box::new(eframe_test::Calculator::new(cc))),
    );
}