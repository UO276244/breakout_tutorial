use macroquad::prelude::*;

//This is a vector containing the size of players bar, Width = 150 and height = 40
const PLAYER_BAR_SIZE: Vec2 = const_vec2!([150f32 , 40f32]);

//Speed of player's bar
const PLAYER_SPEED : f32 = 750f32;



#[macroquad::main("breakout")]
async fn main() {

    let mut player = Player::new();

    loop{

        player.update(get_frame_time());

        clear_background(WHITE);
        
        player.draw();

        next_frame().await;

    }
}

//Player strcut containing player's rectangle
struct Player {

    rectangle: Rect

}

impl Player {

    //Constructor por Player, assigns "rectangle" attribute in struct to a new created Rect
    pub fn new() -> Self {
        Self {

            rectangle:Rect::new(
                screen_width() * 0.5f32 - PLAYER_BAR_SIZE.x*0.5f32,//Top left corner x position --> named x ; Subtract half width of rectangle to center
                screen_height() - 100f32,//Top left corner y position --> named y
                PLAYER_BAR_SIZE.x, //Width of rectangle --> named w
                PLAYER_BAR_SIZE.y //Height of rectangle --> named h
            )

        }
    }

    //Draw player's rectangle
    pub fn draw(&self){
        draw_rectangle(self.rectangle.x, self.rectangle.y, self.rectangle.w, self.rectangle.h, BLUE)
    }


    //dt means delta time
    pub fn update(&mut self, dt: f32){

        //Matching the pattern, if the left key is pressed, move to the left, if right key is pressed, move to the right.
        //If any other combination, do not move.
        let x_move = match (is_key_down(KeyCode::Left), is_key_down(KeyCode::Right)){
            (false, true) => 1f32,
            (true, false) => -1f32,
            _ => 0f32,
        };

        self.rectangle.x += x_move * dt * PLAYER_SPEED;

        
        //If rectangle moves away from the screen, put it back in the limits.
        if self.rectangle.x < 0f32 {
            self.rectangle.x = 0f32;
        }
        if self.rectangle.x > screen_width() - self.rectangle.w{
            self.rectangle.x = screen_width() - self.rectangle.w; 
        }

    }
}



