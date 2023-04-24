use std::{cmp::max, ops, collections::{HashSet, HashMap}};

use bitvec::{vec::BitVec, bitvec};

use crate::{day0::Day, year2022::day17::embedding::intersect};

use self::embedding::EmbeddedShape;

pub struct Day17;

#[derive(Debug, Clone, Copy)]
pub enum Move {
    Left,
    Right,
}

impl TryFrom<char> for Move {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '>' => Ok(Move::Right),
            '<' => Ok(Move::Left),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq,Eq)]
struct Point {
    x: usize,
    y: usize,
}
#[derive(Debug, Clone, Copy, PartialEq,Eq)]
struct Vector {
    x: usize,
    y: usize,
}

#[derive(Debug,Clone, Copy,PartialEq,Eq)]
struct Shape(&'static [Vector]);

const SHAPES: &[Shape] = &[
    Shape(&[
        Vector { x: 0, y: 0 },
        Vector { x: 1, y: 0 },
        Vector { x: 2, y: 0 },
        Vector { x: 3, y: 0 },
    ]),
    Shape(&[
        Vector { x: 0, y: 1 },
        Vector { x: 1, y: 0 },
        Vector { x: 1, y: 1 },
        Vector { x: 2, y: 1 },
        Vector { x: 1, y: 2 },
    ]),
    Shape(&[
        Vector { x: 0, y: 0 },
        Vector { x: 1, y: 0 },
        Vector { x: 2, y: 0 },
        Vector { x: 2, y: 1 },
        Vector { x: 2, y: 2 },
    ]),
    Shape(&[
        Vector { x: 0, y: 0 },
        Vector { x: 0, y: 1 },
        Vector { x: 0, y: 2 },
        Vector { x: 0, y: 3 },
    ]),
    Shape(&[
        Vector { x: 0, y: 0 },
        Vector { x: 1, y: 0 },
        Vector { x: 0, y: 1 },
        Vector { x: 1, y: 1 },
    ]),
];

impl ops::Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
mod embedding{
    use std::marker::PhantomData;

    use super::{Point, Shape};

    

    #[derive(Debug,Clone, Copy,PartialEq,Eq)]
    pub struct EmbeddedShape {
        pub(super) p: Point,
        pub(super) shape: Shape,
        phantom : PhantomData<u32>
    }

    impl EmbeddedShape
    {
        pub(super) fn new(p : Point, shape : Shape) -> EmbeddedShape
        {
            // if p.x < 1{
            //     panic!("x=0 is a wall. A shape shouldn't be inside a wall.");
            // }
            EmbeddedShape { p, shape, phantom: PhantomData }
        }
    }

    impl PartialOrd for EmbeddedShape
    {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.p.y.partial_cmp(&other.p.y)
        }
    }


    pub fn intersect(s1: EmbeddedShape, s2: EmbeddedShape) -> bool {
        for &v1 in s1.shape.0 {
            for &v2 in s2.shape.0 {
                if s1.p + v1 == s2.p + v2 {
                    //     println!("intersection found! vertex {0:?} from the shape at point {1:?} meets with the vertex {2:?} form the shape at point {3:?}",v1,s1.p,v2,s2.p);
                    return true;
                }
            }
        }
        false
    }
}
/// A function that takes a slice of a tetris board. From the list of shapes produces a bit array (ehh I need to get better at Rust)
fn rasterize<const W : usize, const DH : usize>(objs : &[EmbeddedShape], h0 : usize) -> BitVec // [bool ; W * DH] 
{
    // x: 1..=7, y: 0..
    // let heights = h0..h0+DH;
    let size : usize = W*DH ;
    let mut result = bitvec![0; size];

    let mut set = |p : Point| {
        let idx = p.y.checked_sub(h0).and_then(|x| TryInto::<usize>::try_into(x*W+(p.x-1)).ok());
        if let Some(i) = idx{
            let r = result.get_mut(i);
            if let Some(mut v) = r{
                *v = true;
            } 
        }
    };

    for o in objs
    {
        for v in o.shape.0
        {
            set(o.p+*v);
        }
    }

    result
}
#[derive(Debug)]
struct TetrisInfo
{
    shapes : usize,
    moves_idx : usize,
    hmax : usize
}

