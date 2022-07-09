# Snake Game

A command line snake game written in rust. In theory works on ANSI terminals
(unix/macOS), however I can only guarantee that it runs on Linux. 

# Usage

Run the game as normal, using `cargo` or however you please. Controls are the
arrow keys or WASD, which determine what direction you'll attempt to turn on the
next frame. Press `q` to quit the game.

# Features

- ASCII art intro, game-over, and victory screens
- Colorful UI using `termion`
- Score tracking
- Snake is run as a circular array - see `snake.rs` if you're curious
- Game over if snake bumps itself or goes out-of-bounds
- Food is placed in random places
- Can't turn snake back in direction it just came from

# Todo
- [x] add start screen 
- [x] clean up control flow
- [x] add end screen 
- [x] food generation 
- [x] add victory screen 
- [ ] increase game speed when food is eaten
- [ ] re-write in some TUI library?
- [ ] optimize frame rendering (i.e. don't reprint entire board every frame)
