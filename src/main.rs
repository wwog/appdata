use app_data::AppData;

fn main() {
    let app_data = AppData::default();
    println!("app_data: {:?}", app_data.ensure_data_dir().unwrap());
}
