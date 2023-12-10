use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::LinkedList;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Eq, PartialEq, PartialOrd, Ord)]
enum PipeType {
    NotAPipe,
    VerticalPipe,
    HorizontalPipe,
    NorthEastPipe,
    NorthWestPipe,
    SouthWestPipe,
    SouthEastPipe,
    StartingPipe,
}

impl std::fmt::Display for PipeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PipeType::NotAPipe => write!(f, "Not a Pipe"),
            PipeType::VerticalPipe => write!(f, "Vertical Pipe"),
            PipeType::HorizontalPipe => write!(f, "Horizontal Pipe"),
            PipeType::NorthEastPipe => write!(f, "North East Pipe"),
            PipeType::NorthWestPipe => write!(f, "North West Pipe"),
            PipeType::SouthWestPipe => write!(f, "South West Pipe"),
            PipeType::SouthEastPipe => write!(f, "South East Pipe"),
            PipeType::StartingPipe => write!(f, "Starting Pipe"),
        }
    }
}

impl PipeType {
    fn to_connections(&self) -> ((i32, i32), (i32, i32)) {
        match self {
            PipeType::VerticalPipe => ((-1, 0), (1, 0)),
            PipeType::HorizontalPipe => ((0, -1), (0, 1)),
            PipeType::NorthWestPipe => ((-1, 0), (0, -1)),
            PipeType::NorthEastPipe => ((-1, 0), (0, 1)),
            PipeType::SouthWestPipe => ((1, 0), (0, -1)),
            PipeType::SouthEastPipe => ((1, 0), (0, 1)),
            PipeType::StartingPipe => ((2, 2), (2, 2)),
            _ => ((0, 0), (0, 0)),
        }
    }

    fn from_symbol(sym: &char) -> Self {
        match sym {
            '|' => PipeType::VerticalPipe,
            '-' => PipeType::HorizontalPipe,
            'L' => PipeType::NorthEastPipe,
            'J' => PipeType::NorthWestPipe,
            '7' => PipeType::SouthWestPipe,
            'F' => PipeType::SouthEastPipe,
            'S' => PipeType::StartingPipe,
            '.' => PipeType::NotAPipe,
            _ => PipeType::NotAPipe,
        }
    }

    #[allow(dead_code)]
    fn to_unicode(&self) -> char {
        match self {
            PipeType::VerticalPipe => '│',
            PipeType::HorizontalPipe => '─',
            PipeType::NorthEastPipe => '╰',
            PipeType::NorthWestPipe => '╯',
            PipeType::SouthWestPipe => '╭',
            PipeType::SouthEastPipe => '╮',
            PipeType::StartingPipe => '┼',
            // PipeType::StartingPipe => 'X',
            PipeType::NotAPipe => '·',
        }
    }

    fn connections_to_unicode(connections: ((i32, i32), (i32, i32))) -> char {
        match connections {
            ((-1, 0), (1, 0)) => '│',
            ((0, -1), (0, 1)) => '─',
            ((-1, 0), (0, 1)) => '╰',
            ((-1, 0), (0, -1)) => '╯',
            ((1, 0), (0, 1)) => '╭',
            ((1, 0), (0, -1)) => '╮',
            ((2, 2), (2, 2)) => '┼',
            _ => '·',
        }
    }

    fn connections_to_type(connections: &((i32, i32), (i32, i32))) -> Self {
        match connections {
            ((-1, 0), (1, 0)) => PipeType::VerticalPipe,
            ((0, -1), (0, 1)) => PipeType::HorizontalPipe,
            ((-1, 0), (0, 1)) => PipeType::NorthEastPipe,
            ((-1, 0), (0, -1)) => PipeType::NorthWestPipe,
            ((1, 0), (0, 1)) => PipeType::SouthEastPipe,
            ((1, 0), (0, -1)) => PipeType::SouthWestPipe,
            ((2, 2), (2, 2)) => PipeType::StartingPipe,
            _ => PipeType::NotAPipe,
        }
    }
}

struct Graph {
    adjacency_list: HashMap<(i32, i32), ((i32, i32), (i32, i32))>,
    rows: i32,
    cols: i32,
}

