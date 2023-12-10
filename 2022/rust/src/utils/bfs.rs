use std::{collections::HashSet, fmt::Debug, hash::Hash};

#[derive(PartialEq, Eq, Debug)]
pub enum FieldType {
    Blank,
    Stop,
    Exit,
}

pub trait BreadthTraversable
where
    Self::Item: Eq + Hash + Clone + Copy + Debug,
{
    type Item;

    fn get_neighbours(&self, item: &Self::Item) -> Vec<Self::Item>;
    fn field_type(&self, item: &Self::Item) -> FieldType;

    fn run_bfs<R, F>(
        &self,
        start: Self::Item,
        combine: F,
        visited: &mut HashSet<Self::Item>,
    ) -> Option<R>
    where
        F: Fn(Self::Item, R) -> R,
        R: Default + Debug,
    {
        println!("[run_bfs] Started at {0:?}", start);
        let mut result = R::default();
        let mut visited_now = HashSet::new();
        let mut queue = Vec::new();
        queue.push(start);
        while let Some(current) = queue.pop() {
            if visited_now.contains(&current) {
                continue;
            }
            // println!("[run_bfs] BFS is now running, currently at {0:?} with type {1:?}",current, self.field_type(&current));
            result = combine(current, result);
            // println!("[run_bfs] result is now {0:?}",result);
            if self.field_type(&current) == FieldType::Exit || visited.contains(&current) {
                visited.extend(visited_now.iter());
                // println!("[run_bfs] found exit at {0:?}, exiting....",current);
                return None;
            }

            visited_now.insert(current);

            queue.extend(
                self.get_neighbours(&current)
                    .into_iter()
                    .filter(|n| self.field_type(n) != FieldType::Stop),
            );
            // println!("[bfs::run_bfs] queue has size {1:?}, top 5 of the queue is now {0:?}",queue.iter().rev().take(5).collect::<Vec<&Self::Item>>(), queue.len());
        }
        visited.extend(visited_now.iter());
        // println!("[run_bfs] queue empty. Result is {0:?}, stopping...",result);
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::utils::bfs::{BreadthTraversable, FieldType};

    #[test]
    fn easy_graph() {
        struct EasyGraph;
        impl BreadthTraversable for EasyGraph {
            type Item = u32;

            fn get_neighbours(&self, item: &u32) -> Vec<Self::Item> {
                match *item {
                    0 => vec![2],
                    2 => vec![1, 3, 5],
                    5 => vec![0],
                    _ => vec![],
                }
            }

            fn field_type(&self, item: &Self::Item) -> crate::utils::bfs::FieldType {
                match *item {
                    0 | 1 | 2 | 3 | 4 | 5 => FieldType::Blank,
                    _ => FieldType::Stop,
                }
            }
        }

        let result = EasyGraph.run_bfs(0, |_n, r| r + 1, &mut HashSet::new());

        assert_eq!(result, Some(5))
    }

    #[test]
    fn easy_graph_exit() {
        struct EasyGraphExit;
        impl BreadthTraversable for EasyGraphExit {
            type Item = u32;

            fn get_neighbours(&self, item: &u32) -> Vec<Self::Item> {
                match *item {
                    0 => vec![2],
                    2 => vec![1, 3, 5],
                    5 => vec![0],
                    3 => vec![100],
                    _ => vec![],
                }
            }

            fn field_type(&self, item: &Self::Item) -> crate::utils::bfs::FieldType {
                match *item {
                    0 | 1 | 2 | 3 | 4 | 5 => FieldType::Blank,
                    100 => FieldType::Exit,
                    _ => FieldType::Stop,
                }
            }
        }

        let result = EasyGraphExit.run_bfs::<u32, _>(0, |_n, r| r + 1, &mut HashSet::new());

        assert_eq!(result, None);
    }
}
