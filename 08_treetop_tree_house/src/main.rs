use std::{error::Error, fs::read_to_string, iter::Copied, slice::Iter, str::FromStr};

trait TreeMap<T: Copy> {
    fn new(values: &[T], height: usize, width: usize) -> Self;

    fn from_columns(columns: &Vec<Vec<T>>) -> Self
    where
        Self: Sized,
    {
        let values = (0..columns.len())
            .flat_map(|index| {
                columns
                    .iter()
                    .map(|column| column[index])
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Self::new(&values, columns[0].len(), columns.len())
    }

    fn from_rows(rows: &Vec<Vec<T>>) -> Self
    where
        Self: Sized,
    {
        Self::new(&rows.concat(), rows.len(), rows[0].len())
    }

    fn values(&self) -> Copied<Iter<T>>;

    fn height(&self) -> usize;

    fn width(&self) -> usize;

    fn row(&self, index: usize) -> Vec<T> {
        self.values()
            .skip(index * self.width())
            .take(self.width())
            .collect()
    }

    fn rows(&self) -> Vec<Vec<T>> {
        (0..self.height()).map(|index| self.row(index)).collect()
    }

    fn column(&self, index: usize) -> Vec<T> {
        self.values()
            .skip(index)
            .step_by(self.width())
            .take(self.height())
            .collect()
    }

    fn columns(&self) -> Vec<Vec<T>> {
        (0..self.width()).map(|index| self.column(index)).collect()
    }

    fn tree_coordinates(&self) -> Vec<(usize, usize)> {
        (0..self.height())
            .flat_map(|y| (0..self.width()).map(move |x| (x, y)))
            .collect()
    }

    fn at(&self, x: usize, y: usize) -> Option<T> {
        self.values().nth(y * self.width() + x)
    }
}

type TreeHeight = u8;

struct TreeHeightMap {
    tree_heights: Vec<TreeHeight>,
    height: usize,
    width: usize,
}

impl TreeMap<TreeHeight> for TreeHeightMap {
    fn new(values: &[TreeHeight], height: usize, width: usize) -> Self {
        Self {
            tree_heights: values.to_vec(),
            height,
            width,
        }
    }

    fn values(&self) -> Copied<Iter<TreeHeight>> {
        self.tree_heights.iter().copied()
    }

    fn height(&self) -> usize {
        self.height
    }

    fn width(&self) -> usize {
        self.width
    }
}

impl FromStr for TreeHeightMap {
    type Err = Box<dyn Error>;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let tree_height_rows = value.trim().split('\n').collect::<Vec<_>>();

        let tree_heights = tree_height_rows
            .concat()
            .chars()
            .map(|tree_height| tree_height as u8 - 48)
            .collect::<Vec<_>>();

        Ok(Self::new(
            &tree_heights,
            tree_height_rows.len(),
            tree_heights.len() / tree_height_rows.len(),
        ))
    }
}

struct TreeVisibilityMap {
    visibilities: Vec<bool>,
    height: usize,
    width: usize,
}

impl TreeVisibilityMap {
    fn get_row_visibility(row: &[TreeHeight]) -> Vec<bool> {
        let (_, visibility) = row.iter().fold(
            (-1, vec![]),
            |(tallest_tree_height, mut visibility_row), &height| {
                let tree_height = height as i16;
                let tree_is_visible = tree_height > tallest_tree_height;

                visibility_row.push(tree_is_visible);

                if tree_is_visible {
                    (tree_height, visibility_row)
                } else {
                    (tallest_tree_height, visibility_row)
                }
            },
        );

        visibility
    }

    fn reverse_row<T: Copy>(row: &[T]) -> Vec<T> {
        row.iter().rev().copied().collect()
    }

    fn build_left_visibility_map(height_map: &TreeHeightMap) -> TreeVisibilityMap {
        let rows = height_map
            .rows()
            .iter()
            .map(|row| Self::get_row_visibility(row))
            .collect::<Vec<_>>();

        Self::from_rows(&rows)
    }

    fn build_right_visibility_map(height_map: &TreeHeightMap) -> TreeVisibilityMap {
        let rows = height_map
            .rows()
            .iter()
            .map(|row| Self::reverse_row(row))
            .map(|row| Self::get_row_visibility(&row))
            .map(|row| Self::reverse_row(&row))
            .collect::<Vec<_>>();

        Self::from_rows(&rows)
    }

    fn build_up_visibility_map(height_map: &TreeHeightMap) -> TreeVisibilityMap {
        let columns = height_map
            .columns()
            .iter()
            .map(|column| Self::get_row_visibility(column))
            .collect::<Vec<_>>();

        Self::from_columns(&columns)
    }

