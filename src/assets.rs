use ggez::{
    graphics,
    audio,
    Context,
    GameResult,
};

pub struct Assets {
    pub player_base: graphics::Image,
    pub player_damaged: graphics::Image, 
    pub player_dead: graphics::Image,
    pub player_shoot_north: graphics::Image, 
    pub player_shoot_south: graphics::Image, 
    pub player_shoot_west: graphics::Image, 
    pub player_shoot_east: graphics::Image, 
    pub shot_puke_base: graphics::Image,
    pub shot_blood_base: graphics::Image,
    pub enemy_mask_base: graphics::Image,
    pub door_closed: graphics::Image,
    pub door_open: graphics::Image,
    pub floor: graphics::Image,
    pub wall: graphics::Image,
    pub stone: graphics::Image,
    pub heart_full: graphics::Image,
    pub heart_half: graphics::Image,
    pub heart_empty: graphics::Image,

    pub button_font: graphics::Font,

    pub death_sound: audio::Source,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let player_base = graphics::Image::new(ctx, "/player_base.png")?;
        let player_damaged = graphics::Image::new(ctx, "/player_damaged.png")?; 
        let player_dead = graphics::Image::new(ctx, "/player_dead.png")?; 
        let player_shoot_north = graphics::Image::new(ctx, "/player_shoot_north.png")?;
        let player_shoot_south = graphics::Image::new(ctx, "/player_shoot_south.png")?;
        let player_shoot_west = graphics::Image::new(ctx, "/player_shoot_west.png")?;
        let player_shoot_east = graphics::Image::new(ctx, "/player_shoot_east.png")?;
        let shot_puke_base = graphics::Image::new(ctx, "/shot_puke_base.png")?;
        let shot_blood_base = graphics::Image::new(ctx, "/shot_blood_base.png")?;
        let enemy_mask_base = graphics::Image::new(ctx, "/enemy_mask_base.png")?;
        let door_closed = graphics::Image::new(ctx, "/door_closed.png")?;
        let door_open = graphics::Image::new(ctx, "/door_open.png")?;
        let floor = graphics::Image::new(ctx, "/floor.png")?;
        let wall = graphics::Image::new(ctx, "/wall.png")?;
        let stone = graphics::Image::new(ctx, "/stone.png")?;
        let heart_full = graphics::Image::new(ctx, "/heart_full.png")?;
        let heart_half = graphics::Image::new(ctx, "/heart_half.png")?;
        let heart_empty = graphics::Image::new(ctx, "/heart_empty.png")?;

        let button_font = graphics::Font::new(ctx, "/enigma.ttf")?;

        let death_sound = audio::Source::new(ctx, "/death_sound.mp3")?;

        Ok(Self {
            player_base,
            player_damaged,
            player_dead,
            player_shoot_north,
            player_shoot_south,
            player_shoot_west, 
            player_shoot_east, 
            shot_puke_base, 
            shot_blood_base,
            enemy_mask_base,
            door_closed,
            door_open,
            floor,
            wall,
            stone,
            heart_full,
            heart_half,
            heart_empty,           

            button_font,

            death_sound,
        })
    }
}
