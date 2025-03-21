use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{ForInitialization, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct LoopVariableDeclaration;

impl Rule for LoopVariableDeclaration {
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
            RefNode::ForInitialization(ForInitialization::ListOfVariableAssignments(_)) => {
                RuleResult::Fail
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("loop_variable_declaration")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("loop variable must be declared in loop")
    }

    fn reason(&self) -> String {
        String::from("the scope of variable should be minimized")
    }
}
