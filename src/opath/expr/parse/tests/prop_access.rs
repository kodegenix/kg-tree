use crate::opath::*;
use crate::opath::Expr::*;

mod simple {
    use super::*;

    #[test]
    fn current() {
        assert_expr!("name",
        Sequence(
            vec![
                Current,
                Property(box Id::new("name"))
                ]))
    }

    #[test]
    fn current_at() {
        assert_expr!("@.name",
        Sequence(
            vec![
                Current,
                Property(Box::new(Id::new("name")))
                ]))
    }


    #[test]
    fn current_bracket_at() {
        assert_expr!("@[name]",
        Sequence(vec![
                Current,
                IndexExpr(Box::new(
                        Sequence(vec![
                            Current,
                            Property(Box::new(Id::new("name")))
                        ])
                ))
        ]))
    }

    #[test]
    fn current_quot_at() {
        assert_expr!("@.'name'",
        Sequence(
            vec![
                Current,
                Property(Box::new(Id::new("name")))
                ]))
    }

    #[test]
    fn current_double_quot_at() {
        assert_expr!("@.\"name\"",
        Sequence(
            vec![
                Current,
                Property(Box::new(Id::new("name")))
                ]))
    }

    #[test]
    fn current_bracket_double_quot_at() {
        assert_expr!("@[\"name\"]",
        Sequence(vec![
                Current,
                Property(Box::new(Id::new("name")))
        ]))
    }

    #[test]
    fn current_prop_index() {
        assert_expr!("@[1]",
        Sequence(
            vec![
                Current,
                Index(1)
                ]))
    }


    #[test]
    fn root() {
        assert_expr!("$.name",
        Path(vec![PathSegment::Key(Id::new("name"))]))
    }


    #[test]
    fn root_bracket() {
        assert_expr!("$[name]",
                     Sequence(vec![Root, IndexExpr(box Sequence(vec![Current, Property(box Id::new("name"))]))]));
    }

    #[test]
    fn root_quot() {
        assert_expr!("$.'name'",
        Path(vec![PathSegment::Key(Id::new("name"))]))
    }

    #[test]
    fn root_quot_whitespace() {
        assert_expr!("$.'some name'",
        Path(vec![PathSegment::Key(Id::new("some name"))]))
    }

    #[test]
    fn root_double_quot() {
        assert_expr!("$.\"name\"",
        Path(vec![PathSegment::Key(Id::new("name"))]))
    }

    #[test]
    fn root_bracket_double_quot() {
        assert_expr!("$[\"name\"]",
        Path(vec![PathSegment::Key(Id::new("name"))]))
    }

    #[test]
    fn root_prop_index() {
        assert_expr!("$[1]",
        Path(vec![PathSegment::Index(1)]))
    }

    #[test]
    fn var() {
        assert_expr!("$var1", Var(box Id::new("var1")));
    }

    #[test]
    fn var_expr() {
        assert_expr!("${'var' + 1}", VarExpr(box Add(box String("var".to_string()), box Integer(1))));
    }
}

mod wildcards {
    use std::i64;

    use super::*;

    #[test]
    fn dot_star() {
        assert_expr!("@.*",
                Sequence(
                    vec![
                        Current,
                        All
                        ]))
    }

    #[test]
    fn bracket_star() {
        assert_expr!("@[*]",
                Sequence(
                    vec![
                        Current,
                        IndexExpr(Box::new(All))
                        ]))
    }

    #[test]
    fn dot_double_star() {
        assert_expr!("@.**",
                Sequence(
                    vec![
                        Current,
                        Descendants(Box::new(
                            LevelRange::default()))
                        ]))
    }

    #[test]
    fn dot_quot_double_star() {
        assert_expr!("@.'**'",
                Sequence(
                    vec![
                        Current,
                        Property(Box::new(Id::new("**")))
                        ]))
    }

    #[test]
    fn bracket_double_star() {
        assert_expr!("@[**]",
                Sequence(
                    vec![
                        Current,
                        IndexExpr(
                            box Descendants(box LevelRange::default())
                        )
                        ]))
    }

    #[test]
    fn bracket_quot_double_star() {
        assert_expr!("@['**']",
                Sequence(
                    vec![
                        Current,
                        Property(Box::new(Id::new("**")))
                        ]))
    }

    #[test]
    fn double_star_depth_min_max() {
        assert_expr!("@.**{2,4}",
                Sequence(vec![
                        Current,
                        Descendants(Box::new(
                            LevelRange {
                                min: Integer(2),
                                max: Integer(4),
                            }
                        ))
                ]))
    }

    #[test]
    fn double_star_depth_max() {
        assert_expr!("@.**{,4}",
                Sequence(vec![
                        Current,
                        Descendants(Box::new(
                            LevelRange {
                                min: Integer(1),
                                max: Integer(4),
                            }
                        ))
                ]))
    }

    #[test]
    fn double_star_depth() {
        assert_expr!("@.**{2}",
                Sequence(vec![
                        Current,
                        Descendants(Box::new(
                            LevelRange {
                                min: Integer(2),
                                max: Integer(i64::MAX),
                            }
                        ))
                ]))
    }

    #[test]
    fn parent() {
        assert_expr!("@^",
                Sequence(vec![
                        Current,
                        Parent
                ]))
    }

    #[test]
    fn property_parent() {
        assert_expr!("@.prop^",
                Sequence(vec![
                        Current,
                        Property(Box::new(Id::new("prop"))),
                        Parent
                ]))
    }

    #[test]
    fn nested_property_parent() {
        assert_expr!("@.nested.prop^",
                Sequence(vec![
                        Current,
                        Property(Box::new(Id::new("nested"))),
                        Property(Box::new(Id::new("prop"))),
                        Parent
                ]))
    }

    #[test]
    fn ancestors() {
        assert_expr!("@^**",
                    Sequence(
                        vec![
                            Current,
                            Ancestors(Box::new(
                                LevelRange::default()))
                            ]))
    }

    #[test]
    fn ancestors_depth_min_max() {
        assert_expr!("@^**{2,4}",
                    Sequence(
                        vec![
                            Current,
                            Ancestors(Box::new(
                                LevelRange {
                                    min: Integer(2),
                                    max: Integer(4),
                                }))
                            ]))
    }

    #[test]
    fn ancestors_depth_max() {
        assert_expr!("@^**{,4}",
                    Sequence(
                        vec![
                            Current,
                            Ancestors(Box::new(
                                LevelRange {
                                    min: Integer(1),
                                    max: Integer(4),
                                }))
                            ]))
    }

    #[test]
    fn ancestors_depth_min() {
        assert_expr!("@^**{2}",
                    Sequence(
                        vec![
                            Current,
                            Ancestors(Box::new(
                                LevelRange {
                                    min: Integer(2),
                                    max: Integer(i64::MAX),
                                }))
                            ]))
    }
}


