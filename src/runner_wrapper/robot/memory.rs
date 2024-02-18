use std::time::{SystemTime, UNIX_EPOCH};
use image::{DynamicImage, GenericImage, Rgba};
use robotics_lib::world::tile::{Tile, TileType};
use robotics_lib::world::tile::TileType::ShallowWater;

pub(crate) struct MyMemory{
    pub discovered_map:Vec<Vec<Option<Tile>>>,
    pub discovered_number:i32,
    pub water_found:i32,
    pub world_size:usize,
}

impl MyMemory {
    pub(crate) fn new(world_size:usize) -> Self {
        MyMemory{
            discovered_map:vec![vec![None;world_size];world_size],
            discovered_number:0,
            water_found:0,
            world_size,
        }
    }

    pub(crate) fn discover_view(&mut self, view:Vec<Vec<Option<Tile>>>, c_row:usize, c_col:usize){
        let mut valid_cells = vec![vec![true;3];3];
        if c_row == 0 {
            valid_cells[0][0] = false;
            valid_cells[0][1] = false;
            valid_cells[0][2] = false;
        }
        if c_col == 0 {
            valid_cells[0][0] = false;
            valid_cells[1][0] = false;
            valid_cells[2][0] = false;
        }
        if c_row == self.world_size-1 {
            valid_cells[2][0] = false;
            valid_cells[2][1] = false;
            valid_cells[2][2] = false;
        }
        if c_col == self.world_size-1 {
            valid_cells[0][2] = false;
            valid_cells[1][2] = false;
            valid_cells[2][2] = false;
        }
        for (i,row) in valid_cells.iter().enumerate() {
            for (j, is_valid) in row.iter().enumerate() {
                if *is_valid {
                    if let None = self.discovered_map[c_row+i-1][c_col+j-1]{
                        let tile = view[i][j].clone().unwrap();
                        if tile.tile_type == ShallowWater {self.water_found += 1}
                        self.discovered_map[c_row+i-1][c_col+j-1] = Some(tile);
                        self.discovered_number += 1;
                    }

                }
            }
        }
    }

    pub(crate) fn gen_img(&self) {
        // Assuming you have the image crate in your dependencies
        let mut img = DynamicImage::new_rgba8(self.discovered_map[0].len() as u32 * 5, self.discovered_map.len() as u32 * 5);

        for (i, row) in self.discovered_map.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let color = match cell {
                    Some(tile) => match &tile.tile_type {
                        TileType::DeepWater => Rgba([0, 0, 128, 255]),
                        TileType::ShallowWater => Rgba([0, 0, 255, 255]),
                        TileType::Sand => Rgba([255, 255, 128, 255]),
                        TileType::Grass => Rgba([0, 255, 0, 255]),
                        TileType::Street => Rgba([128, 128, 128, 255]),
                        TileType::Hill => Rgba([0, 128, 0, 255]),
                        TileType::Mountain => Rgba([128, 128, 128, 255]),
                        TileType::Snow => Rgba([255, 255, 255, 255]),
                        TileType::Lava => Rgba([255, 0, 0, 255]),
                        TileType::Teleport(_) => Rgba([255, 255, 0, 255]),
                        TileType::Wall => Rgba([128, 0, 0, 255]),
                    },
                    None => Rgba([0, 0, 0, 255]), // Black for None
                };

                for x in 0..5 {
                    for y in 0..5 {
                        img.put_pixel((j as u32 * 5 + x) as u32, (i as u32 * 5 + y) as u32, color);
                    }
                }
            }
        }
        let name = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let path = format!("frames/{name}.jpg");
        // Save or display the generated image
        img.save(path).expect("Failed to save image");
    }
}