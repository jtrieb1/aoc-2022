use crate::util::{AOCSolution, read_input_to_str};

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
        let mut lines = input_str.lines();
        let width = lines.nth(0).map(|l| l.len()).unwrap();
        lines = input_str.lines();
        let height = lines.count();
        let mut trees = Vec::new();
        for c in input_str.split("") {
            if c.is_empty() || c == "\n" { continue; }
            let tree = Tree {
                height: c.parse::<u32>()?
            };
            trees.push(tree);
        }
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
        x == 0 || y == 0 || x == self.width || y == self.height
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
            let mut visible = path.iter().take_while(|&&t| t.height < tree.height).count();
            if visible < path.len() { visible += 1; } // Add one for blocking tree
            total *= visible; 
        }
        total
    }
}

#[derive(Debug)]
struct Tree {
    height: u32
}

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