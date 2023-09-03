pub mod back_of_house   {
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_toast: String
    }

    impl Breakfast {
        pub fn make_breakfast(toast: &str) -> Self {
            Self {
                toast: String::from(toast),
                seasonal_toast: String::from("Peaches")
            }
        }
    }
}

pub fn eat_at_restaurant(){
    let mut breakfast = back_of_house::Breakfast::make_breakfast("Honeybuns");
    breakfast.toast = String::from("Cornflakes");
    println!("Breakfast {:?}",breakfast)

}