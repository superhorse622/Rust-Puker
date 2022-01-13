use ggez::{
    graphics::{self, Font, Color, DrawMode, DrawParam, Rect, Mesh, MeshBuilder, Text, PxScale},
    Context,
    GameResult,
    mint::{Point2},
    input,
};
use std::{
    any::Any,
};

use crate::{
    assets::*,
    utils::*,
    traits::*,
    consts::*,
    entities::{Player},
    dungeon::{Dungeon},
};

pub struct TextSprite {
    pub pos: Point2<f32>,
    pub text: String,
    pub font: Font,
    pub font_size: f32,
    pub color: Color,
}

impl TextSprite {
    pub fn get_text(&self, sh: f32) -> Text {
        let mut text = Text::new(self.text.as_str());
        text.fragments_mut().iter_mut().map(|f| {
            f.font = Some(self.font);
            f.scale = Some(PxScale::from(sh * self.font_size));
            f.color = Some(self.color);
        }).count();
        text
    }
}

impl UIElement for TextSprite {
    fn update(&mut self, _ctx: &mut Context, _conf: &Config) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, _assets: &Assets, conf: &Config) -> GameResult {
        let (sw, sh) = (conf.screen_width, conf.screen_height);
        let text = self.get_text(sh);
        let tl = self.top_left(ctx, sw, sh);

        graphics::draw(ctx, &text, DrawParam::default().dest([tl.x, tl.y]))?;

        Ok(())
    }

    fn pos(&self, sw: f32, sh: f32) -> Point2<f32> { Point2 { x: sw * self.pos.x, y: sh * self.pos.y } }

    fn width(&self, ctx: &mut Context, sh: f32) -> f32 { self.get_text(sh).width(ctx) as f32 }

    fn height(&self, ctx: &mut Context, sh: f32) -> f32 { self.get_text(sh).height(ctx) as f32 }

    fn top_left(&self, ctx: &mut Context, sw: f32, sh: f32) -> Point2<f32> {
        let pos = self.pos(sw, sh);
        let (w, h) = (self.width(ctx, sh), self.height(ctx, sh));
        Point2 { x: pos.x - w / 2., y: pos.y - h / 2. }
    }
        
    fn mouse_overlap(&self, ctx: &mut Context, sw: f32, sh: f32) -> bool {
        let tl = self.top_left(ctx, sw, sh);
        let (w, h) = (self.width(ctx, sh), self.height(ctx, sh));
        Rect::new(tl.x, tl.y, w, h).contains(input::mouse::position(ctx))
    }

    fn as_any(&self) -> &dyn Any { self }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

pub struct Button {
    pub pos: Point2<f32>,
    pub tag: State,
    pub text: TextSprite,
    pub color: Color,
}

impl UIElement for Button {
    fn update(&mut self, ctx: &mut Context, conf: &Config) -> GameResult {
        let (sw, sh) = (conf.screen_width, conf.screen_height);

        self.color = Color::WHITE;
        if self.mouse_overlap(ctx, sw, sh) {
            self.color = Color::RED;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, _assets: &Assets, conf: &Config) -> GameResult {
        let (sw, sh) = (conf.screen_width, conf.screen_height);
        let (tw, th) = (self.text.width(ctx, sh) as f32, self.text.height(ctx, sh) as f32);
        let tl = self.top_left(ctx, sw, sh);

        let btn = Mesh::new_rounded_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(tl.x, tl.y, tw, th),
            5.,
            self.color,
        )?;

        graphics::draw(ctx, &btn, DrawParam::default())?;
        self.text.draw(ctx, _assets, conf)?;

        Ok(())
    }

    fn pos(&self, sw: f32, sh: f32) -> Point2<f32> { Point2 { x: sw * self.pos.x, y: sh * self.pos.y } }

    fn width(&self, ctx: &mut Context, sh: f32) -> f32 { self.text.width(ctx, sh) as f32 }

    fn height(&self, ctx: &mut Context, sh: f32) -> f32 { self.text.height(ctx, sh) as f32 }

