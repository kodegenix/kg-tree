use super::*;

pub trait ResolveStrategy {
    fn resolve_interpolation(
        &mut self,
        interpolation: &Interpolation,
        node: &NodeRef,
        parent: &NodeRef,
        root: &NodeRef,
    ) -> Option<NodeRef>;
}

#[derive(Debug)]
pub struct DefaultResolveStrategy;

impl ResolveStrategy for DefaultResolveStrategy {
    fn resolve_interpolation(
        &mut self,
        interpolation: &Interpolation,
        _node: &NodeRef,
        parent: &NodeRef,
        root: &NodeRef,
    ) -> Option<NodeRef> {
        interpolation.resolve(root, parent)
    }
}

#[derive(Debug)]
pub struct RootedResolveStrategy;

impl ResolveStrategy for RootedResolveStrategy {
    fn resolve_interpolation(
        &mut self,
        interpolation: &Interpolation,
        _node: &NodeRef,
        _parent: &NodeRef,
        root: &NodeRef,
    ) -> Option<NodeRef> {
        interpolation.resolve(root, root)
    }
}

#[derive(Debug)]
pub struct TreeResolver {
    parser: self::interpolation::Parser,
}

impl TreeResolver {
    #[inline]
    pub fn new() -> TreeResolver {
        Self::with_parser(self::interpolation::Parser::new())
    }

    #[inline]
    pub fn with_delims(open_delim: &str, close_delim: &str) -> TreeResolver {
        Self::with_parser(self::interpolation::Parser::with_delims(
            open_delim,
            close_delim,
        ))
    }

    #[inline]
    pub fn with_parser(parser: self::interpolation::Parser) -> TreeResolver {
        TreeResolver { parser }
    }

    pub fn resolve(&mut self, root: &NodeRef) {
        self.resolve_custom(DefaultResolveStrategy, root)
    }

    pub fn resolve_custom<P>(&mut self, mut strategy: P, root: &NodeRef)
    where
        P: ResolveStrategy,
    {
        let mut replacements = Vec::new();
        let mut iter = 0;

        root.visit_recursive(|_r, p, n| {
            if n.is_string() {
                let i = self
                    .parser
                    .parse_str(&n.data().as_string())
                    .unwrap_or(Interpolation::Empty);
                if !i.is_empty() {
                    let index = n.data().metadata().index();
                    let key = Symbol::from(n.data().metadata().key());
                    replacements.push((i, p.clone(), index, key));
                }
            }
            true
        });

        if replacements.is_empty() {
            return;
        }

        loop {
            iter += 1;
            if iter == 100 {
                panic!("too many iterations while resolving interpolations"); //FIXME (jc) add proper error handling
            }

            let mut change = false;
            for (i, p, index, key) in replacements.iter() {
                if let Some(nn) = strategy.resolve_interpolation(&i, &p, &p, root) {
                    let n = p.get_child_index(*index).unwrap();
                    if !n.is_identical_deep(&nn) {
                        change = true;
                        p.set_child(Some(*index), Some(key.clone()), nn.into_consumable())
                            .unwrap();
                    }
                }
            }

            if !change {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_interpolate_recursively() {
        let n = NodeRef::from_json(
            r#"
            {
                "child1": {
                    "my_key": "<% @key %>",
                    "sub2": "<% @.subchild %>",
                    "subchild": {
                        "my_key": "<% @^.my_key %>"
                    }
                }
            }
        "#,
        )
        .unwrap();

        let mut r = TreeResolver::new();
        r.resolve(&n);

        assert_eq!(n.to_json(), r#"{"child1":{"my_key":"child1","sub2":{"my_key":"child1"},"subchild":{"my_key":"child1"}}}"#);
    }
}
