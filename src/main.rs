use macroquad::prelude::*;
mod player;
mod pipe;

#[macroquad::main("gravedad")]
async fn main() {
    let mut player = player::Player::new();
    let mut pipe = pipe::Pipe::new();
    let mut pipe2 = pipe::Pipe::new();
    let mut pipe3 = pipe::Pipe::new();

    loop { // shitty game loop
        player.update(&pipe, &pipe2, &pipe3);
        player.draw();
        pipe.draw();
        pipe2.draw();
        pipe3.draw();
        pipe.update(10.0);
        pipe2.update(7.0);
        pipe3.update(12.0);
        draw_rectangle(
            0.0,screen_height()/2.0 + 200.0,
            screen_width(),200.0,ORANGE
        );
        if player.rect.y >= screen_height() / 2.0 + 200.0
         || player.rect.y <= -200.0 { break;} 
        if is_key_down(KeyCode::Q) {break;}
        next_frame().await;
    }
}