    fn top_left(&self, ctx: &mut Context, sw: f32, sh: f32) -> Point2<f32> {
        let pos = self.pos(sw, sh);
        let (w, h) = (self.width(ctx, sh), self.height(ctx, sh));
        Point2 { x: pos.x - w / 2., y: pos.y - h / 2. }
    }
        
    fn mouse_overlap(&self, ctx: &mut Context, sw: f32, sh: f32) -> bool {
        let tl = self.top_left(ctx, sw, sh);
        let (w, h) = (self.width(ctx, sh), self.height(ctx, sh));
        Rect::new(tl.x, tl.y, w, h).contains(input::mouse::position(ctx))
    }

    fn as_any(&self) -> &dyn Any { self }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

pub struct Minimap {
    pub pos: Point2<f32>,
    pub width: f32,
    pub height: f32,
    pub cur_room: (usize, usize),
    pub visited: [[usize; DUNGEON_GRID_COLS]; DUNGEON_GRID_ROWS],
}

impl UIElement for Minimap {
    fn update(&mut self, _ctx: &mut Context, _conf: &Config) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, _assets: &Assets, conf: &Config) -> GameResult {
        let (sw, sh) = (conf.screen_width, conf.screen_height);
        let (mw, mh) = (self.width(ctx, sw), self.height(ctx, sh));
        let pos = self.pos(sw, sh);
        let map_rect = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(pos.x, pos.y, mw, mh),
            Color::new(0., 0., 0., 0.7),
        )?;          
        graphics::draw(ctx, &map_rect, DrawParam::default())?;

        let (rw, rh) = (mw / (DUNGEON_GRID_COLS as f32), mh / (DUNGEON_GRID_ROWS as f32));
        let mut room_rect;

        for r in 0..DUNGEON_GRID_ROWS {
            for c in 0..DUNGEON_GRID_COLS {
                room_rect = MeshBuilder::new()
                    .rectangle( 
                        DrawMode::fill(),
                        Rect::new(pos.x + (c as f32) * rw, pos.y + (r as f32) * rh, rw, rh),
                        Color::BLACK,
                    )?
                    .rounded_rectangle(
                        DrawMode::fill(),
                        Rect::new(pos.x + (c as f32) * rw, pos.y + (r as f32) * rh, rw, rh),
                        8.,
                        Color::WHITE,
                    )?
                    .build(ctx)?;

                match self.visited[r][c] {
                    1 => graphics::draw(ctx, &room_rect, DrawParam::default().color(Color::new(0.3, 0.3, 0.3, 1.)))?,
                    2 => graphics::draw(ctx, &room_rect, DrawParam::default().color(Color::new(0.6, 0.6, 0.6, 1.)))?,
                    3 => graphics::draw(ctx, &room_rect, DrawParam::default().color(Color::WHITE))?,
                    _ => (),
                }
            }
        }
        Ok(())
    }

    fn pos(&self, sw: f32, sh: f32) -> Point2<f32> { Point2 { x: sw * self.pos.x, y: sh * self.pos.y } }

    fn width(&self, _ctx: &mut Context, sw: f32) -> f32 { sw * self.width }

    fn height(&self, _ctx: &mut Context, sh: f32) -> f32 { sh * self.height }

    fn as_any(&self) -> &dyn Any { self }
    
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

pub struct HealthBar {
    pub pos: Point2<f32>,
    pub width: f32,
    pub height: f32,
    pub health: f32,
    pub max_health: f32,
}

impl UIElement for HealthBar {
    fn update(&mut self, _ctx: &mut Context, _conf: &Config) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, assets: &Assets, conf: &Config) -> GameResult {
        let (sw, sh) = (conf.screen_width, conf.screen_height);
        let pos = self.pos(sw, sh);
        let img_dims = assets.sprites.get("heart_full").unwrap().dimensions();
        let img_width = self.width(ctx, sw) / self.max_health;

        for i in 1..=(self.max_health as i32) {
            let index = i as f32;

            let draw_params = DrawParam::default()
                .dest([pos.x + index * img_width * 1.1, pos.y])
                .scale([img_width / img_dims.w, self.height(ctx, sh) / img_dims.h])
                .offset([0.5, 0.5]);

            let dif = self.health - index;

            if dif >= 0. { graphics::draw(ctx, assets.sprites.get("heart_full").unwrap(), draw_params)?; }
            else if dif >= -0.5 { graphics::draw(ctx, assets.sprites.get("heart_half").unwrap(), draw_params)?; }
            else { graphics::draw(ctx, assets.sprites.get("heart_empty").unwrap(), draw_params)?; }
        }

        Ok(())
    }

    fn pos(&self, sw: f32, sh: f32) -> Point2<f32> { Point2 { x: sw * self.pos.x, y: sh * self.pos.y } }

    fn width(&self, _ctx: &mut Context, sw: f32) -> f32 { sw * self.width }

    fn height(&self, _ctx: &mut Context, sh: f32) -> f32 { sh * self.height }

    fn as_any(&self) -> &dyn Any { self }
    
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    
}