    fn build_down_visibility_map(height_map: &TreeHeightMap) -> TreeVisibilityMap {
        let columns = height_map
            .columns()
            .iter()
            .map(|column| Self::reverse_row(column))
            .map(|column| Self::get_row_visibility(&column))
            .map(|column| Self::reverse_row(&column))
            .collect::<Vec<_>>();

        Self::from_columns(&columns)
    }

    fn from_height_map(height_map: &TreeHeightMap) -> TreeVisibilityMap {
        let left_visibility_map = Self::build_left_visibility_map(height_map);
        let right_visibility_map = Self::build_right_visibility_map(height_map);
        let up_visibility_map = Self::build_up_visibility_map(height_map);
        let down_visibility_map = Self::build_down_visibility_map(height_map);

        let values = height_map
            .tree_coordinates()
            .iter()
            .map(|&(x, y)| {
                left_visibility_map.at(x, y).unwrap_or(false)
                    || right_visibility_map.at(x, y).unwrap_or(false)
                    || up_visibility_map.at(x, y).unwrap_or(false)
                    || down_visibility_map.at(x, y).unwrap_or(false)
            })
            .collect::<Vec<_>>();

        Self::new(&values, height_map.height(), height_map.width())
    }
}

impl TreeMap<bool> for TreeVisibilityMap {
    fn new(values: &[bool], height: usize, width: usize) -> Self {
        Self {
            visibilities: values.to_vec(),
            height,
            width,
        }
    }

    fn values(&self) -> Copied<Iter<bool>> {
        self.visibilities.iter().copied()
    }

    fn height(&self) -> usize {
        self.height
    }

    fn width(&self) -> usize {
        self.width
    }
}

type TreeScenicScore = u32;

struct TreeScenicScoreMap {
    tree_scenic_scores: Vec<TreeScenicScore>,
    height: usize,
    width: usize,
}

impl TreeMap<TreeScenicScore> for TreeScenicScoreMap {
    fn new(values: &[TreeScenicScore], height: usize, width: usize) -> Self {
        Self {
            tree_scenic_scores: values.to_vec(),
            height,
            width,
        }
    }

    fn values(&self) -> Copied<Iter<TreeScenicScore>> {
        self.tree_scenic_scores.iter().copied()
    }

    fn height(&self) -> usize {
        self.height
    }

    fn width(&self) -> usize {
        self.width
    }
}

impl TreeScenicScoreMap {
    fn max(&self) -> TreeScenicScore {
        self.values().max().unwrap_or_default()
    }

    fn get_scenic_score_for_tree(height_map: &TreeHeightMap, x: usize, y: usize) -> u32 {
        if x == 0 || y == 0 {
            return 0;
        }

        let tree_height = height_map.at(x, y).unwrap();

        let row = height_map.row(y).to_vec();
        let trees_to_right = &row[x + 1..];
        let trees_to_left = &mut row[..x].to_vec();
        trees_to_left.reverse();

        let column = height_map.column(x);
        let trees_downwards = &column[y + 1..];
        let trees_upwards = &mut column[..y].to_vec();
        trees_upwards.reverse();

        let tree_views = [
            trees_to_right,
            trees_to_left,
            trees_downwards,
            trees_upwards,
        ];

        tree_views
            .iter()
            .map(|tree_heights| {
                tree_heights
                    .iter()
                    .enumerate()
                    .find(|&(_, &height)| height >= tree_height)
                    .map(|(i, _)| i + 1)
                    .unwrap_or(tree_heights.len()) as u32
            })
            .product()
    }

    fn from_height_map(height_map: &TreeHeightMap) -> Self {
        let values = height_map
            .tree_coordinates()
            .iter()
            .map(|&(x, y)| Self::get_scenic_score_for_tree(height_map, x, y))
            .collect::<Vec<_>>();

        Self::new(&values, height_map.height(), height_map.width())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("./input.txt")?;

    let height_map = input.parse::<TreeHeightMap>()?;

    let visibility_map = TreeVisibilityMap::from_height_map(&height_map);

    let trees_visible_outside_the_grid = visibility_map
        .values()
        .filter(|&is_visible| is_visible)
        .count();

    println!(
        "There are {} trees visible from outside the grid",
        trees_visible_outside_the_grid
    );

    let scenic_score_map = TreeScenicScoreMap::from_height_map(&height_map);

    println!(
        "The highest scenic score possible in the grid is {}",
        scenic_score_map.max()
    );

    Ok(())
}
