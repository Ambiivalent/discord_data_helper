use serde_json::{Result, Value};
use eframe::{run_native, NativeOptions, egui};
use egui_extras::RetainedImage;
use image::io::Reader as ImageReader;
fn main() -> eframe::Result<()>{
// GUI starts here
    let options = NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 220.0)),
        ..Default::default()
    };
    run_native(
        "Discord Data Analyzer",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )

}

struct MyApp {
    text: String,
    username: String,
    id: String,
    pfp: RetainedImage,
    Message_count: u32,
    server_ids: Vec<String>,
    dm_ids: Vec<String>,
}

impl Default for MyApp {
    fn default() -> Self {
        // default setup for app
        let index_dir = "package/messages/index.json";
        let user_dir = "package/account/user.json";
        let user_pfp = "package/account/avatar.png";

        // read data
        let text = match std::fs::read_to_string(index_dir) {
            Ok(file) => file,
            Err(error) => panic!("File not found {:?}", error),
        };

        let usertext = match std::fs::read_to_string(user_dir) {
            Ok(file) => file,
            Err(error) => panic!("File not found {:?}", error),
        };
        // get data as serde value
        let msg_data: Value = serde_json::from_str(&text).unwrap();
        let user_data: Value = serde_json::from_str(&usertext).unwrap();

        let mut username = user_data.get("username").unwrap().to_string();
        let mut id = user_data.get("id").unwrap().to_string();

        // remove quotations from names
        username = username.replace('\"', "");
        id = id.replace('\"', "");

        // load pfp data
        /* 
        let image = ImageReader::open(user_pfp).unwrap().decode();
        let pfp_data: RetainedImage = match RetainedImage::from_image_bytes("avatar.png", image.unwrap().as_bytes()) {
            Ok(T) => T,
            Err(error) => panic!("Failure to load image {:?}", error)
        };
        */
        let pfp_data = RetainedImage::from_image_bytes("avatar.png", include_bytes!("../package/account/avatar.png")).unwrap();

        let mut server_ids: Vec<&str> = Vec::new();
        let mut dm_ids: Vec<&str> = Vec::new();
    
        // Get Messages
        for (name, obj) in msg_data.as_object().unwrap().iter() {
            if !(obj.is_null() || match obj.as_str() { Some(x) => x,None => "",}.starts_with("Direct Message")) {
                server_ids.push(name);
            }
            else if match obj.as_str() { Some(x) => x,None => "",}.starts_with("Direct Message") {
                dm_ids.push(name)
            }
        }
    
        let total_msg = get_total_msg(&server_ids, &dm_ids);
        // return struct 
        Self {
            text: "".to_owned(),
            username: username,
            id: id,
            pfp: pfp_data,
            Message_count: total_msg,
            server_ids: server_ids.into_iter().map(|x| x.to_owned()).collect(),
            dm_ids: dm_ids.into_iter().map(|x| x.to_owned()).collect()
        }
    }
}


impl eframe::App for MyApp {
    // ui updates happen here
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Discord Data Analyzer");
            /* 
            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                self.pfp.show(ui);
            }); */
            ui.label(format!("Username: {}", self.username));
            ui.label(format!("Discord ID: {}", self.id));
            ui.label(format!("Hello {}! I found a total of {} typed messages on Discord since you requested your data.", self.username, self.Message_count));
        });
        egui::SidePanel::right("right_panel").show(ctx, |ui| {
            self.pfp.show(ui);
         });
    }
}

fn get_total_msg(server_id: &Vec<&str>, dm_id: &Vec<&str>) -> u32{
    // function returns total messages of dms and servers
    // get file name -> read every record
    let mut count: u32 = 0;
    let all_ids: Vec<&str> = [server_id.to_owned(), dm_id.to_owned()].concat();
    for values in all_ids {
        let file_path = "package/messages/c".to_owned() + values + "/messages.csv";
        let mut rdr = match csv::Reader::from_path(&file_path) {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file {filename} {:?}", error, filename = file_path),
        };
        count = count + rdr.records().count() as u32;
    }
    count
}

fn get_instanceof(message: String, server_id: Vec<&str>, dm_id: Vec<&str>) {
    // to work on
}

fn get_msgcount_givenid(id: &str) -> i32{
    // to work on
    let mut count: i32 = 0;
    let file_path = "package/messages/c".to_owned() + id + "/messages.csv";
    let mut rdr = match csv::Reader::from_path(&file_path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file {filename} {:?}", error, filename = file_path),
    };
    for result in rdr.records() {
        count += 1;
    }
count
}