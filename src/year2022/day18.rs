mod cube_arena
{
    use bitvec::{vec::BitVec, bits, bitvec};

    /// Field of coordinates
    type K = usize;
    struct Point
    {
        x : K,
        y : K,
        z : K
    }
    pub struct CubeSpace
    {
        cubes : BitVec,
        depth: K,
        height: K,
        width: K,
    }

    impl CubeSpace
    {
        fn new(width : K, height : K, depth : K) -> CubeSpace
        {
            CubeSpace{ cubes : bitvec![0;width*height*depth], width, height, depth}
        }

        fn get(&self, Point { x, y, z } : Point) -> bool
        {
            self.cubes[z * self.depth + y * self.height + x]
        }

        fn set(&mut self, Point { x, y, z } : Point, val : bool)
        {
            self.cubes.set(z*self.depth + y*self.height + x, val);
        }
    }
}

pub struct Day18;