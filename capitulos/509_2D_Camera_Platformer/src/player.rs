use raylib::core::math::Vector2;

use libfinal::parametros::{CodigosTecla, Parametros};

use crate::env_item::EnvItem;

const G: f32 = 400.0;
const PLAYER_JUMP_SPD: f32 = 350.0;
const PLAYER_HOR_SPD: f32 = 200.0;

pub struct Player {
    pub position: Vector2,
    pub speed: f32,
    pub can_jump: bool,
}

impl Player {
    pub fn new() -> Player {
        Player {
            position: Vector2::new(400.0, 280.0),
            speed: 0.0,
            can_jump: false,
        }
    }

    pub fn update_player(&mut self, param: &mut Parametros, env_items: &Vec<EnvItem>, delta: f32) {
        if param.key == CodigosTecla::LeftArrow {
            self.position.x -= PLAYER_HOR_SPD * delta;
        }
        if param.key == CodigosTecla::RightArrow {
            self.position.x += PLAYER_HOR_SPD * delta;
        }

        if param.key == CodigosTecla::SPACE && self.can_jump {
            self.speed = -PLAYER_JUMP_SPD;
            self.can_jump = false;
        }

        let mut hit_obstacle: bool = false;

        for i in 0..env_items.len() {
            let mut p = self.position;
            if env_items[i].blocking && env_items[i].rect.x <= p.x
                && env_items[i].rect.x + env_items[i].rect.width >= p.x
                && env_items[i].rect.y >= p.y
                && env_items[i].rect.y < p.y + self.speed * delta {
                hit_obstacle = true;
                self.speed = 0.0;
                p.y = env_items[i].rect.y
            }
        }

        if hit_obstacle {
            self.position.y += self.speed * delta;
            self.speed += G * delta;
            self.can_jump = false;
        }
    }


//
//    if (!hitObstacle)
//    {
//        player->position.y += player->speed*delta;
//        player->speed += G*delta;
//        player->canJump = false;
//    }
//    else player->canJump = true;
//    }
}
