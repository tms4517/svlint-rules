use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_locate, unwrap_node, AlwaysKeyword, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct SequentialBlockInAlwaysFf;

impl Rule for SequentialBlockInAlwaysFf {
    fn check(
        &mut self,
        _syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> RuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Pass;
            }
        };

        match node {
            RefNode::AlwaysConstruct(x) => {
                let (t, x) = &x.nodes;
                match t {
                    AlwaysKeyword::AlwaysFf(_) => {
                        if let Some(x) = unwrap_node!(x, SeqBlock) {
                            let loc = unwrap_locate!(x.clone()).unwrap();
                            RuleResult::FailLocate(*loc)
                        } else {
                            RuleResult::Pass
                        }
                    }
                    _ => RuleResult::Pass,
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("sequential_block_in_always_ff")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("begin/end forbidden within `always_ff` constuct")
    }

    fn reason(&self) -> String {
        String::from("prevent introducing sequential dependencies")
    }
}
