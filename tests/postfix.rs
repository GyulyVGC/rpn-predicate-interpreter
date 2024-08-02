use rpn_predicate_interpreter::{Operator, PostfixExpression, PostfixToken, PredicateEvaluator};

struct Predicate {
    condition: PredicateCondition,
    val: String,
}

enum PredicateCondition {
    Equal,
    NotEqual,
    GreaterThan,
    LowerThan,
}

struct MyInteger {
    val: i32,
}

struct MyReal {
    val: f32,
}

impl PredicateEvaluator for MyInteger {
    type Predicate = Predicate;

    fn evaluate_predicate(&self, predicate: &Self::Predicate) -> bool {
        match predicate.condition {
            PredicateCondition::Equal => self.val == predicate.val.parse().unwrap(),
            PredicateCondition::NotEqual => self.val != predicate.val.parse().unwrap(),
            PredicateCondition::GreaterThan => self.val > predicate.val.parse().unwrap(),
            PredicateCondition::LowerThan => self.val < predicate.val.parse().unwrap(),
        }
    }
}

impl PredicateEvaluator for MyReal {
    type Predicate = Predicate;

    fn evaluate_predicate(&self, predicate: &Self::Predicate) -> bool {
        match predicate.condition {
            PredicateCondition::Equal => self.val == predicate.val.parse().unwrap(),
            PredicateCondition::NotEqual => self.val != predicate.val.parse().unwrap(),
            PredicateCondition::GreaterThan => self.val > predicate.val.parse().unwrap(),
            PredicateCondition::LowerThan => self.val < predicate.val.parse().unwrap(),
        }
    }
}

#[test]
// a --> a
fn test_postfix_evaluate_single() {
    let a = Predicate {
        condition: PredicateCondition::Equal,
        val: "33".to_string(),
    };

    let expr = PostfixExpression::from_tokens(vec![PostfixToken::Predicate(a)]).unwrap();

    assert!(!expr.evaluate(&MyInteger { val: 34 }));
    assert!(expr.evaluate(&MyInteger { val: 33 }));
    assert!(!expr.evaluate(&MyInteger { val: 12 }));

    assert!(!expr.evaluate(&MyReal { val: 34.0 }));
    assert!(expr.evaluate(&MyReal { val: 33.0 }));
    assert!(!expr.evaluate(&MyReal { val: 12.0 }));
}

#[test]
// a+b --> ab+
fn test_postfix_evaluate_simple() {
    let a = Predicate {
        condition: PredicateCondition::Equal,
        val: "33".to_string(),
    };
    let b = Predicate {
        condition: PredicateCondition::LowerThan,
        val: "10".to_string(),
    };

    let expr = PostfixExpression::from_tokens(vec![
        PostfixToken::Predicate(a),
        PostfixToken::Predicate(b),
        PostfixToken::Operator(Operator::Or),
    ])
    .unwrap();

    assert!(!expr.evaluate(&MyInteger { val: 34 }));
    assert!(expr.evaluate(&MyInteger { val: 33 }));
    assert!(!expr.evaluate(&MyInteger { val: 12 }));
    assert!(!expr.evaluate(&MyInteger { val: 11 }));
    assert!(!expr.evaluate(&MyInteger { val: 10 }));
    assert!(expr.evaluate(&MyInteger { val: 9 }));
    assert!(expr.evaluate(&MyInteger { val: 8 }));
    assert!(expr.evaluate(&MyInteger { val: 7 }));
    assert!(expr.evaluate(&MyInteger { val: 6 }));

    assert!(!expr.evaluate(&MyReal { val: 34.0 }));
    assert!(expr.evaluate(&MyReal { val: 33.0 }));
    assert!(!expr.evaluate(&MyReal { val: 12.0 }));
    assert!(!expr.evaluate(&MyReal { val: 11.0 }));
    assert!(!expr.evaluate(&MyReal { val: 10.0 }));
    assert!(expr.evaluate(&MyReal { val: 9.0 }));
    assert!(expr.evaluate(&MyReal { val: 8.0 }));
    assert!(expr.evaluate(&MyReal { val: 7.0 }));
    assert!(expr.evaluate(&MyReal { val: 6.0 }));
}