struct TetrisCache
{
    cache : HashSet<BitVec>,
    infos : HashMap<BitVec,TetrisInfo>
}

fn tetris<const W : usize, F>(n: usize, moves: Vec<Move>, _draw: F) -> usize
where
    F: Fn(&[EmbeddedShape], &EmbeddedShape, usize),
{
    let mut world : Vec<EmbeddedShape> = Vec::new();
    let mut world_cache = TetrisCache{ cache: HashSet::new(), infos : HashMap::new()};
    let mut h = 0;
    let mut vh = 0;
    let mut j = 0;
    let mut i = 0;
    while i < n {
        let shape = SHAPES[i % SHAPES.len()];
        let mut p = Point { x: 3, y: h + 4 }; // powinno być 3 ale
        let mut jet = false; // zaczynamy od spadania 1 w dół
        'fall: loop {
            jet = !jet;
            // _draw(&world, &EmbeddedShape::new( p, shape ), h.saturating_sub(20));
            let pn = do_move(jet, moves[j % moves.len()], p);
            if jet {
                j += 1;
            }
            let curr = EmbeddedShape::new(pn, shape);
            for emb in &world {
                // dbg!(emb.p.y);
                if intersect(*emb, curr) {
                    if !jet {
                        //         println!("fatal: block impact");
                        break 'fall;
                    } else {
                        //             println!("fail: block");
                        continue 'fall;
                    }
                }
            }
            if pn.x == 0 {
                //     println!("fail: left wall");
                continue 'fall;
            }
            if pn.x + max_x(shape) > W {
                //     println!("fail: right wall");
                continue 'fall;
            }
            if pn.y == 0 {
                //     println!("fatal: floor impact");
                break 'fall;
            }

            p = pn;
            // _draw(&world[0..10.min(world.len())],&EmbeddedShape::new(p, shape ),h.saturating_sub(3));
        }
        let new_elem =EmbeddedShape::new(p, shape ); 
        match world.binary_search_by(|e| new_elem.p.y.cmp(&e.p.y) )
        {
            Ok(pos) => world.insert(pos,new_elem),
            Err(pos) => world.insert(pos,new_elem),
        }
        i += 1;
        h = max(h, p.y + max_y(shape));
        const DH :usize = 3;
        let snapshot = rasterize::<W,DH>(&world[0..(DH*W).min(world.len())],h.saturating_sub(3)); // cannot be more than dh*w 

        // dbg!(&snapshot);
        if check_snapshot::<W>(&snapshot) && vh == 0{
            if !world_cache.cache.insert(snapshot.clone())
            {
                // _draw(&world[0..10.min(world.len())],&new_elem,h.saturating_sub(3));
                // kończymy
                // konfiguracja się powtórzyła, więc wiemy jaka będzie przyszłość, mamy wzór na obliczenie wysokości dla dowolnego n
                // dbg!(&world_cache.infos[&snapshot]);
                // dbg!(&snapshot);
                let TetrisInfo { shapes, moves_idx, hmax } = world_cache.infos[&snapshot];
                if moves_idx % moves.len() != j % moves.len() {continue;}
                if shapes % SHAPES.len() != i % SHAPES.len() {continue;}
                // println!("Match found. KOŃCZYYYMYYY");
                let dh = h - hmax;
                let di = i - shapes;
                // let dj = (j + moves.len() - moves_idx) % moves.len(); // obejście tego że -1 % 5 = -1 -_-...

                let m = (n - i)/di; 
                // dbg!(m);
                vh += (m )*dh;
                // j += m*dj;
                i += m*di;
            }
            else
            {
                world_cache.infos.insert(snapshot,TetrisInfo { shapes: i, moves_idx: j, hmax: h });
                // dbg!(world_cache.infos.len());
                // println!("World cache, theoretical maximum: {0}, current: {1}",2_usize.pow((DH*W) as u32),world_cache.infos.len());
            }
            }        
    }

    h + vh
}