pub struct Overlay {
    pos: Point2<f32>,
    width: f32,
    height: f32,
    ui_elements: Vec<Box<dyn UIElement>>,
}

impl Overlay {
    pub fn new(player: &Player, _dungeon: &Dungeon, cur_room: (usize, usize)) -> Self {
        let pos = Point2 { x: 0., y: 0.};
        let (width, height) = (1.0, 1.0);
        let ui_elements: Vec<Box<dyn UIElement>> = vec![
            Box::new(HealthBar {
                pos: Point2 { x: HEALTH_BAR_POS.0, y: HEALTH_BAR_POS.1 },
                width: HEALTH_BAR_SCALE.0,
                height: HEALTH_BAR_SCALE.1,
                health: player.health,
                max_health: player.max_health,
            }),
            Box::new(Minimap {
                pos: Point2 { x: MINIMAP_POS.0, y: MINIMAP_POS.1 },
                width: MINIMAP_SCALE,
                height: MINIMAP_SCALE,
                cur_room,
                visited: [[0; DUNGEON_GRID_COLS]; DUNGEON_GRID_ROWS],
            }),
        ];

        Self {
            pos,
            width,
            height,
            ui_elements,
        }
    }

    pub fn update_vars(&mut self, player: &Player, dungeon: &Dungeon, cur_room: (usize, usize)) {
        for e in self.ui_elements.iter_mut() {
            if let Some(h) = e.as_any_mut().downcast_mut::<HealthBar>() {
                h.health = player.health;
                h.max_health = player.max_health;
            }
            else if let Some(m) = e.as_any_mut().downcast_mut::<Minimap>() {
                let (r, c) = cur_room;
                let grid = dungeon.get_grid();

                m.visited[m.cur_room.0][m.cur_room.1] = 2;
                m.cur_room = cur_room;
                m.visited[r][c] = 3;

                if r > 1                     && grid[r - 1][c] != 0 { m.visited[r - 1][c] = usize::max(1, m.visited[r - 1][c]); }
                if r < DUNGEON_GRID_ROWS - 1 && grid[r + 1][c] != 0 { m.visited[r + 1][c] = usize::max(1, m.visited[r + 1][c]); }
                if c > 1                     && grid[r][c - 1] != 0 { m.visited[r][c - 1] = usize::max(1, m.visited[r][c - 1]); }
                if c < DUNGEON_GRID_COLS - 1 && grid[r][c + 1] != 0 { m.visited[r][c + 1] = usize::max(1, m.visited[r][c + 1]); }
            }
        }
    }
}

impl UIElement for Overlay {
    fn update(&mut self, _ctx: &mut Context, _conf: &Config) -> GameResult {
        for e in self.ui_elements.iter_mut() { e.update(_ctx, _conf)?; }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, _assets: &Assets, _conf: &Config) -> GameResult {
        for e in self.ui_elements.iter_mut() { e.draw(ctx, _assets, _conf)?; }

        Ok(())
    }

    fn pos(&self, sw: f32, sh: f32) -> Point2<f32> { Point2 { x: sw * self.pos.x, y: sh * self.pos.y } }

    fn width(&self, _ctx: &mut Context, sw: f32) -> f32 { sw * self.width }

    fn height(&self, _ctx: &mut Context, sh: f32) -> f32 { sh * self.height }

    fn as_any(&self) -> &dyn Any { self }
    
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}