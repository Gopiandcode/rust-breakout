# Rust Breakout
Shameless Rust based translation of tutorial at [LearnOpengl](https://learnopengl.com/In-Practice/2D-Game/Breakout).

Actually had to change quite a bit of stuff to use rust in this project - 
might even be worthwhile writing up a tutorial for creating opengl games in rust.  

Cool things in this project:
- Using SDL2 and OpenGL to render to the screen.
- Using rust-bindgen and build scripts to link to arbitrary c-projects - specifically SOIL.
- Identifying where and when to use references vs Rc<RefCell<>>
- Structuring larger scale projects in rust's module system
- Doing OOP without inheritance and excessive performance losses.

## Screenshots