fn check_snapshot<const W : usize>(snapshot: &BitVec) -> bool {
    let mut columns = 0;
    for i in 0..W{
        let mut j = 0;
        let any1 = loop
        {
            if i + j*W >= snapshot.len()
            {
                break false;
            }
            if snapshot[i+j*W] {
                break true;
            } 
            j += 1;
        };
        if !any1 {columns += 1;}
    }
    columns == 0
}

fn max_x(shape: Shape) -> usize {
    shape.0.iter().map(|p| p.x).max().unwrap()
}

fn max_y(shape: Shape) -> usize {
    shape.0.iter().map(|p| p.y).max().unwrap()
}

fn do_move(jet: bool, j: Move, p: Point) -> Point {
    if !jet {
        Point { y: p.y - 1, ..p }
    } else {
        match j {
            Move::Left => Point { x: p.x - 1, ..p },
            Move::Right => Point { x: p.x + 1, ..p },
        }
    }
}

impl Day<2022, 17, Vec<Move>, usize> for Day17 {
    fn solve(input: Vec<Move>) -> usize {
        const W: usize = 7;

        tetris::<W,_>(2022, input, draw::<W,20>)
    }

    fn solve2(input: Vec<Move>) -> usize {
        const W : usize = 7;
        tetris::<W,_>(1000000000000, input, draw::<7,20>)
    }

    fn parse(input: &str) -> Vec<Move> {
        input
            .chars()
            .map(Move::try_from)
            .map(Result::unwrap)
            .collect()
    }
}

fn draw<const W : usize, const H : usize>(world: &[EmbeddedShape], curr: &EmbeddedShape, h: usize) {
        #[derive(Clone, Copy)]
        enum State {
            Blank,
            Stale,
            Falling,
        }
        fn state_str(s: &State) -> &str {
            match s {
                State::Blank => ".",
                State::Stale => "#",
                State::Falling => "@",
            }
        }
        let w = W + 2;
        let mut board = vec![State::Blank; H * w];
        for e in world {
            for &v in e.shape.0 {
                let p = e.p + v;
                if p.y > h && w * (p.y - 1 - h) + (p.x) < board.len(){
                    board[w * (p.y - 1 - h) + (p.x) ] = State::Stale;
                }
            }
        }
        for &v in curr.shape.0 {
            let p = curr.p + v;
            if p.y > h && w * (p.y - 1 - h) + (p.x) < board.len(){
                board[w * (p.y - 1 - h) + (p.x) ] = State::Falling;
            }
        }
        let mut s = String::new();
        for y in 0..H {
            for x in 0..w {
                if x == 0 || x == w - 1 {
                    s.push('|');
                } else {
                    s.push_str(state_str(&board[w * (H - y - 1) + x ]));
                }
            }
            if y != H - 1 {
                s.push('\n');
            }
        }
        println!("{}",s);
        println!("+-------+");
    }

#[cfg(test)]
mod tetris_tests
{
    use super::{SHAPES, EmbeddedShape, Point, rasterize};
    use bitvec::prelude::*;
  #[test]
    fn rasterize_test()
    {
        let l = SHAPES[2];
        let shape = EmbeddedShape::new(Point { x: 2, y: 1 },l);

        let result = rasterize::<7,4>(&[shape],0);
        dbg!(&result);
        let should = 
        bits![
        0, 0, 0, 0, 0,0,0,
        0, 1,  1,  1, 0,0,0,
        0, 0, 0, 1, 0,0,0,
        0, 0, 0, 1, 0,0,0,
        ];
        assert_eq!(result,should);
    }  
  #[test]
    fn rasterize_out_of_bounds()
    {
        let l = SHAPES[1];
        let shape = EmbeddedShape::new(Point { x: 2, y: 2 },l);

        let result = rasterize::<7,4>(&[shape],0);
        dbg!(&result);
        let should = 
        bits![
        0, 0, 0, 0, 0,0,0,
        0, 0,  0,  0, 0,0,0,
        0, 0, 1, 0, 0,0,0,
        0, 1, 1, 1, 0,0,0,
        ];
        assert_eq!(result,should);
    }  
}
