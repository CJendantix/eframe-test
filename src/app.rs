
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]

// if we add new fields, give them default values when deserializing old state
#[serde(default)]


pub struct Calculator {
    // Create Variables
}


// Turn struct: Calculator into template: Default
impl Default for Calculator {
    fn default() -> Self {
        Self {
            //Instanciate Variables
        }
    }
}


impl Calculator {
    /// Persistance, Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}


impl eframe::App for Calculator {


    /// Called by the frame work to save state before shutdown.
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
