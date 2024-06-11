pub struct Player {
    pub name: String,
    pub rating: Option<u32>,
    pub active: bool
}

impl Player {

    pub fn new(name: String, rating: Option<u32>) -> Self {

        let player = Self {
            name,
            rating,
            active: true
        };

        player

    }
}