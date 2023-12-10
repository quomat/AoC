pub mod bfs;

pub mod tuple_map {
    pub trait TupleMap {
        type U;
        type Wrapped<B>: TupleMap;

        fn tmap<B, F>(self, f: F) -> Self::Wrapped<B>
        where
            F: Fn(Self::U) -> B;
    }

    impl<T> TupleMap for (T, T) {
        type U = T;

        type Wrapped<B> = (B, B);

        fn tmap<B, F>(self, f: F) -> Self::Wrapped<B>
        where
            F: Fn(Self::U) -> B,
        {
            (f(self.0), f(self.1))
        }
    }
    impl<T> TupleMap for (T, T, T) {
        type U = T;

        type Wrapped<B> = (B, B, B);

        fn tmap<B, F>(self, f: F) -> Self::Wrapped<B>
        where
            F: Fn(Self::U) -> B,
        {
            (f(self.0), f(self.1), f(self.2))
        }
    }
}
