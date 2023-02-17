enum Action {
    Timer(u32),
    Music(String),
    Weather(String),
}

fn dispatch(action: Action) {
    match action {
        Action::Timer(time) => println!("AI -- Timer requested {}", time),
        Action::Music(song) => println!("AI -- Play following song requested {}", song),
        Action::Weather(city) => println!("AI -- Weather for this city {} requested", city),
    }
}

fn main() {
    dispatch(Action::Timer(32));
    dispatch(Action::Music("Fly -- Ludovico Einaudi".to_string()));
    dispatch(Action::Weather("Sedona - Arizona".to_string()));
} 
