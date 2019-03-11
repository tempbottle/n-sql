// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ast::AggregateType;
use ast::{Expression, PredicateExpression};

#[derive(Clone, Debug)]
pub struct CountFn {
    pub expr: Box<Expression>,
    pub aggregate_type: Option<AggregateType>,
}

impl CountFn {
    pub fn new(aggregate_type: Option<AggregateType>, expr: Box<Expression>) -> CountFn {
        CountFn {
            expr,
            aggregate_type,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CountIfFn {
    pub expr: Box<Expression>,
    pub predicate: Box<PredicateExpression>,
    pub aggregate_type: Option<AggregateType>,
}

impl CountIfFn {
    pub fn new(predicate: Box<PredicateExpression>, aggregate_type: Option<AggregateType>, expr: Box<Expression>) -> CountIfFn {
        CountIfFn {
            predicate,
            expr,
            aggregate_type,
        }
    }
}