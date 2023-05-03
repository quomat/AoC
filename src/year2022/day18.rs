use std::collections::HashSet;

use itertools::Itertools;

use crate::{day0::Day, utils::{tuple_map::TupleMap, bfs::*}, year2022::day18::cube_arena::parser::point};

use self::cube_arena::*;

pub struct Day18;

impl Day<2022, 18, CubeSpace, u32> for Day18 {
    fn solve(input: CubeSpace) -> u32 {
        input.sum()
    }

    fn solve2(input: CubeSpace) -> u32 {
        let mut answer = input.sum();

        let mut visited = HashSet::new();

        let test_point = Point {x:4,y:8,z:8};
        for point in [test_point]//input.get_all_possible_points()
        {
            if visited.contains(&point) || input.field_type(&point) != FieldType::Blank {continue;}

            if let Some(inner) = input.run_bfs(point, |n,r:u32|
                {
                    let res = r + (CUBE_SIDES - input.get(n).unwrap()) as u32;
                    println!("Value at {0:?} is {1:?}, and the current overall is {2:?}",n,input.get(n).unwrap(),res);
                    res 
                }, &mut visited){
                println!("Hole found with success! {0:?} inner sides substracted from total {1:?} giving  new total {2:?}",inner,answer,answer - inner);
                answer -= inner;
            }
        }
        answer
    }

    fn parse(input: &str) -> CubeSpace {
        let points = input.lines().map(point).collect::<HashSet<Point>>();
        let (max_x, max_y, max_z) = points
            .iter()
            .map(|&Point { x, y, z }| (x, y, z))
            .multiunzip()
            .tmap(|v: Vec<K>| v.into_iter().max().unwrap());
        let mut cubes = CubeSpace::new(max_x + 1, max_y + 1, max_z + 1);
        for p in &points {
            for q in cubes.neighbours_of(&p) {
                if cubes.get(q).is_ok() {
                    cubes.sub1(q);
                }
            }
        }
        cubes.set_points(points);
        cubes
    }
}

impl BreadthTraversable for CubeSpace
{
    type Item = Point;

    fn get_neighbours(&self, item: &Self::Item) -> Vec<Self::Item> {
        self.neighbours_of(&item)
    }

    fn field_type(&self, item: &Self::Item) -> FieldType {
        if self.get_points().contains(item) {
            FieldType::Stop
        }
        else if self.get_neighbours(item).len() < 6{
            FieldType::Exit
        }
        else {
            FieldType::Blank
        }
        
    }
}
mod cube_arena {
    use std::collections::HashSet;

    /// Field of coordinates
    pub type K = usize;
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Point {
        pub x: K,
        pub y: K,
        pub z: K,
    }

    impl Point {
    }

    pub mod parser {
        use itertools::Itertools;

        use super::{Point, K};

        pub fn point(input: &str) -> Point {
            let (x, y, z) = input
                .split(',')
                .map(str::parse::<K>)
                .map(Result::<_, _>::unwrap)
                .tuples()
                .next()
                .unwrap();

            Point { x, y, z }
        }
    }

    pub struct CubeSpace {
        cubes: Vec<u8>,
        points: HashSet<Point>,
        depth: K,
        height: K,
        width: K,
    }

    pub const CUBE_SIDES: u8 = 6;

    impl CubeSpace {
        pub fn new(width: K, height: K, depth: K) -> CubeSpace {
            CubeSpace {
                cubes: vec![CUBE_SIDES; width * height * depth],
                points: HashSet::new(),
                width,
                height,
                depth,
            }
        }

        fn check(&self, Point { x, y, z }: Point) -> bool {
            if x >= self.width {
                return false;
            }

            if y >= self.height {
                return false;
            }
            if z >= self.depth {
                return false;
            }

            true
        }

        pub fn get(&self, p @ Point { x, y, z }: Point) -> Result<u8, String> {
            if !self.check(p) {
                return Err(format!(
                    "Error. Tried to get something at {0:?} while the dimensions are {1},{2},{3}",
                    p, self.width, self.height, self.depth
                ));
            }

            Ok(self.cubes[(z * self.height + y) * self.width + x])
        }

        pub fn set(&mut self, p @ Point { x, y, z }: Point, v: u8) {
            if !self.check(p) {
                panic!(
                    "Error. Tried to set something at {0:?} while the dimensions are {1},{2},{3}",
                    p, self.width, self.height, self.depth
                );
            }
            self.cubes[(z * self.height + y) * self.width + x] = v;
        }
        pub fn sub1(&mut self, p @ Point { x, y, z }: Point) {
            if !self.check(p) {
                panic!(
                    "Error. Tried to sub1 something at {0:?} while the dimensions are {1},{2},{3}",
                    p, self.width, self.height, self.depth
                );
            }
            self.cubes[(z * self.height + y) * self.width + x] -= 1;
        }

        pub fn sum(&self) -> u32 {
            self.points
                .iter()
                .map(|&p| self.get(p).unwrap() as u32)
                .sum()
        }

        pub fn add_point(&mut self, p: Point) {
            self.points.insert(p);
        }

        pub fn set_points(&mut self, pts: HashSet<Point>) {
            self.points = pts;
        }

        pub(crate) fn get_points(&self) -> &HashSet<Point> {
            &self.points
        }

        pub fn neighbours_of(&self, p : &Point) -> Vec<Point> {
            let mut v = vec![];
            if p.x < self.width - 1 {
                v.push(Point {
                    x: p.x + 1,
                    ..*p
                });
            };
            if p.y < self.height - 1{
                v.push(Point {
                    y: p.y + 1,
                    ..*p
                });
            };
            if p.z < self.depth - 1 {
                v.push(Point {
                    z: p.z + 1,
                    ..*p
                });
            };
            
            if p.x > 0 {
                v.push(Point {
                    x: p.x - 1,
                    ..*p
                })
            }
            if p.y > 0 {
                v.push(Point {
                    y: p.y - 1,
                    ..*p
                })
            }
            if p.z > 0 {
                v.push(Point {
                    z: p.z - 1,
                    ..*p
                })
            }
            v
        }

        pub(crate) fn get_all_possible_points(&self) -> Vec<Point> {
            let mut points = Vec::new();
            for x in 0..self.width {
                for y in 0..self.height {
                    for z in 0..self.depth {
                        points.push(Point{x,y,z});
                    }
                }
            }
            points
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::year2022::day18::cube_arena::CUBE_SIDES;

        use super::{CubeSpace, Point};

        #[test]
        fn cube_space_init() {
            let width = 5;
            let height = 3;
            let length = 7;
            let mut cubes = CubeSpace::new(width, height, length);
            for x in 0..width {
                for y in 0..height {
                    for z in 0..length {
                        cubes.set(Point { x, y, z }, 6);
                        assert_eq!(Ok(6), cubes.get(Point { x, y, z }));
                    }
                }
            }
        }
        #[test]
        fn cube_space_sum() {
            let width = 5;
            let height = 3;
            let length = 7;
            let mut cubes = CubeSpace::new(width, height, length);
            for x in 0..width {
                for y in 0..height {
                    for z in 0..length {
                        let p = Point { x, y, z };
                        cubes.set(p, CUBE_SIDES);
                        cubes.add_point(p);
                    }
                }
            }
            assert_eq!(
                cubes.sum() as usize,
                height * width * length * (CUBE_SIDES as usize)
            );
        }
    }
}
