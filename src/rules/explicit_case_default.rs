use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_locate, unwrap_node, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct ExplicitCaseDefault;

impl Rule for ExplicitCaseDefault {
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
                if let Some(x) = unwrap_node!(*x, CaseStatementNormal) {
                    let loc = unwrap_locate!(x.clone()).unwrap();
                    let a = unwrap_node!(x, CaseItemDefault);
                    if a.is_some() {
                        RuleResult::Pass
                    } else {
                        RuleResult::FailLocate(*loc)
                    }
                } else {
                    RuleResult::Pass
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("explicit_case_default")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("`case` must have `default` in `always*`")
    }

    fn reason(&self) -> String {
        String::from("explicit `default` makes design intent clearer")
    }
}
