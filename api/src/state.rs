// struct Vote {
//     proof: todo!("proof type"),
//     gov_key: todo!("public key of gov")
// }

// struct Election {
//     gov_key: todo!("public key of gov"),
//     gov_sigs: Vec<todo!("sigs")>,
//     options: Vec<String>,
//     votes: Vec<Vote>,
// }

// #[derive(Clone)]
// pub struct AppState {
//     elections: Vec<Election>
// }

// impl AppState {
//     pub fn new() -> Self {
//         AppState {
//             data: Vec::new()
//         }
//     }
// }

#[derive(Clone)]
pub struct AppState {
    data: Vec<i8>
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            data: Vec::new()
        }
    }
}