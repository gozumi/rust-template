pub struct Results {
    pub bar: String
}

impl Results {
    pub fn new() -> Results {
        Results { bar: "It is working!!!!".to_string() }
    }
}
