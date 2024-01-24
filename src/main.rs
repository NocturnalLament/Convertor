use inquire::Text;
#[path = "Conversions/Temperature/InitialData.rs"] mod TempType;
enum Conversion {
    Temperature,
    Length,
}

fn main() {
    TempType::get_data_type();
}
