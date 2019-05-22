use amethyst::core::Float;
use amethyst::core::Transform;
use amethyst::ecs::prelude::*;
use amethyst::ecs::NullStorage;
use amethyst::renderer::{SpriteRender, SpriteSheetHandle};

use crate::zombie_curtains::WorldResources;
extern crate rand;
use rand::prelude::*;

pub struct Chunk {
    pub tile_start_id: i32,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk { tile_start_id: 0 }
    }
}

impl Component for Chunk {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct GenerateChunk {
    pub x: i32,
    pub y: i32,
    pub generated: bool,
}

impl GenerateChunk {
    pub fn new(pos: (i32, i32)) -> GenerateChunk {
        GenerateChunk {
            x: pos.0,
            y: pos.1,
            generated: false,
        }
    }
}

impl Component for GenerateChunk {
    type Storage = VecStorage<Self>;
}

pub struct ChunkGeneratorSystem;

impl<'s> System<'s> for ChunkGeneratorSystem {
    type SystemData = (
        WriteStorage<'s, GenerateChunk>,
        Entities<'s>,
        ReadExpect<'s, WorldResources>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
    );

    fn run(
        &mut self,
        (mut chunk, entities, resources, mut transforms, mut sprites): Self::SystemData,
    ) {
        for chunk_comp in (&mut chunk).join() {
            if !chunk_comp.generated {
                chunk_comp.generated = true;
                for y in 0..16 {
                    for x in 0..16 {
                        let mut transform = Transform::default();
                        let tile_x = chunk_comp.x * 512 + x * 32;
                        let tile_y = chunk_comp.y * 512 + y * 32;
                        
                        let sprite = SpriteRender {
                            sprite_sheet: resources.world_sprites[generate_tile((tile_x, tile_y))]
                                .clone(),
                            sprite_number: 0,
                        };

                        transform.set_translation_xyz(
                            Float::from(tile_x as f32),
                            Float::from(tile_y as f32),
                            0.,
                        );

                        entities
                            .build_entity()
                            .with(transform, &mut transforms)
                            .with(sprite, &mut sprites)
                            .build();
                    }
                }
            }
        }
    }
}

fn generate_tile(pos: (i32, i32)) -> usize {
    let mut rng = rand::thread_rng();
    let float: f32 = rng.gen();
    let sprite_id: usize = {
        let r = (float * 1.).round() as usize;
        if r > 0 {
            0
        } else {
            1
        }
    };

    sprite_id
}
