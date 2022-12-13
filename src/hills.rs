use crate::shared::read_lines;
use std::collections::HashMap;
use pathfinding::prelude::bfs;

#[derive(Eq, PartialEq, Clone, Debug)]
struct Map {
    grid: HashMap<(usize, usize), Point>,
    lowest_points: Vec<Point>
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
    height: usize,
}

impl Point {
    fn successors(&self, map: Map) -> Vec<Point>
    {
        let mut options: Vec<Point> = Vec::new();

        let mut point = map.get_point(self.x + 1, self.y);
        match point {
            Some(point) => {
                if point.height + 1 >= self.height {
                    options.push(point.clone());
                }
            }
            _ => ()
        }

        point = map.get_point(self.x, self.y + 1);
        match point {
            Some(point) => {
                if point.height + 1 >= self.height {
                    options.push(point.clone());
                }
            }
            _ => ()
        }

        if self.y > 0 {
            point = map.get_point(self.x, self.y - 1);
            match point {
                Some(point) => {
                    if point.height + 1 >= self.height {
                        options.push(point.clone());
                    }
                }
                _ => ()
            }
        }

        if self.x > 0 {
            point = map.get_point(self.x - 1, self.y);
            match point {
                Some(point) => {
                    if point.height + 1 >= self.height {
                        options.push(point.clone());
                    }
                }
                _ => ()
            }
        }
        // dbg!(&options);
        options
    }
}

impl Map {
    pub fn get_start(&self) -> Point
    {
        Point {
            x: 0,
            y: 20,
            height: 0
        }
    }
    pub fn get_end(&self) -> Point
    {
        Point {
            x: 88,
            y: 20,
            height: 25
        }
    }
    pub fn get_point(&self, x: usize, y: usize) -> Option<Point>
    {
        let target = self.grid.get(&(x, y));
        target.cloned()
    }
}
// 115 too low
// 381 too high
// part 2 122 too low
pub fn route()
{
    let map: Map = read_map();
    // dbg!(&map.grid.get(&(88, 19)));
    let goal: Point = map.get_start();
    let start: Point = map.get_end();
    let mut result = bfs(&start, |p| p.successors(map.clone()), |p| *p == goal);
    // dbg!(&result);
    // match result {
    //     Some(points) => {
    //         println!("Steps needed: {}", points.len() - 1);
    //     }
    //     _ => {
    //         println!("Not found!");
    //     }
    // }
    let mut min = result.unwrap().len() - 1;
    let mut count = 0;
    for point in map.lowest_points.iter() {
        println!("Trying ({}/{})", count, map.lowest_points.len());
        result = bfs(&start, |p| p.successors(map.clone()), |p| *p == *point);
        match result {
            Some(result) => {
                println!("{} steps!", result.len() - 1);
                if result.len() -1 < min {
                    min = result.len() - 1;
                }
            }
            _ => {
                println!("Not possible!");
            }
        }
        count = count + 1;
    }
    println!("Min steps a to E: {}", min);
}

fn read_map() -> Map
{
    let mut map: Map = Map {
        grid: HashMap::new(),
        lowest_points: Vec::new()
    };

    let mut char_height_map: HashMap<char, usize> = HashMap::new();
    for (height, char) in ('a'..='z').enumerate() {
        char_height_map.insert(char, height);
    }
    char_height_map.insert('S', 0);
    char_height_map.insert('E', 25);

    if let Ok(lines) = read_lines("./src/data/12/data.txt") {

        for (y, line) in lines.enumerate() {
            if let Ok(item) = line {
                let points: Vec<char> = item.chars().collect();
                for (x, point) in points.iter().enumerate() {
                    let height: usize = *char_height_map.get(&point).unwrap();
                    if *point == 'E' || *point == 'S' {
                        println!("({x},{y}), {point}");
                    }
                    let grid_point: Point = Point {
                        height: height,
                        x: x,
                        y: y
                    };
                    if grid_point.height == 0 && x < 3 {
                        map.lowest_points.push(grid_point.clone());
                    }

                    map.grid.insert((x, y), grid_point);
                }
            }
        }
    }

    map
}
