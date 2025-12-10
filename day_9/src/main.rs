use std::{
    cmp::Ordering,
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Point(f64, f64);

impl Point {
    fn calculate_area(&self, other: &Self) -> f64 {
        let width = (self.0 - other.0).abs() + 1.0;
        let height = (self.1 - other.1).abs() + 1.0;
        width * height
    }
}

#[derive(Debug)]
struct PointArea<'a>(&'a Point, &'a Point, f64);

impl<'a> Ord for PointArea<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if &self.2 < &other.2 {
            return Ordering::Less;
        } else if &self.2 > &other.2 {
            return Ordering::Greater;
        }
        return Ordering::Equal;
    }
}

impl<'a> PartialOrd for PointArea<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<'a> PartialEq for PointArea<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.2 == other.2
    }
}
impl<'a> Eq for PointArea<'a> {}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Line(i64, i64);

fn get_perimiter(points: &[Point]) -> Vec<Line> {
    let mut lines = HashSet::new();
    for (i, point_a) in points.iter().enumerate() {
        let point_b = points[(i + 1) % points.len()];

        if point_a.1 == point_b.1 {
            let start = point_a.0.min(point_b.0) as i64;
            let end = point_a.0.max(point_b.0) as i64;

            for x in start..=end {
                lines.insert(Line(x, point_a.1 as i64));
            }
        } else {
            let start = point_a.1.min(point_b.1) as i64;
            let end = point_a.1.max(point_b.1) as i64;

            for y in start..=end {
                lines.insert(Line(point_a.0 as i64, y));
            }
        }
    }

    lines.into_iter().collect::<Vec<_>>()
}

fn get_greatest_area<'a>(points: &'a [Point]) -> (&'a Point, &'a Point) {
    let mut distances = Vec::new();

    for (i, point_a) in points[..points.len() - 1].iter().enumerate() {
        for point_b in &points[i + 1..] {
            distances.push(PointArea(point_a, point_b, point_a.calculate_area(point_b)));
        }
    }

    distances.sort();
    let result = distances.into_iter().rev().next().unwrap();
    (result.0, result.1)
}

fn get_greates_area_in_permiter(points: &[Point], perminter: &[Line]) -> f64 {
    let mut best_area = f64::MAX;

    for (i, point_a) in points[..points.len() - 1].iter().enumerate() {
        for point_b in &points[i + 1..] {
            let area = point_a.calculate_area(point_b);

            if area > best_area {
                // Should not check if is bigger anyway
                continue;
            }

            let min_x = point_a.0.min(point_b.0) as i64;
            let max_x = point_a.0.max(point_b.0) as i64;
            let min_y = point_a.1.min(point_b.1) as i64;
            let max_y = point_a.1.max(point_b.1) as i64;

            let conflicts = perminter
                .iter()
                .any(|l| l.0 > min_x && l.0 < max_x && l.1 > min_y && l.1 < max_y);

            if conflicts {
                continue;
            }

            best_area = area;
        }
    }

    best_area
}

fn part1() {
    let input_file = File::open("input.txt").unwrap();
    let reader = BufReader::new(input_file);
    let mut points = Vec::new();
    for input in reader.lines() {
        let input = input.unwrap();
        let coords = input
            .split(',')
            .map(|c| c.parse::<f64>())
            .flatten()
            .collect::<Vec<_>>();
        debug_assert_eq!(coords.iter().count(), 2);

        points.push(Point(coords[0], coords[1]));
    }

    let (point_a, point_b) = get_greatest_area(&points);
    let area = point_a.calculate_area(point_b);
    println!("Largest area is = {area}");
}

fn part2() {
    let input_file = File::open("input.txt").unwrap();
    let reader = BufReader::new(input_file);
    let mut points = Vec::new();
    for input in reader.lines() {
        let input = input.unwrap();
        let coords = input
            .split(',')
            .map(|c| c.parse::<f64>())
            .flatten()
            .collect::<Vec<_>>();
        debug_assert_eq!(coords.iter().count(), 2);

        points.push(Point(coords[0], coords[1]));
    }

    let permiter = get_perimiter(&points);
    let area = get_greates_area_in_permiter(&points, &permiter);
    println!("Largest area is = {area}");
}

fn main() {
    part1();
    part2();
}


use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point(f64, f64);

impl Point {
    fn calculate_area(&self, other: &Self) -> f64 {
        let width = (self.0 - other.0).abs() + 1.0;
        let height = (self.1 - other.1).abs() + 1.0;
        width * height
    }

