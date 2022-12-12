use std::{collections::HashSet, str::FromStr};

use crate::util::{parse_each_char, read_input_to_str, str_to_grid_info, AOCSolution};

solution!(Day 12 => HeightMap);

pub struct HeightMap {
    tiles: Vec<MapTile>,
    width: usize,
    height: usize,
    start_tile: usize,
    end_tile: usize,
}

impl AOCSolution for HeightMap {
    fn load_from(input_file_path: &str) -> Result<Box<Self>, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        Ok(Box::new(Self::new_from_file(input_file_path)?))
    }

    fn part_1(&mut self) -> String {
        self.initialize();
        format!("{}", self.get_shortest_path_to(TileType::End))
    }

    fn part_2(&mut self) -> String {
        format!("{}", self.get_shortest_path_to(TileType::Start))
    }
}

impl HeightMap {
    pub fn new_from_file(input_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let input_str = read_input_to_str(input_path, true)?;
        Self::new_from_str(&input_str)
    }

    pub fn new_from_str(input_str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let (width, height) = str_to_grid_info(input_str);
        let tiles = parse_each_char::<MapTile>(input_str)?;

        Ok(Self {
            tiles,
            width,
            height,
            start_tile: 0,
            end_tile: 0,
        })
    }

    pub fn initialize(&mut self) {
        let xy_list = (0..self.tiles.len())
            .map(|idx| self.xy_idx(idx))
            .collect::<Vec<(usize, usize)>>();
        for (idx, tile) in self.tiles.iter_mut().enumerate() {
            tile.position = xy_list[idx];
            if tile.kind == TileType::Start {
                self.start_tile = idx;
            }
            if tile.kind == TileType::End {
                self.end_tile = idx;
            }
        }
    }

    fn xy_idx(&self, idx: usize) -> (usize, usize) {
        (idx % self.width, idx / self.width)
    }

    fn coordinate_to_tile(&self, coord: (usize, usize)) -> Option<&MapTile> {
        if coord.0 > self.width - 1 || coord.1 > self.height - 1 {
            return None;
        }
        let idx = coord.1 * self.width + coord.0;
        if idx > self.tiles.len() - 1 {
            return None;
        }
        Some(&self.tiles[idx])
    }

    fn get_valid_neighbors(
        &self,
        tile: &MapTile,
        seen: &mut HashSet<(usize, usize)>,
        tile_kind: TileType,
    ) -> Vec<(usize, usize)> {
        let neighbors = tile.neighbors();
        neighbors
            .iter()
            .filter(|(x, y)| {
                if let Some(n_tile) = self.coordinate_to_tile((*x, *y)) {
                    let diff = match tile_kind {
                        TileType::End => n_tile.height as i32 - tile.height as i32,
                        _ => tile.height as i32 - n_tile.height as i32,
                    };
                    if diff > 1 {
                        return false;
                    }
                    return !seen.contains(&n_tile.position);
                }
                false
            })
            .map(|c| *c)
            .collect::<Vec<(usize, usize)>>()
    }

    fn get_shortest_path_to(&self, tile_kind: TileType) -> usize {
        let mut coords_seen = std::collections::HashSet::new();
        let mut paths = match tile_kind {
            TileType::End => vec![vec![self.get_start_tile()]],
            TileType::Start => vec![vec![self.get_end_tile()]],
            _ => vec![vec![self.get_start_tile()]],
        };
        while paths.len() > 0 {
            let mut next_paths = Vec::new();
            for path in paths.iter() {
                let last_tile = path.last().unwrap();
                let neighbors = self.get_valid_neighbors(last_tile, &mut coords_seen, tile_kind);
                for neighbor in neighbors.iter() {
                    coords_seen.insert(*neighbor);
                    let new_tile = self.coordinate_to_tile(*neighbor).unwrap();
                    let check = match tile_kind {
                        TileType::End => new_tile.kind == TileType::End,
                        _ => new_tile.height == 'a' as usize,
                    };
                    if check {
                        return path.len();
                    }
                    let mut new_path = path.clone();
                    new_path.push(new_tile);
                    next_paths.push(new_path);
                }
            }
            paths = next_paths;
        }
        0
    }

    fn get_start_tile(&self) -> &MapTile {
        self.tiles
            .iter()
            .find(|t| t.kind == TileType::Start)
            .unwrap()
    }

    fn get_end_tile(&self) -> &MapTile {
        self.tiles
            .iter()
            .find(|t| t.kind == TileType::End)
            .unwrap()
    }
}

#[derive(Debug)]
struct MapTile {
    kind: TileType,
    height: usize,
    position: (usize, usize),
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum TileType {
    Start,
    Mid,
    End,
}

impl FromStr for MapTile {
    type Err = MapTileParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let kind = match s {
            "S" => TileType::Start,
            "E" => TileType::End,
            _ => TileType::Mid,
        };
        let height = match s {
            "S" => 'a' as u8,
            "E" => 'z' as u8,
            _ => s.chars().last().unwrap() as u8,
        } as usize;
        Ok(Self {
            kind,
            height,
            position: (0, 0),
        })
    }
}

custom_error!(MapTileParseError);

impl MapTile {
    pub fn neighbors(&self) -> Vec<(usize, usize)> {
        let mut res = vec![
            (self.position.0 + 1, self.position.1),
            (self.position.0, self.position.1 + 1)
        ];
        
        if self.position.0 > 0 {
            res.push((self.position.0 - 1, self.position.1));
        }
        if self.position.1 > 0 {
            res.push((self.position.0, self.position.1 - 1));
        }
        
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given() {
        let input_str = "
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        let mut heightmap = HeightMap::new_from_str(input_str).expect("Failed to parse grid");
        heightmap.initialize();
        assert_eq!(heightmap.get_shortest_path_to(TileType::End), 31);
        assert_eq!(heightmap.get_shortest_path_to(TileType::Start), 29);
    }
}
