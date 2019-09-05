macro_rules! parse_node {
    ($input: expr) => {{
        let mut r = kg_diag::MemCharReader::new($input.as_bytes());
        let mut parser = Parser::new();
        parser.parse(&mut r).unwrap_or_else(|err| {
            eprintln!("{}", err);
            panic!("Error parsing node!")
        })
    }};
}

macro_rules! parse_node_err {
    ($input: expr) => {{
        let mut r = kg_diag::MemCharReader::new($input.as_bytes());
        let mut parser = Parser::new();
        let err = parser
            .parse(&mut r)
            .map(|node| panic!("Error expected! got node: {}", node.to_json_pretty()))
            .unwrap_err();
        err
    }};
}


mod json;
mod toml;
mod yaml;
