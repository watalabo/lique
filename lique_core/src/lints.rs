pub mod measurement_twice;
pub mod op_after_measurement;

use oq3_syntax::ast::{Expr, GateOperand, Identifier, IndexKind, IndexedIdentifier, LiteralKind};

pub(crate) fn contains_or_equal(operand: &GateOperand, other_operand: &GateOperand) -> bool {
    fn contains(identifier: &Identifier, indexed_identifier: &IndexedIdentifier) -> bool {
        identifier.ident_token().unwrap().text() == indexed_identifier.identifier().unwrap().ident_token().unwrap().text()
    }

    match (operand, other_operand) {
        (GateOperand::IndexedIdentifier(q1), GateOperand::IndexedIdentifier(q2)) => {
            if let Some(q1_index) = q1.index_operators().next().unwrap().index_kind()
                && let Some(q2_index) = q2.index_operators().next().unwrap().index_kind()
                && let IndexKind::ExpressionList(q1_index) = q1_index
                && let IndexKind::ExpressionList(q2_index) = q2_index
            {
                q1_index.exprs().zip(q2_index.exprs()).all(|(e1, e2)| {
                    if let Expr::Literal(index1) = e1
                        && let Expr::Literal(index2) = e2
                        && let LiteralKind::IntNumber(index1) = index1.kind()
                        && let LiteralKind::IntNumber(index2) = index2.kind()
                    {
                        index1.value().unwrap() == index2.value().unwrap()
                    } else {
                        false
                    }
                }) && q1.identifier().unwrap().ident_token().unwrap().text()
                    == q2.identifier().unwrap().ident_token().unwrap().text()
            } else {
                false
            }
        }
        (GateOperand::Identifier(q1), GateOperand::IndexedIdentifier(q2)) => contains(q1, q2),
        (GateOperand::IndexedIdentifier(q1), GateOperand::Identifier(q2)) => contains(q2, q1),
        _ => false,
    }
}
