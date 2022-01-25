#[macro_use] extern crate rocket;

#[get("/patientData/treatment/<name>")]
fn patient_data(name: &str) -> String {
    let treatment = get_patient_data(name);
    format!("Treatment is {} ", treatment) 
}

fn get_patient_data(name: &str) -> String {
    format!("Ergo Therapy for {}, probably", name)
}

#[get("/")]
fn index() -> &'static str {
    "Welcome to the doctor's page!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, patient_data])
}