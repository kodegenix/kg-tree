use super::*;

use std::collections::HashMap;


pub trait ResolveStrategy {
    fn resolve_interpolation(&mut self, interpolation: &Interpolation, node: &NodeRef, parent: &NodeRef, root: &NodeRef) -> Option<NodeRef>;
}

#[derive(Debug)]
pub struct DefaultResolveStrategy;

impl ResolveStrategy for DefaultResolveStrategy {
    fn resolve_interpolation(&mut self, interpolation: &Interpolation, _node: &NodeRef, parent: &NodeRef, root: &NodeRef) -> Option<NodeRef> {
        interpolation.resolve(root, parent)
    }
}


#[derive(Debug)]
pub struct RootedResolveStrategy;

impl ResolveStrategy for RootedResolveStrategy {
    fn resolve_interpolation(&mut self, interpolation: &Interpolation, _node: &NodeRef, _parent: &NodeRef, root: &NodeRef) -> Option<NodeRef> {
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
        Self::with_parser(self::interpolation::Parser::with_delims(open_delim, close_delim))
    }

    #[inline]
    pub fn with_parser(parser: self::interpolation::Parser) -> TreeResolver {
        TreeResolver {
            parser,
        }
    }

    pub fn resolve(&mut self, root: &NodeRef) {
        self.resolve_custom(DefaultResolveStrategy, root)
    }

    pub fn resolve_custom<P>(&mut self, mut strategy: P, root: &NodeRef) where P: ResolveStrategy {
        let mut exprs: HashMap<Symbol, Interpolation> = HashMap::new();
        let mut replacements = Vec::new();
        let mut iter = 0;

        loop {
            iter += 1;
            if iter == 100 {
                panic!("too many iterations while resolving interpolations"); //FIXME (jc) add proper multi.rs handling
            }

            root.visit_recursive(|r, p, n| {
                if n.is_string() {
                    let nd = n.data();
                    let s = nd.as_string();
                    let i = exprs.entry(s.as_ref().into()).or_insert_with(|| {
                        self.parser.parse_str(&s).unwrap_or(Interpolation::Empty)
                    });
                    if let Some(nn) = strategy.resolve_interpolation(i, n, p, r) {
                        let nn = if !nn.is_consumable() {
                            nn.deep_copy()
                        } else {
                            nn
                        };
                        replacements.push((p.clone(), n.clone(), nn));
                    }
                }
                true
            });

            if replacements.is_empty() {
                break;
            }

            for (p, o, n) in replacements.drain(..) {
                let (index, key) = {
                    let od = o.data();
                    if n.data().file().is_none() {
                        n.data_mut().set_file(od.metadata().file());
                    }
                    (od.index(), od.key().to_string())
                };
                p.add_child(Some(index), Some(key.into()), n).unwrap();
            }
        }
    }
}


