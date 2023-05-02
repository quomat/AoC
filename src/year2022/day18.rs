use std::collections::HashSet;

use itertools::Itertools;

use crate::{day0::Day, utils::tuple_map::TupleMap, year2022::day18::cube_arena::parser::point};

use self::cube_arena::*;

pub struct Day18;

impl Day<2022, 18, CubeSpace, u32> for Day18 {
    fn solve(input: CubeSpace) -> u32 {
        input.sum()
    }

    fn solve2(input: CubeSpace) -> u32 {
        todo!()
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
            for q in p.neighbours() {
                    if cubes.get(q).is_ok() {
                        cubes.sub1(q);
                    }
                }

        }
        cubes.set_points(points);
        cubes
    }
}


mod cube_arena {
    use std::collections::HashSet;

    /// Field of coordinates
    pub type K = usize;
    #[derive(Clone, Copy, Debug, PartialEq,Eq)]
    #[derive(Hash)]
    pub struct Point {
        pub x: K,
        pub y: K,
        pub z: K,
    }

    impl Point {
        pub fn neighbours(&self) -> Vec<Point> {
            let mut v = vec![
                Point {
                    x: self.x + 1,
                    ..*self
                },
                Point {
                    y: self.y + 1,
                    ..*self
                },
                Point {
                    z: self.z + 1,
                    ..*self
                },
            ];
            if self.x > 0 {
                v.push(Point {
                    x: self.x - 1,
                    ..*self
                })
            }
            if self.y > 0 {
                v.push(Point {
                    y: self.y - 1,
                    ..*self
                })
            }
            if self.z > 0 {
                v.push(Point {
                    z: self.z - 1,
                    ..*self
                })
            }
            v
        }
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


    const CUBE_SIDES : u8 = 6;

    impl CubeSpace {
        pub fn new(width: K, height: K, depth: K) -> CubeSpace {
            CubeSpace {
                cubes: vec![CUBE_SIDES; width * height * depth],
                points : HashSet::new(),
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
            self.points.iter().map(|&p| self.get(p).unwrap() as u32).sum()
        }

        pub fn add_point(&mut self, p: Point)  {
            self.points.insert(p);
        }

        pub fn set_points(&mut self, pts : HashSet<Point>)
        {
            self.points = pts;
        }

        pub(crate) fn get_points(&self) -> &HashSet<Point> {
            &self.points
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
            assert_eq!(cubes.sum() as usize,height*width*length*(CUBE_SIDES as usize));
        }
    }
}