    // pnpoly algorithm
    fn is_inside_polygon(&self, poly: &[Point]) -> bool {
        let mut inside = false;
        let mut j = poly.len() - 1;

        for i in 0..poly.len() {
            let pi = poly[i];
            let pj = poly[j];

            if ((pi.1 > self.1) != (pj.1 > self.1))
                && (self.0
                    < (pj.0 - pi.0) * (self.1 - pi.1) / (pj.1 - pi.1) + pi.0)
            {
                inside = !inside;
            }

            j = i;
        }

        inside
    }
}

#[derive(Debug)]
struct PointArea<'a>(&'a Point, &'a Point, f64);

impl<'a> Ord for PointArea<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.2.partial_cmp(&other.2).unwrap()
    }
}

impl<'a> PartialOrd for PointArea<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for PointArea<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.2 == other.2
    }
}

impl<'a> Eq for PointArea<'a> {}

#[derive(Clone, Copy)]
struct Line {
    start: Point,
    end: Point,
}

fn lines_intersect(a: &Line, b: &Line) -> bool {
    let (ax1, ax2) = (a.start.0.min(a.end.0), a.start.0.max(a.end.0));
    let (ay1, ay2) = (a.start.1.min(a.end.1), a.start.1.max(a.end.1));
    let (bx1, bx2) = (b.start.0.min(b.end.0), b.start.0.max(b.end.0));
    let (by1, by2) = (b.start.1.min(b.end.1), b.start.1.max(b.end.1));

    ax1 <= bx2 && ax2 >= bx1 && ay1 <= by2 && ay2 >= by1
}

#[derive(Clone, Copy)]
struct Rectangle {
    tl: Point,
    br: Point,
}

impl Rectangle {
    fn area(&self) -> f64 {
        ((self.br.0 - self.tl.0).abs() + 1.0)
            * ((self.br.1 - self.tl.1).abs() + 1.0)
    }

    fn points(&self) -> [Point; 4] {
        let tr = Point(self.br.0, self.tl.1);
        let bl = Point(self.tl.0, self.br.1);
        [self.tl, tr, bl, self.br]
    }

    fn lines(&self) -> [Line; 4] {
        let [tl, tr, bl, br] = self.points();
        [
            Line { start: tl, end: tr },
            Line { start: tl, end: bl },
            Line { start: tr, end: br },
            Line { start: bl, end: br },
        ]
    }

    fn shrink(&self) -> Self {
        Self {
            tl: Point(self.tl.0 + 1.0, self.tl.1 + 1.0),
            br: Point(self.br.0 - 1.0, self.br.1 - 1.0),
        }
    }
}

fn get_greatest_area<'a>(points: &'a [Point]) -> (&'a Point, &'a Point) {
    let mut distances = Vec::new();

    for (i, a) in points[..points.len() - 1].iter().enumerate() {
        for b in &points[i + 1..] {
            distances.push(PointArea(a, b, a.calculate_area(b)));
        }
    }

    distances.sort();
    let best = distances.pop().unwrap();
    (best.0, best.1)
}

fn get_greatest_area_in_polygon(points: &[Point]) -> f64 {
    let mut best = 0.0;

    for (i, a) in points.iter().enumerate() {
        for b in &points[i + 1..] {
            let tl = Point(a.0.min(b.0), a.1.min(b.1));
            let br = Point(a.0.max(b.0), a.1.max(b.1));
            let rect = Rectangle { tl, br }.shrink();

            // All rectangle corners must be inside polygon
            if !rect
                .points()
                .iter()
                .all(|p| p.is_inside_polygon(points))
            {
                continue;
            }

            // Rectangle edges must not cross polygon edges
            let poly_lines = points.iter().enumerate().map(|(i, p)| Line {
                start: *p,
                end: points[(i + 1) % points.len()],
            });

            let mut invalid = false;
            for rl in rect.lines() {
                for pl in poly_lines.clone() {
                    if lines_intersect(&rl, &pl) {
                        invalid = true;
                        break;
                    }
                }
                if invalid {
                    break;
                }
            }

            best = best.max(rect.area());
        }
    }

    best
}

fn read_points() -> Vec<Point> {
    let input_file = File::open("input.txt").unwrap();
    let reader = BufReader::new(input_file);

    reader
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let (x, y) = l.split_once(',').unwrap();
            Point(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

fn part1() {
    let points = read_points();
    let (a, b) = get_greatest_area(&points);
    println!("Part 1: {}", a.calculate_area(b));
}

fn part2() {
    let points = read_points();
    let area = get_greatest_area_in_polygon(&points);
    println!("Part 2: {area}");
}

fn main() {
    part1();
    part2();
}
