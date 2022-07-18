use macroquad::prelude::*;

//This is a vector containing the size of players bar, Width = 150 and height = 40
const PLAYER_BAR_SIZE: Vec2 = const_vec2!([150f32 , 32f32]);

//Speed of player's bar
const PLAYER_SPEED : f32 = 750f32;

//Blocks to destroy sizes
const BLOCK_SIZE : Vec2 = const_vec2!([80f32 , 32f32]);

const BALL_SPEED :f32 = 400f32;
const BALL_SIZE : f32 = 50f32;

#[macroquad::main("breakout")]
async fn main() {

    let font = load_ttf_font("res/Roboto-Medium.ttf").await.unwrap();

    let mut score = 0;

    let mut player = Player::new();
    let mut blocks : Vec<Block> = Vec::new();
    let mut balls : Vec<Ball> = Vec::new();


    let (width_num_of_blocks, height_num_of_blocks) = (6,6);

    let padding = 5f32; //Separation between blocks
    let total_block_size = BLOCK_SIZE + vec2(padding,padding);
    //Upper left corner of the array of blocks
    //Lots of maths: widthOfScreen - (numOfBlocks * itsSize), this is the empty part of the screen, divide it by 2, so we
    //center the array of blocks.
    let starting_blocks_point = vec2((screen_width() - (total_block_size.x * width_num_of_blocks as f32)) * 0.5f32, 50f32);


    //Fill matrix of blocks
    for i in 0..width_num_of_blocks * height_num_of_blocks{
        //x position of the block: 0,1,2,3,4,5,0,1,2,3,4,5....
        let block_x = (i % width_num_of_blocks) as f32 * total_block_size.x;

        let block_y = (i / width_num_of_blocks) as f32 *total_block_size.y; 

        //Push a new block with the calculated coordinates
        blocks.push(Block::new(starting_blocks_point + vec2(block_x,block_y)));
    }


    balls.push(Ball::new(vec2(screen_width() * 0.5f32, screen_height()*0.5f32)));

    loop{

        if is_key_pressed(KeyCode::Space){
            balls.push(Ball::new(vec2(screen_width() * 0.5f32, screen_height()*0.5f32)));
        }

        player.update(get_frame_time());

       

        for ball in balls.iter_mut(){

            ball.update(get_frame_time());

            //Collision between ball and player
            resolve_collision(&mut ball.square, &mut ball.velocity, &mut player.rectangle);

            //Collision between ball and block
            for block in blocks.iter_mut() {
                if resolve_collision(&mut ball.square, &mut ball.velocity, &block.rectangle) {
                    block.lives -= 1;
                    score += 10;
                }
            }

        }

        //Retain function, if lambda expression is true, the element remains in the list, if not, its removed.
        blocks.retain(|block| block.lives >0);

        clear_background(WHITE);
        
        player.draw();

        for block in blocks.iter() {
            block.draw();
        }

        for ball in balls.iter() {
            ball.draw();
        }


        let score_text = format!("score: {}", score);
        let score_text_dim = measure_text(&score_text,  Some(font), 20u16, 1.0);

        draw_text_ex(
            &format!("score: {}", score), 
            screen_width()*0.5f32 - score_text_dim.width*0.5f32 ,
            40.0,
            TextParams{font, font_size: 20u16, color:BLACK, ..Default::default()}
        );

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


struct Block {
    rectangle : Rect,
    lives: i32,
}

impl Block {

    pub fn new(pos: Vec2) -> Self {

        Self{
            rectangle: Rect::new(
                pos.x,
                pos.y, 
                BLOCK_SIZE.x,
                BLOCK_SIZE.y),
            lives : 2,
        }

    }


    pub fn draw(&self) {

        let color = match self.lives {
            2 => RED,
            _ => ORANGE,
        };

        draw_rectangle(
            self.rectangle.x, 
            self.rectangle.y, 
            self.rectangle.w,
            self.rectangle.h, 
            color)
    }

}


pub struct Ball{
    square: Rect,
    velocity: Vec2,
}

impl Ball{

    pub fn new(pos: Vec2) -> Self {
        Self{
            square: Rect::new(pos.x,pos.y, BALL_SIZE,BALL_SIZE),
            velocity : vec2(rand::gen_range(-1f32,1f32) , 1f32).normalize()
        }
    }


    pub fn draw(&self){
        draw_rectangle(self.square.x, self.square.y, self.square.w, self.square.h, RED)
    }

    

    pub fn update(&mut self , dt : f32){
        self.square.x += self.velocity.x * dt * BALL_SPEED;
        self.square.y += self.velocity.y * dt * BALL_SPEED;

        if self.square.x < 0f32 {
        
            
            self.velocity.x = 1f32;
        }

        if self.square.x > screen_width() - self.square.w {
           
            
            self.velocity.x = -1f32;
        }

        if self.square.y < 0f32{
            
            self.velocity.y = 1f32;
        }
    }
}


fn resolve_collision(a: &mut Rect, vel: &mut Vec2, b: &Rect) -> bool {

    if let Some(intersection) = a.intersect(*b){
        
        let a_center = a.point() + a.size() * 0.5f32;
        let b_center = b.point() + b.size() * 0.5f32;

        let to = b_center - a_center;
        let to_signum = to.signum();

        match intersection.w > intersection.h {

            true => {
           

                //width is higher than the height, rebound ball in Y axis
                a.y -= to_signum.y * intersection.h;
                vel.y = -to_signum.y * vel.y.abs();

            }
        false=> {

                //rebound on X axis
                a.x -= to_signum.x * intersection.w;
                vel.x = -to_signum.x * vel.x.abs();
        
        }
    }
        return true;

    }else{
        return false;
    }

}