#[test]
// a+b*(c+d+e*(f+g)) --> abcd+efg+*+*+
fn test_postfix_evaluate_complex() {
    let a = Predicate {
        condition: PredicateCondition::Equal,
        val: "5".to_string(),
    };
    let b = Predicate {
        condition: PredicateCondition::Equal,
        val: "3".to_string(),
    };
    let c = Predicate {
        condition: PredicateCondition::NotEqual,
        val: "4".to_string(),
    };
    let d = Predicate {
        condition: PredicateCondition::GreaterThan,
        val: "6".to_string(),
    };
    let e = Predicate {
        condition: PredicateCondition::LowerThan,
        val: "9".to_string(),
    };
    let f = Predicate {
        condition: PredicateCondition::Equal,
        val: "7".to_string(),
    };
    let g = Predicate {
        condition: PredicateCondition::NotEqual,
        val: "8".to_string(),
    };

    let expr = PostfixExpression::from_tokens(vec![
        PostfixToken::Predicate(a),
        PostfixToken::Predicate(b),
        PostfixToken::Predicate(c),
        PostfixToken::Predicate(d),
        PostfixToken::Operator(Operator::Or),
        PostfixToken::Predicate(e),
        PostfixToken::Predicate(f),
        PostfixToken::Predicate(g),
        PostfixToken::Operator(Operator::Or),
        PostfixToken::Operator(Operator::And),
        PostfixToken::Operator(Operator::Or),
        PostfixToken::Operator(Operator::And),
        PostfixToken::Operator(Operator::Or),
    ])
    .unwrap();

    assert!(!expr.evaluate(&MyInteger { val: 7 }));
    assert!(!expr.evaluate(&MyInteger { val: 6 }));
    assert!(expr.evaluate(&MyInteger { val: 5 }));
    assert!(!expr.evaluate(&MyInteger { val: 4 }));
    assert!(expr.evaluate(&MyInteger { val: 3 }));

    assert!(!expr.evaluate(&MyReal { val: 7.0 }));
    assert!(!expr.evaluate(&MyReal { val: 6.0 }));
    assert!(expr.evaluate(&MyReal { val: 5.0 }));
    assert!(!expr.evaluate(&MyReal { val: 4.0 }));
    assert!(expr.evaluate(&MyReal { val: 3.0 }));
}

#[test]
// a*b*c*d --> ab*c*d*
fn test_postfix_evaluate_many_and() {
    let a = Predicate {
        condition: PredicateCondition::Equal,
        val: "1".to_string(),
    };
    let b = Predicate {
        condition: PredicateCondition::NotEqual,
        val: "2".to_string(),
    };
    let c = Predicate {
        condition: PredicateCondition::NotEqual,
        val: "3".to_string(),
    };
    let d = Predicate {
        condition: PredicateCondition::LowerThan,
        val: "4".to_string(),
    };

    let expr = PostfixExpression::from_tokens(vec![
        PostfixToken::Predicate(a),
        PostfixToken::Predicate(b),
        PostfixToken::Operator(Operator::And),
        PostfixToken::Predicate(c),
        PostfixToken::Operator(Operator::And),
        PostfixToken::Predicate(d),
        PostfixToken::Operator(Operator::And),
    ])
    .unwrap();

    assert!(!expr.evaluate(&MyInteger { val: 7 }));
    assert!(expr.evaluate(&MyInteger { val: 1 }));

    assert!(!expr.evaluate(&MyReal { val: 7.0 }));
    assert!(expr.evaluate(&MyReal { val: 1.0 }));
}

