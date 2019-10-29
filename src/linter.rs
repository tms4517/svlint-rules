use crate::config::Config;
use std::path::PathBuf;
use sv_parser::{unwrap_locate, Locate, RefNode, SyntaxTree};

#[derive(Clone, Copy)]
pub enum RuleResult {
    Pass,
    Fail,
    FailAt(usize, usize),
    FailLocate(Locate),
}

pub trait Rule {
    fn check(&self, syntax_tree: &SyntaxTree, node: &RefNode) -> RuleResult;
    fn name(&self) -> String;
    fn hint(&self) -> String;
}

pub struct Linter {
    rules: Vec<Box<dyn Rule>>,
}

#[derive(Debug)]
pub struct LintFailed {
    pub path: PathBuf,
    pub beg: usize,
    pub len: usize,
    pub name: String,
    pub hint: String,
}

impl Linter {
    pub fn new(config: Config) -> Linter {
        let rules = config.gen_rules();
        Linter { rules }
    }

    pub fn check(&self, syntax_tree: &SyntaxTree, node: &RefNode) -> Vec<LintFailed> {
        let locate = if let Some(x) = unwrap_locate!(node.clone()) {
            x
        } else {
            return vec![];
        };

        let mut ret = Vec::new();
        for rule in &self.rules {
            match rule.check(&syntax_tree, &node) {
                RuleResult::Fail => {
                    if let Some((path, beg)) = syntax_tree.get_origin(&locate) {
                        let result = LintFailed {
                            path: path.clone(),
                            beg,
                            len: locate.len,
                            name: rule.name(),
                            hint: rule.hint(),
                        };
                        ret.push(result);
                    }
                }
                RuleResult::FailAt(offset, len) => {
                    if let Some((path, beg)) = syntax_tree.get_origin(&locate) {
                        let result = LintFailed {
                            path: path.clone(),
                            beg: beg + offset,
                            len,
                            name: rule.name(),
                            hint: rule.hint(),
                        };
                        ret.push(result);
                    }
                }
                RuleResult::FailLocate(x) => {
                    if let Some((path, beg)) = syntax_tree.get_origin(&x) {
                        let result = LintFailed {
                            path: path.clone(),
                            beg,
                            len: x.len,
                            name: rule.name(),
                            hint: rule.hint(),
                        };
                        ret.push(result);
                    }
                }
                _ => (),
            }
        }
        ret
    }
}
