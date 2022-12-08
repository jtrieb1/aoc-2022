use crate::util::{AOCSolution, read_input_to_str, str_to_grid_info, parse_each_char};
use std::str::FromStr;

solution!(Day 8 => Forest);

pub struct Forest {
    trees: Vec<Tree>,
    width: usize,
    height: usize
}

impl AOCSolution for Forest {
    fn load_from(input_file_path: &str) -> Result<Box<Self>, Box<dyn std::error::Error>>
    where
        Self: Sized {
        Ok(Box::new(Self::new_from_file(input_file_path)?))
    }
    
    fn part_1(&mut self) -> String {
        format!("{}", self.count_visible_trees())
    }
    
    fn part_2(&mut self) -> String {
        let score = (0..self.trees.len()).map(|idx| self.get_scenic_score(idx)).max().unwrap();
        format!("{}", score)
    }
}

impl Forest {
    pub fn new_from_file(input_file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let input_str = read_input_to_str(input_file_path, true)?;
        Self::new_from_str(&input_str)
    }
    
    pub fn new_from_str(input_str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let (width, height) = str_to_grid_info(input_str);
        let trees = parse_each_char::<Tree>(input_str)?;
        
        Ok(Self {
            trees,
            width,
            height
        })
    }
    
    fn get_tree_at_x_y(&self, x: usize, y: usize) -> &Tree {
        let idx = (y * self.width) + (x % self.width);
        &self.trees[idx]
    }
    
    fn on_edge(&self, tree_idx: usize) -> bool {
        let (x, y) = self.get_tree_coord(tree_idx);
        x == 0 || y == 0 || x == self.width - 1 || y == self.height - 1
    }
    
    fn get_tree_coord(&self, tree_idx: usize) -> (usize, usize) {
        (tree_idx % self.width, tree_idx / self.width)
    }
    
    fn get_lines_from_tree_to_edge(&self, tree_idx: usize) -> Vec<Vec<&Tree>> {
        let (x, y) = self.get_tree_coord(tree_idx);
        let north = (0..y).map(|y1| self.get_tree_at_x_y(x, y1)).rev().collect::<Vec<&Tree>>();
        let south = (y+1..self.height).map(|y1| self.get_tree_at_x_y(x, y1)).collect::<Vec<&Tree>>();
        let east = (0..x).map(|x1| self.get_tree_at_x_y(x1, y)).rev().collect::<Vec<&Tree>>();
        let west = (x+1..self.width).map(|x1| self.get_tree_at_x_y(x1, y)).collect::<Vec<&Tree>>();
        vec![north, south, east, west]
    }
    
    fn tree_is_visible(&self, tree_idx: usize) -> bool {
        if self.on_edge(tree_idx) { return true; }
        let tree = &self.trees[tree_idx];
        let paths = self.get_lines_from_tree_to_edge(tree_idx);
        paths.iter().any(|p| p.iter().all(|t| t.height < tree.height))
    }
    
    fn count_visible_trees(&self) -> usize {
        (0..self.trees.len())
            .map(|idx| self.tree_is_visible(idx))
            .filter(|b| *b)
            .count()
    }
    
    fn get_scenic_score(&self, tree_idx: usize) -> usize {
        let tree = &self.trees[tree_idx];
        let paths = self.get_lines_from_tree_to_edge(tree_idx);
        let mut total = 1;
        for path in paths {
            total *= self.count_visible_in_path(tree.height, path);
        }
        total
    }
    
    fn count_visible_in_path(&self, my_height: u32, path: Vec<&Tree>) -> usize {
        let mut visible_ct = path.iter().take_while(|&&t| t.height < my_height).count();
        if visible_ct < path.len() { visible_ct += 1; } // Add one for blocking tree
        visible_ct
    }
}

#[derive(Debug)]
struct Tree {
    height: u32
}

impl FromStr for Tree {
    type Err = TreeParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(TreeParseErr("Invalid string len"));
        }
        Ok(Tree {
            height: s.parse::<u32>().map_err(|_| TreeParseErr("Invalid digit"))?
        })
    }
}

custom_error!(TreeParseErr);

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn given_test() {
        let input_str = 
"30373
25512
65332
33549
35390";
        let mut forest = Forest::new_from_str(input_str).expect("Failed to parse input");
        assert!(forest.tree_is_visible(6));
        assert!(forest.tree_is_visible(7));
        assert!(forest.tree_is_visible(11));
        assert!(forest.tree_is_visible(13));
        assert!(forest.tree_is_visible(17));
        assert_eq!(forest.part_1(), "21");
        assert_eq!(forest.get_scenic_score(17), 8);
        assert_eq!(forest.part_2(), "8");
    }
}