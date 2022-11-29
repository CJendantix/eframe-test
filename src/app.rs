#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]

pub struct Calculator {
    //Peristant Variables
}


impl Default for Calculator {
    fn default() -> Self {
        Self {
            //Instanciate Persitant variables
        }
    }
}


impl Calculator {
    //Persistance
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}


impl eframe::App for Calculator {
    /// Save State Before Shutdown
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {} = self;


        egui::SidePanel::right("side_panel").show(ctx, |ui| {

        });


        egui::CentralPanel::default().show(ctx, |_ui| {

        });
    }
}