#[allow(dead_code)]
impl Graph {
    fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
            rows: 0,
            cols: 0,
        }
    }

    fn add_edge(&mut self, entry: (i32, i32), connections: ((i32, i32), (i32, i32))) {
        if let Some(existing_connections) = self.adjacency_list.get_mut(&entry) {
            println!("warning: overwriting connections of {:?}", entry);
            *existing_connections = connections;
        } else {
            self.adjacency_list.insert(entry, connections);
            self.rows = self.rows.max(entry.0);
            self.cols = self.cols.max(entry.1);
        }
    }

    fn parse_point_connections(
        &self,
        point: &(i32, i32),
        connections: &((i32, i32), (i32, i32)),
    ) -> Vec<(i32, i32)> {
        let mut conn: Vec<(i32, i32)> = Vec::new();
        if connections == &((2, 2), (2, 2)) {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 || (dx != 0 && dy != 0) {
                        continue;
                    }

                    let new_point = (point.0 + dx, point.1 + dy);
                    let new_connections = self.adjacency_list.get(&new_point);

                    if let Some(pipe_connections) = new_connections {
                        let new_parsed_conns =
                            self.parse_point_connections(&new_point, pipe_connections);
                        if new_parsed_conns.iter().any(|conn| conn == point) {
                            conn.push(new_point);
                        }
                    }
                }
            }
        } else {
            conn.push((point.0 + connections.0 .0, point.1 + connections.0 .1));
            conn.push((point.0 + connections.1 .0, point.1 + connections.1 .1));
        }
        conn
    }

    fn dfs(&self, start: &(i32, i32), visited: &mut HashSet<(i32, i32)>) {
        if visited.contains(&start) {
            return;
        }

        visited.insert(start.clone());

        if let Some(connections) = self.adjacency_list.get(start) {
            let parsed_connections = self.parse_point_connections(&start, connections);
            for conn in parsed_connections {
                self.dfs(&conn, visited);
            }
        }
    }

    fn extract_loop(&self, start: &(i32, i32)) -> HashSet<(i32, i32)> {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        self.dfs(start, &mut visited);
        visited
    }

    fn bfs(
        &self,
        start: &(i32, i32),
        visited: &mut HashSet<(i32, i32)>,
    ) -> HashMap<(i32, i32), u32> {
        let mut q: LinkedList<(i32, i32)> = LinkedList::new();
        q.push_back(start.clone());

        let mut distances: HashMap<(i32, i32), u32> = HashMap::new();
        distances.insert(start.clone(), 0);

        while let Some(current) = q.pop_front() {
            if let Some(connections) = self.adjacency_list.get(&current) {
                let parsed_connections = self.parse_point_connections(&current, connections);
                for conn in parsed_connections {
                    if visited.contains(&conn) {
                        continue;
                    }
                    visited.insert(conn.clone());
                    let distance = distances[&current] + 1;
                    distances.entry(conn.clone()).or_insert(distance);
                    q.push_back(conn.clone());
                }
            }
        }

        distances
    }

    fn farthest_from(&self, start: &(i32, i32)) -> u32 {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let distances = self.bfs(start, &mut visited);
        // self.print_maze_with_distances(&distances);
        *distances.values().max().unwrap()
    }

    fn count_inner_points(&self, circuit: &HashSet<(i32, i32)>) -> Vec<(i32, i32)> {
        let mut inner: Vec<(i32, i32)> = Vec::new();
        let mut last_corner_pipe = PipeType::NotAPipe;

        for i in 0..=self.rows {
            let mut intersections: u32 = 0;
            for j in 0..=self.cols {
                let point = (i, j);
                let node_conns = self.adjacency_list.get(&point).unwrap();

                let point_in_circuit = circuit.contains(&point);

                let mut current_pipe_type = if !point_in_circuit {
                    PipeType::NotAPipe
                } else {
                    PipeType::connections_to_type(node_conns)
                };

                // Update the type of the starting point
                if current_pipe_type == PipeType::StartingPipe {
                    let actual_pipe = self.parse_point_connections(&point, node_conns);

                    let mut first = actual_pipe.last().unwrap().clone();
                    first.0 = first.0 - point.0;
                    first.1 = first.1 - point.1;

                    let mut second = actual_pipe.first().unwrap().clone();
                    second.0 = second.0 - point.0;
                    second.1 = second.1 - point.1;

                    current_pipe_type = PipeType::connections_to_type(&(first, second));
                }

                if current_pipe_type != PipeType::HorizontalPipe && point_in_circuit {
                    if !((last_corner_pipe == PipeType::SouthEastPipe
                        && current_pipe_type == PipeType::NorthWestPipe)
                        || (last_corner_pipe == PipeType::NorthEastPipe
                            && current_pipe_type == PipeType::SouthWestPipe))
                    {
                        intersections += 1;
                    }
                }

                last_corner_pipe = if current_pipe_type == PipeType::NorthEastPipe
                    || current_pipe_type == PipeType::SouthEastPipe
                {
                    current_pipe_type
                } else {
                    last_corner_pipe
                };

                if intersections % 2 != 0 {
                    if (node_conns.0 == (0, 0) && node_conns.1 == (0, 0)) || !point_in_circuit {
                        inner.push(point);
                    }
                }
            }
        }
        inner
    }

    // Draw functions

    fn draw(&self) {
        let max_x = self
            .adjacency_list
            .keys()
            .map(|&(x, _)| x)
            .max()
            .unwrap_or(0);

        let max_y = self
            .adjacency_list
            .keys()
            .map(|&(_, y)| y)
            .max()
            .unwrap_or(0);

        for i in 0..=max_x {
            for j in 0..=max_y {
                let point = (i, j);
                if let Some(connections) = self.adjacency_list.get(&point) {
                    let symbol = PipeType::connections_to_unicode(*connections);
                    print!("{}", symbol);
                } else {
                    print!("   ");
                }
            }
            println!();
        }
    }

    fn draw_filter(&self, filter: &HashSet<(i32, i32)>) {
        let max_x = self
            .adjacency_list
            .keys()
            .map(|&(x, _)| x)
            .max()
            .unwrap_or(0);

        let max_y = self
            .adjacency_list
            .keys()
            .map(|&(_, y)| y)
            .max()
            .unwrap_or(0);

        for i in 0..=max_x {
            for j in 0..=max_y {
                let point = (i, j);
                if filter.contains(&point) {
                    if let Some(connections) = self.adjacency_list.get(&point) {
                        let symbol = PipeType::connections_to_unicode(*connections);
                        print!("{}", symbol);
                    } else {
                        print!("   ");
                    }
                } else {
                    print!("{}", PipeType::to_unicode(&PipeType::NotAPipe));
                }
            }
            println!();
        }
    }

    fn draw_inner_points(&self, inner: &Vec<(i32, i32)>, filter: &HashSet<(i32, i32)>) {
        let max_x = self
            .adjacency_list
            .keys()
            .map(|&(x, _)| x)
            .max()
            .unwrap_or(0);

        let max_y = self
            .adjacency_list
            .keys()
            .map(|&(_, y)| y)
            .max()
            .unwrap_or(0);

        for i in 0..=max_x {
            for j in 0..=max_y {
                let point = (i, j);
                if let Some(connections) = self.adjacency_list.get(&point) {
                    let symbol = PipeType::connections_to_unicode(*connections);
                    if inner.contains(&point) {
                        print!("I");
                    } else {
                        if filter.contains(&point) {
                            print!("{}", symbol);
                        } else {
                            print!("{}", PipeType::to_unicode(&PipeType::NotAPipe));
                        }
                    }
                } else {
                    print!("   ");
                }
            }
            println!();
        }
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: ./day10 <filename>");
        std::process::exit(1);
    }
    let f = File::open(&args[1])?;
    let reader = BufReader::new(f);

    let mut graph = Graph::new();
    let mut starting_point: (i32, i32) = (0, 0);
    for (i, line) in reader.lines().enumerate() {
        for (j, c) in line?.chars().enumerate() {
            let point = (i as i32, j as i32);
            let pipe = PipeType::from_symbol(&c);
            let connections = pipe.to_connections();
            starting_point = if pipe == PipeType::StartingPipe {
                point
            } else {
                starting_point
            };
            graph.add_edge(point, connections);
        }
    }

    let extracted_loop = graph.extract_loop(&starting_point);
    let inner_points = graph.count_inner_points(&extracted_loop);

    // println!("\n---------------STARTING---------------\n");
    // graph.draw();
    // println!("\n-----------------LOOP-----------------\n");
    // graph.draw_filter(&extracted_loop);
    // println!("\n-------------INNER-POINTS-------------\n");
    // graph.draw_inner_points(&inner_points, &extracted_loop);
    // println!("\n----------------RESULT----------------\n");

    let result1 = graph.farthest_from(&starting_point);
    let result2 = inner_points.len();

    println!("Result for part 1: {}", result1);
    println!("Result for part 2: {}", result2);

    Ok(())
}
