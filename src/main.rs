#[macro_use] extern crate rocket;

#[get("/patientData/<name>")]
fn patient_data(name: &str) -> String {
    format!("retrieving patient data for {}", name) 
}

#[get("/")]
fn index() -> &'static str {
    "Welcome to the doctor's page!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, patient_data])
}