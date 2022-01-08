use std::{collections::HashSet, hash::Hash, num::ParseIntError, str};

advent_of_code::day!(09);

type ParseError = ParseIntError;

#[derive(Debug)]
pub struct Heightmap {
    inner: Vec<u8>,
    width: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    height: u8,
    idx: usize,
}

#[derive(Debug)]
pub struct Basin {
    points: Vec<Point>,
}

fn depth_first_search<T, Ns, P>(node: T, neighbours: Ns, predicate: P) -> HashSet<T>
where
    T: Hash + Eq,
    Ns: Fn(&T) -> Vec<T>,
    P: Fn(&T) -> bool,
{
    let mut visited = HashSet::new();
    let mut stack = Vec::new();

    stack.push(node);
    while !stack.is_empty() {
        let node = stack.pop().unwrap();
        if !visited.contains(&node) {
            for neigbhour in neighbours(&node).into_iter().filter(|n| predicate(n)) {
                stack.push(neigbhour);
            }
            visited.insert(node);
        }
    }

    visited
}

impl Heightmap {
    fn point(&self, idx: usize) -> Option<Point> {
        let height = *self.inner.get(idx)?;

        Some(Point { height, idx })
    }

    fn neighbours(&self, idx: usize) -> Vec<Point> {
        let idx = idx as isize;
        let width = self.width as isize;

        [
            Some(idx - width),
            Some(idx + width),
            if idx % width == 0 {
                None
            } else {
                Some(idx - 1)
            },
            if idx % width == width - 1 {
                None
            } else {
                Some(idx + 1)
            },
        ]
        .iter()
        .filter_map(|&idx| self.point(idx? as usize))
        .collect::<Vec<_>>()
    }

    fn low_points(&self) -> Vec<Point> {
        self.inner
            .iter()
            .enumerate()
            .filter(|&(idx, &height)| self.neighbours(idx).iter().all(|n| n.height > height))
            .map(|(idx, &height)| Point { height, idx })
            .collect::<Vec<_>>()
    }

    fn basins(&self) -> Vec<Basin> {
        let low_points = self.low_points();

        low_points.into_iter().map(|p| self.basin(p)).collect()
    }

    fn basin(&self, point: Point) -> Basin {
        let points = depth_first_search(point, |p| self.neighbours(p.idx), |p| p.height != 9);
        let points = Vec::from_iter(points);

        Basin { points }
    }
}

impl str::FromStr for Heightmap {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.find('\n').unwrap_or_else(|| s.len());

        let inner = s
            .chars()
            .filter(|&c| c != '\n')
            .map(|c| {
                let mut buf = [0; 4];
                c.encode_utf8(&mut buf).parse::<u8>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Heightmap { inner, width })
    }
}

impl<'a> advent_of_code::Solution<'a> for Day09 {
    type Input = Box<Heightmap>;
    type ParseError = ParseError;

    type P1 = u32;
    type P2 = usize;

    fn parse(input: &'a str) -> Result<Self::Input, Self::ParseError> {
        input.parse().map(Box::new)
    }

    fn part1(input: &Heightmap) -> Self::P1 {
        input
            .low_points()
            .iter()
            .map(|Point { height, .. }| (height + 1) as u32)
            .sum::<u32>()
    }

    fn part2(input: &Heightmap) -> Self::P2 {
        let mut basins = input.basins();
        basins.sort_unstable_by_key(|b| b.points.len());

        basins
            .iter()
            .rev()
            .take(3)
            .map(|Basin { points }| points.len())
            .product()
    }
}

#[cfg(test)]
mod tests {
    use advent_of_code::Solution;

    const INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test() {
        assert_eq!(super::Day09::solve(INPUT), Ok((15, 1_134)));
    }
}
