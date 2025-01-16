pub mod conditional_without_measurement;
pub mod double_measurement;
pub mod op_after_measurement;
pub mod oversized_circuit;
pub mod unmanipulated_qubits;
pub mod unmeasurable_qubits;

use core::ops::Range;
use std::collections::HashMap;

use oq3_syntax::{
    ast::{
        AstChildren, Expr, GateOperand, HasName, Identifier, IndexKind, IndexedIdentifier,
        LiteralKind, Stmt,
    },
    AstNode,
};

pub(crate) fn contains_or_equal(operand: &GateOperand, other_operand: &GateOperand) -> bool {
    fn contains(identifier: &Identifier, indexed_identifier: &IndexedIdentifier) -> bool {
        identifier.ident_token().unwrap().text()
            == indexed_identifier
                .identifier()
                .unwrap()
                .ident_token()
                .unwrap()
                .text()
    }

    match (operand, other_operand) {
        (GateOperand::Identifier(q1), GateOperand::IndexedIdentifier(q2)) => contains(q1, q2),
        (GateOperand::IndexedIdentifier(q1), GateOperand::Identifier(q2)) => contains(q2, q1),
        (GateOperand::Identifier(q1), GateOperand::Identifier(q2)) => {
            q1.ident_token().unwrap().text() == q2.ident_token().unwrap().text()
        }
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
        _ => false,
    }
}

/// Returns the number of qubits declared in the given statements and the byte range of the declaration in the QASM file.
pub(crate) fn count_qubits(stmts: AstChildren<Stmt>) -> (usize, Range<usize>) {
    let mut num_qubits = 0;
    let mut qubits_range: Range<usize> = 0..0;
    for stmt in stmts.clone() {
        if let Stmt::QuantumDeclarationStatement(declaration) = stmt.clone()
            && let Some(qubit) = declaration.qubit_type()
            && let Some(designator) = qubit.designator()
            && let Some(expr) = designator.expr()
            && let Expr::Literal(bits) = expr
        {
            num_qubits += bits.to_string().parse::<usize>().unwrap();
            qubits_range = stmt.syntax().text_range().into();
        }
    }
    (num_qubits, qubits_range)
}

/// Returns a mapping from classical register name to the number of classical bits declared in the given statements
/// and the byte range of the declaration in the QASM file.
pub(crate) fn count_clbits(stmts: AstChildren<Stmt>) -> HashMap<String, (usize, Range<usize>)> {
    let mut clbits = HashMap::new();
    for stmt in stmts.clone() {
        if let Stmt::ClassicalDeclarationStatement(declaration) = stmt.clone()
            && let Some(qubit) = declaration.scalar_type()
            && let Some(bits_name) = declaration.name()
            && let Some(designator) = qubit.designator()
            && let Some(expr) = designator.expr()
            && let Expr::Literal(bits) = expr
        {
            let num_classical_bits = bits.to_string().parse::<usize>().unwrap();
            let classical_bits_range: Range<usize> = stmt.syntax().text_range().into();
            clbits.insert(
                bits_name.to_string(),
                (num_classical_bits, classical_bits_range),
            );
        }
    }
    clbits
}

/// Returns a mask where each bit represents a qubit that is manipulated by the operand.
/// The mask is little-endian, i.e. the least significant bit represents the first qubit.
pub(crate) fn manipulated_qubits(operand: &GateOperand, num_qubits: usize) -> usize {
    let mut manipulated_mask = 0;
    match operand.clone() {
        GateOperand::IndexedIdentifier(indexed_identifier) => {
            for operator in indexed_identifier.index_operators() {
                if let Some(kind) = operator.index_kind()
                    && let IndexKind::ExpressionList(list) = kind
                {
                    for expr in list.exprs() {
                        if let Expr::Literal(literal) = expr {
                            let qubit_index =
                                literal.syntax().to_string().parse::<usize>().unwrap();
                            manipulated_mask |= 1 << qubit_index;
                        }
                    }
                }
            }
        }
        GateOperand::Identifier(_) => {
            manipulated_mask = (1 << num_qubits) - 1;
        }
        _ => {}
    }
    manipulated_mask
}