#[test]
// a+b+c+d --> ab+c+d+
fn test_postfix_evaluate_many_or() {
    let a = Predicate {
        condition: PredicateCondition::Equal,
        val: "1".to_string(),
    };
    let b = Predicate {
        condition: PredicateCondition::Equal,
        val: "2".to_string(),
    };
    let c = Predicate {
        condition: PredicateCondition::Equal,
        val: "3".to_string(),
    };
    let d = Predicate {
        condition: PredicateCondition::Equal,
        val: "4".to_string(),
    };

    let expr = PostfixExpression::from_tokens(vec![
        PostfixToken::Predicate(a),
        PostfixToken::Predicate(b),
        PostfixToken::Operator(Operator::Or),
        PostfixToken::Predicate(c),
        PostfixToken::Operator(Operator::Or),
        PostfixToken::Predicate(d),
        PostfixToken::Operator(Operator::Or),
    ])
    .unwrap();

    assert!(!expr.evaluate(&MyInteger { val: 0 }));
    assert!(expr.evaluate(&MyInteger { val: 1 }));
    assert!(expr.evaluate(&MyInteger { val: 2 }));
    assert!(expr.evaluate(&MyInteger { val: 3 }));
    assert!(expr.evaluate(&MyInteger { val: 4 }));
    assert!(!expr.evaluate(&MyInteger { val: 5 }));

    assert!(!expr.evaluate(&MyReal { val: 0.0 }));
    assert!(expr.evaluate(&MyReal { val: 1.0 }));
    assert!(expr.evaluate(&MyReal { val: 2.0 }));
    assert!(expr.evaluate(&MyReal { val: 3.0 }));
    assert!(expr.evaluate(&MyReal { val: 4.0 }));
    assert!(!expr.evaluate(&MyReal { val: 5.0 }));
}

#[test]
// a+b [invalid]
fn test_postfix_invalid_using_infix() {
    let postfix = PostfixExpression::from_tokens(vec![
        PostfixToken::Predicate("a"),
        PostfixToken::Operator(Operator::Or),
        PostfixToken::Predicate("b"),
    ]);
    assert!(postfix.is_none());
}

#[test]
// *ab [invalid]
fn test_postfix_invalid_using_prefix() {
    let postfix = PostfixExpression::from_tokens(vec![
        PostfixToken::Operator(Operator::And),
        PostfixToken::Predicate("a"),
        PostfixToken::Predicate("b"),
    ]);
    assert!(postfix.is_none());
}

#[test]
// ab [invalid]
fn test_postfix_invalid_only_predicates() {
    let postfix = PostfixExpression::from_tokens(vec![
        PostfixToken::Predicate("a"),
        PostfixToken::Predicate("b"),
    ]);
    assert!(postfix.is_none());
}

#[test]
// *+ [invalid]
fn test_postfix_invalid_only_operators() {
    let postfix = PostfixExpression::<u8>::from_tokens(vec![
        PostfixToken::Operator(Operator::And),
        PostfixToken::Operator(Operator::Or),
    ]);
    assert!(postfix.is_none());
}

#[test]
// * [invalid]
// + [invalid]
fn test_postfix_invalid_single_operator() {
    for op in vec![Operator::And, Operator::Or] {
        let postfix = PostfixExpression::<u8>::from_tokens(vec![PostfixToken::Operator(op)]);
        assert!(postfix.is_none());
    }
}

#[test]
// a*+ [invalid]
fn test_postfix_invalid_too_many_operators() {
    let postfix = PostfixExpression::from_tokens(vec![
        PostfixToken::Predicate("a"),
        PostfixToken::Operator(Operator::And),
        PostfixToken::Operator(Operator::Or),
    ]);
    assert!(postfix.is_none());
}

#[test]
// a* [invalid]
fn test_postfix_invalid_missing_a_predicate() {
    let postfix = PostfixExpression::from_tokens(vec![
        PostfixToken::Predicate("a"),
        PostfixToken::Operator(Operator::And),
    ]);
    assert!(postfix.is_none());
}

#[test]
// abc+ [invalid]
fn test_postfix_invalid_too_many_predicates() {
    let postfix = PostfixExpression::from_tokens(vec![
        PostfixToken::Predicate("a"),
        PostfixToken::Predicate("b"),
        PostfixToken::Predicate("c"),
        PostfixToken::Operator(Operator::Or),
    ]);
    assert!(postfix.is_none());
}

#[test]
// ab*c [invalid]
fn test_postfix_invalid_too_many_predicates_bis() {
    let postfix = PostfixExpression::from_tokens(vec![
        PostfixToken::Predicate("a"),
        PostfixToken::Predicate("b"),
        PostfixToken::Operator(Operator::And),
        PostfixToken::Predicate("c"),
    ]);
    assert!(postfix.is_none());
}
