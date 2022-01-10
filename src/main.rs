use snake_game::*;

fn main() {
    start_screen();
    run();
}

fn start_screen() {
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    // Please note that I used an ascii generator, I'm shameless
    print!(
        r#"
        _________              __              ________                       
        /   _____/ ____ _____  |  | __ ____    /  _____/_____    _____   ____  
        \_____  \ /    \\__  \ |  |/ // __ \  /   \  ___\__  \  /     \_/ __ \ 
        /        \   |  \/ __ \|    <\  ___/  \    \_\  \/ __ \|  Y Y  \  ___/ 
        /_______  /___|  (____  /__|_ \\___  >  \______  (____  /__|_|  /\___  >
                \/     \/     \/     \/    \/          \/     \/      \/     \/ "#
    );
    std::thread::sleep(std::time::Duration::from_secs(2));
}
