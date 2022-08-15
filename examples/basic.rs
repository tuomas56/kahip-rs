fn main() {
    let graph = kahip::Graph::new(
        6, [
            (0, 1),
            (0, 2),
            (1, 2),
            (0, 3),
            (0, 4),
            (3, 4),
            (5, 1),
            (5, 2),
            (5, 3),
            (5, 4)
        ]
    );

    println!("{:?}", graph.node_separator(2, 0.0, kahip::Mode::Fast, 1234, false));
}