#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Literal {
    id: usize,
    sign: bool,
}

impl Literal {
    fn new(id: usize, sign: bool) -> Literal {
        Literal { id, sign }
    }
    fn id(&self) -> usize {
        self.id
    }
    fn sign(&self) -> bool {
        self.sign
    }
    fn to_dimacs(&self) -> String {
        format!(
            "{}",
            if self.sign {
                self.id as i64 + 1
            } else {
                -(self.id as i64 + 1)
            }
        )
        .to_string()
    }
}

#[derive(Debug, Clone)]
struct Clause(Vec<Literal>);

impl Clause {
    fn new() -> Clause {
        Clause(vec![])
    }
    fn new_from_vec(xs: Vec<Literal>) -> Clause {
        Clause(xs)
    }
    fn push(&mut self, x: Literal) {
        self.0.push(x);
    }
    fn len(&self) -> usize {
        self.0.len()
    }
    fn get_index(&self, id: usize) -> Option<usize> {
        for (i, literal) in self.iter().enumerate() {
            if literal.id() == id {
                return Some(i);
            }
        }
        None
    }
    fn to_dimacs(&self) -> String {
        let mut res = String::new();
        res.push_str(&self.0[0].to_dimacs().to_string());
        for i in 1..self.0.len() {
            res.push(' ');
            res.push_str(&self.0[i].to_dimacs().to_string());
        }
        res
    }
    fn resolution(left: &Clause, right: &Clause) -> Option<Clause> {
        let mut left_valids = vec![true; left.0.len()];
        let mut right_valids = vec![true; right.0.len()];
        let mut succeeded = false;
        for i in 0..left.0.len() {
            for k in 0..right.len() {
                if left[i].id() == right[k].id() && left[i].sign() != right[k].sign() {
                    left_valids[i] = false;
                    right_valids[k] = false;
                    succeeded = true;
                } else if left[i].id() == right[k].id() {
                    right_valids[k] = false;
                }
            }
        }
        if succeeded {
            let mut res = Clause::new();
            for i in 0..left.0.len() {
                if left_valids[i] {
                    res.push(left[i]);
                }
            }
            for k in 0..right.0.len() {
                if right_valids[k] {
                    res.push(right[k]);
                }
            }
            Some(res)
        } else {
            None
        }
    }
}

use std::convert::AsRef;
use std::iter::IntoIterator;
use std::ops::Deref;

impl IntoIterator for Clause {
    type Item = Literal;
    type IntoIter = std::vec::IntoIter<Literal>;
    fn into_iter(self) -> std::vec::IntoIter<Literal> {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Clause {
    type Item = &'a Literal;
    type IntoIter = std::slice::Iter<'a, Literal>;
    fn into_iter(self) -> std::slice::Iter<'a, Literal> {
        self.0.iter()
    }
}

impl AsRef<Clause> for Clause {
    fn as_ref(&self) -> &Clause {
        self
    }
}

impl Deref for Clause {
    type Target = [Literal];
    fn deref(&self) -> &[Literal] {
        self.0.as_slice()
    }
}

#[derive(Debug, Clone)]
struct Clauses(Vec<Clause>);

impl Clauses {
    fn new() -> Clauses {
        Clauses(vec![])
    }
    fn new_from_vec(xs: Vec<Clause>) -> Clauses {
        Clauses(xs)
    }
    fn push(&mut self, clause: Clause) {
        self.0.push(clause);
    }
    fn num(&self) -> usize {
        self.0.len()
    }
    fn iter(&self) -> std::slice::Iter<Clause> {
        self.0.iter()
    }
}

impl IntoIterator for Clauses {
    type Item = Clause;
    type IntoIter = std::vec::IntoIter<Clause>;
    fn into_iter(self) -> std::vec::IntoIter<Clause> {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Clauses {
    type Item = &'a Clause;
    type IntoIter = std::slice::Iter<'a, Clause>;
    fn into_iter(self) -> std::slice::Iter<'a, Clause> {
        self.0.iter()
    }
}

impl AsRef<Clauses> for Clauses {
    fn as_ref(&self) -> &Clauses {
        self
    }
}

impl<I: SliceIndex<[Clause]>> Index<I> for Clauses {
    type Output = <I as SliceIndex<[Clause]>>::Output;
    fn index(&self, index: I) -> &Self::Output {
        &self.0[index]
    }
}

impl<I: SliceIndex<[Clause]>> IndexMut<I> for Clauses {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[derive(Debug)]
pub struct SatProblem {
    n_variables: usize,
    clauses: Clauses,
}

impl SatProblem {
    pub fn new_from_dimacs(s: &str) -> SatProblem {
        let s2 = {
            let mut res = String::new();
            for line in s.lines() {
                let t = line.to_string();
                if t.chars().take(1).next() == Some('c') {
                    continue;
                }
                res.push_str(line);
                res.push('\n');
            }
            res
        };
        let mut iter = s2.split_whitespace();
        assert_eq!(iter.next(), Some("p"));
        assert_eq!(iter.next(), Some("cnf"));
        let n_variables = iter.next().unwrap().parse::<usize>().unwrap();
        let n_clauses = iter.next().unwrap().parse::<usize>().unwrap();
        let mut clauses = Clauses::new();
        let mut xs = vec![];
        for t in iter {
            let u = t.parse::<i64>().unwrap();
            if u == 0 {
                clauses.push(Clause(xs.clone()));
                xs.clear();
            } else if u > 0 {
                xs.push(Literal::new(u as usize - 1, true));
            } else if u < 0 {
                xs.push(Literal::new(-u as usize - 1, false));
            } else {
                unreachable!();
            }
        }
        assert_eq!(clauses.num(), n_clauses);
        SatProblem {
            n_variables,
            clauses,
        }
    }
    pub fn to_dimacs(&self) -> String {
        let mut res = String::new();
        res.push_str(&format!(
            "p cnf {} {}\n",
            self.n_variables,
            self.clauses.num()
        ));
        for clause in &self.clauses {
            for &literal in clause.iter() {
                let t = literal.id() + 1;
                let u = t as i64 * if literal.sign() { 1 } else { -1 };
                res.push_str(&format!("{} ", u));
            }
            res.push_str("0\n");
        }
        res
    }
    pub fn gen_random_sat(
        n_variables: usize,
        n_clauses: usize,
        k_sat: usize,
        prob_true: f64,
    ) -> SatProblem {
        use rand::distributions::Uniform;
        use rand::prelude::*;
        let mut assignments = vec![false; n_variables];
        let mut rng = rand::thread_rng();
        for assignment in assignments.iter_mut() {
            *assignment = rng.gen::<bool>();
        }
        let mut clauses = Clauses::new();
        for _ in 0..n_clauses {
            let mut clause = Clause::new();
            let dist = Uniform::from(0..n_variables);
            'l1: while clause.len() < k_sat {
                let id = dist.sample(&mut rng);
                for &x in &clause {
                    if x.id() == id {
                        continue 'l1;
                    }
                }
                let sign = if clause.len() == 0 || rng.gen::<f64>() < prob_true {
                    assignments[id]
                } else {
                    !assignments[id]
                };
                clause.push(Literal::new(id, sign));
            }
            clauses.push(clause);
        }
        SatProblem {
            n_variables,
            clauses,
        }
    }
    fn check_assingemnt(&self, assignment: &SatAssignments) -> bool {
        for clause in &self.clauses {
            let mut tf = false;
            for &x in &clause.0 {
                if assignment[x.id()] == x.sign() {
                    tf = true;
                    break;
                }
            }
            if !tf {
                return false;
            }
        }
        true
    }
}

struct TaggedClause {
    clause: Clause,
    learnt: bool,
    watched: [Literal; 2],
}

impl TaggedClause {
    fn new(clause: Clause, learnt: bool, watched: [Literal; 2]) -> TaggedClause {
        TaggedClause {
            clause,
            learnt,
            watched,
        }
    }
    fn clause(&self) -> &Clause {
        &self.clause
    }
    fn learnt(&self) -> bool {
        self.learnt
    }
    fn watched(&self) -> &[Literal; 2] {
        &self.watched
    }
    fn watched_mut(&mut self) -> &mut [Literal; 2] {
        &mut self.watched
    }
    fn to_dimacs(&self) -> String {
        self.clause.to_dimacs()
    }
}

#[derive(Debug)]
enum AssignmentState {
    First,
    Second,
    Propageted(usize),
}

#[derive(Debug, Clone, Copy)]
enum VariableState {
    NotAssigned,
    Assigned { sign: bool, decision_level: usize },
}

impl VariableState {
    fn new() -> VariableState {
        VariableState::NotAssigned
    }
    fn is_not_assigned(&self) -> bool {
        match self {
            VariableState::NotAssigned => true,
            _ => false,
        }
    }
    fn sign(&self) -> Option<bool> {
        match self {
            VariableState::NotAssigned => None,
            VariableState::Assigned { sign, .. } => Some(*sign),
        }
    }
    fn decision_level(&self) -> Option<usize> {
        match self {
            VariableState::NotAssigned => None,
            VariableState::Assigned { decision_level, .. } => Some(*decision_level),
        }
    }
}

pub struct SatSolver<'a> {
    problem: &'a SatProblem,
    clauses: Vec<TaggedClause>,
    variables: Vec<VariableState>,
    watch: Vec<Vec<usize>>,
    dpll_stack: Vec<(usize, AssignmentState)>,
    decision_level: usize,
    conflict_count: usize,
}

impl<'a> SatSolver<'a> {
    pub fn new(problem: &'a SatProblem) -> SatSolver {
        let clauses: Vec<TaggedClause> = problem
            .clauses
            .iter()
            .map(|x| TaggedClause::new(x.clone(), false, [x[0], x[0]]))
            .collect();
        SatSolver {
            problem,
            clauses,
            variables: vec![VariableState::new(); problem.n_variables],
            watch: vec![vec![]; problem.n_variables],
            dpll_stack: vec![],
            decision_level: 0,
            conflict_count: 0,
        }
    }
    fn first_signs(&self) -> Vec<bool> {
        let mut count = vec![0; self.problem.n_variables];
        let mut total = vec![0; self.problem.n_variables];
        for clause in &self.clauses {
            for literal in clause.clause().iter() {
                if literal.sign() {
                    count[literal.id()] += 1;
                }
                total[literal.id()] += 1;
            }
        }
        let mut res = vec![];
        for i in 0..self.problem.n_variables {
            if count[i] > total[i] / 2 {
                res.push(false);
            } else {
                res.push(true);
            }
        }
        res
    }
    fn learn_clause(&mut self, clause: &Clause) {
        let mut assigned_literals = vec![];
        let mut not_assigned_literals = vec![];
        for &literal in clause.iter() {
            match self.variables[literal.id()] {
                VariableState::NotAssigned => {
                    not_assigned_literals.push(literal);
                }
                VariableState::Assigned { decision_level, .. } => {
                    assigned_literals.push((literal, decision_level));
                }
            }
        }
        let clause_id = self.clauses.len();
        let (literal_1, literal_2) = if not_assigned_literals.len() >= 2 {
            let literal_1 = not_assigned_literals[0];
            let literal_2 = not_assigned_literals[1];
            (literal_1, literal_2)
        } else if not_assigned_literals.len() == 1 {
            if clause.len() >= 2 {
                let literal_1 = not_assigned_literals[0];
                assert!(!assigned_literals.is_empty());
                let literal_2 = assigned_literals
                    .iter()
                    .max_by(|&x, &y| x.1.cmp(&y.1))
                    .unwrap()
                    .0;
                (literal_1, literal_2)
            } else {
                (clause[0], clause[0])
            }
        } else {
            panic!();
        };
        self.clauses.push(TaggedClause::new(
            clause.clone(),
            true,
            [literal_1, literal_2],
        ));
        self.watch[literal_1.id()].push(clause_id);
        self.watch[literal_2.id()].push(clause_id);
    }
    pub fn assign_unit_clause(&mut self) -> bool {
        loop {
            let mut updated = false;
            'l1: for tagged_clause in &self.clauses {
                let mut unknowns = vec![];
                for literal in tagged_clause.clause() {
                    match self.variables[literal.id()].sign() {
                        Some(sign) => {
                            if sign == literal.sign() {
                                continue 'l1;
                            }
                        }
                        None => {
                            unknowns.push(literal);
                        }
                    }
                }
                if unknowns.is_empty() {
                    return false;
                }
                if unknowns.len() == 1 {
                    let literal = unknowns[0];
                    self.variables[literal.id()] = VariableState::Assigned {
                        sign: literal.sign(),
                        decision_level: self.decision_level,
                    };
                    updated = true;
                }
            }
            if !updated {
                break;
            }
        }
        true
    }
    fn try_next_assignment(&mut self, i: usize) -> bool {
        for k in i..self.problem.n_variables {
            if self.variables[k].is_not_assigned() {
                self.dpll_stack.push((k, AssignmentState::First));
                self.decision_level += 1;
                return true;
            }
        }
        false
    }
    fn try_backtrack(&mut self, clause_id: usize) -> bool {
        self.conflict_count += 1;
        // conflict
        let mut clause = self.clauses[clause_id].clause().clone();
        while let Some((k, state)) = self.dpll_stack.pop() {
            match state {
                AssignmentState::First => {
                    self.dpll_stack.push((k, AssignmentState::Second));
                    return true;
                }
                AssignmentState::Second => {
                    self.variables[k] = VariableState::NotAssigned;
                    self.decision_level -= 1;
                }
                AssignmentState::Propageted(clause_id) => {
                    self.variables[k] = VariableState::NotAssigned;
                    let t = Clause::resolution(&clause, &self.clauses[clause_id].clause());
                    if let Some(new_clause) = t {
                        clause = new_clause;
                        let num_current_decision_level = {
                            let mut num_current_decision_level = 0;
                            for i in 0..clause.len() {
                                match self.variables[clause[i].id()].decision_level() {
                                    Some(level) => {
                                        if level == self.decision_level {
                                            num_current_decision_level += 1;
                                        }
                                    }
                                    None => {
                                        num_current_decision_level += 1;
                                    }
                                }
                            }
                            num_current_decision_level
                        };
                        if num_current_decision_level == 1 {
                            let second_decision_level = {
                                let mut second_decision_level = 0;
                                for i in 0..clause.len() {
                                    if let Some(level) =
                                        self.variables[clause[i].id()].decision_level()
                                    {
                                        if level != self.decision_level
                                            && level > second_decision_level
                                        {
                                            second_decision_level = level;
                                        }
                                    }
                                }
                                second_decision_level
                            };
                            info!(
                                "backtrack, clause = {:?}, first_level = {}, second_level = {}",
                                clause.to_dimacs(),
                                self.decision_level,
                                second_decision_level
                            );
                            assert!(self.decision_level > second_decision_level);
                            while let Some((k, state)) = self.dpll_stack.pop() {
                                match state {
                                    AssignmentState::First => {
                                        if self.decision_level <= second_decision_level {
                                            self.dpll_stack.push((k, AssignmentState::First));
                                            self.learn_clause(&clause);
                                            return true;
                                        } else {
                                            self.variables[k] = VariableState::NotAssigned;
                                            self.decision_level -= 1;
                                        }
                                    }
                                    AssignmentState::Second => {
                                        if self.decision_level <= second_decision_level {
                                            self.dpll_stack.push((k, AssignmentState::Second));
                                            self.learn_clause(&clause);
                                            return true;
                                        } else {
                                            self.variables[k] = VariableState::NotAssigned;
                                            self.decision_level -= 1;
                                        }
                                    }
                                    AssignmentState::Propageted(_) => {
                                        self.variables[k] = VariableState::NotAssigned;
                                    }
                                }
                            }
                            assert_eq!(second_decision_level, 0);
                            self.learn_clause(&clause);
                            self.assign_unit_clause();
                            let t = self.try_next_assignment(0);
                            assert!(t);
                            return true;
                        }
                    }
                }
            }
        }
        // UNSAT
        false
    }
    fn init_watch(&mut self) {
        for (clause_id, tagged_clause) in self.clauses.iter_mut().enumerate() {
            let clause = tagged_clause.clause();
            if clause.len() >= 2 {
                let mut xs = vec![false; clause.len()];
                for (i, literal) in clause.iter().enumerate().take(2) {
                    xs[i] = true;
                    self.watch[literal.id()].push(clause_id);
                }
                *tagged_clause.watched_mut() = [clause[0], clause[1]];
            } else {
                *tagged_clause.watched_mut() = [clause[0], clause[0]];
            }
        }
    }
    pub fn solve(&mut self) -> Option<SatAssignments> {
        let success = self.assign_unit_clause();
        if !success {
            // UNSAT
            return None;
        }

        self.init_watch();
        let first_signs = self.first_signs();

        if !self.try_next_assignment(0) {
            // end(SAT)
            let xs: Vec<bool> = self.variables.iter().map(|&x| x.sign().unwrap()).collect();
            let res = SatAssignments::new_from_vec(xs);
            assert!(self.problem.check_assingemnt(&res));
            return Some(res);
        }
        let mut loop_count = 0;
        'l1: loop {
            if loop_count % 100 == 0 {
                warn!(
                    "clauses.len() = {}, conflict_count = {}",
                    self.clauses.len(),
                    self.conflict_count
                );
            }
            loop_count += 1;
            // try
            assert!(!self.dpll_stack.is_empty());
            info!("dpll_stack_top: {:?}", self.dpll_stack.last());
            let i = self.dpll_stack.last().unwrap().0;
            match self.dpll_stack.last().unwrap().1 {
                AssignmentState::First => {
                    self.variables[i] = VariableState::Assigned {
                        sign: first_signs[i],
                        decision_level: self.decision_level,
                    };
                }
                AssignmentState::Second => {
                    let old_sign = self.variables[i].sign().unwrap();
                    self.variables[i] = VariableState::Assigned {
                        sign: !old_sign,
                        decision_level: self.decision_level,
                    };
                }
                AssignmentState::Propageted(_) => {
                    panic!();
                }
            }

            // unit propagation
            use std::collections::VecDeque;
            let mut unit_propagation_stack = VecDeque::new();
            unit_propagation_stack.push_back(i);
            use std::collections::HashSet;
            let mut visited = HashSet::new();
            while let Some(id) = unit_propagation_stack.pop_back() {
                if visited.contains(&id) {
                    continue;
                }
                visited.insert(id);
                info!("propagated: {}", id);

                let visit_clause_ids: Vec<usize> = self.watch[id].clone();
                for &clause_id in &visit_clause_ids {
                    let tagged_clause = &self.clauses[clause_id];
                    let clause = tagged_clause.clause();
                    let watched = tagged_clause.watched();
                    assert!(clause.len() != 1);
                    let prev_i_literal = clause.get_index(id);
                    assert!(prev_i_literal.is_some());
                    let prev_i_literal = prev_i_literal.unwrap();
                    let prev_i_literal_i = if watched[0].id() == id {
                        0
                    } else if watched[1].id() == id {
                        1
                    } else {
                        continue;
                    };
                    if self.clauses[clause_id].clause()[prev_i_literal].sign()
                        == self.variables[id].sign().unwrap()
                    {
                        continue;
                    }
                    let mut next_literal = None;
                    for literal in clause.iter() {
                        assert!(watched[0].id() == id || watched[1].id() == id);
                        if literal.id() != id
                            && self.variables[literal.id()].sign() != Some(!literal.sign())
                            && (watched[0].id() != id || watched[1].id() != literal.id())
                            && (watched[1].id() != id || watched[0].id() != literal.id())
                        {
                            next_literal = Some(literal);
                        }
                    }
                    if let Some(next_literal) = next_literal {
                        let next_literal_id = next_literal.id();
                        assert!(id != next_literal_id);
                        assert!(watched[prev_i_literal_i].id() == id);
                        assert!(watched[prev_i_literal_i].id() != next_literal_id);
                        self.clauses[clause_id].watched[prev_i_literal_i] = *next_literal;
                        self.watch[id] = self.watch[id]
                            .iter()
                            .filter(|&&x| x != clause_id)
                            .cloned()
                            .collect();
                        self.watch[next_literal_id].push(clause_id);
                    } else {
                        let literal2 = watched[1 - prev_i_literal_i];
                        let id2 = literal2.id();
                        if self.variables[id2].is_not_assigned() {
                            self.variables[id2] = VariableState::Assigned {
                                sign: literal2.sign(),
                                decision_level: self.decision_level,
                            };
                            self.dpll_stack
                                .push((id2, AssignmentState::Propageted(clause_id)));
                            unit_propagation_stack.push_back(id2);
                        } else if self.variables[id2].sign().unwrap() != literal2.sign() {
                            // conflict
                            let succeeded = self.try_backtrack(clause_id);
                            if succeeded {
                                continue 'l1;
                            } else {
                                // UNSAT
                                return None;
                            }
                        }
                    }
                }
            }

            if !self.try_next_assignment(i) {
                // SAT
                let xs: Vec<bool> = self.variables.iter().map(|&x| x.sign().unwrap()).collect();
                let res = SatAssignments::new_from_vec(xs);
                assert!(self.problem.check_assingemnt(&res));
                return Some(res);
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SatAssignments(Vec<bool>);

impl SatAssignments {
    fn new_from_vec(xs: Vec<bool>) -> SatAssignments {
        SatAssignments(xs)
    }
    pub fn to_dimacs(&self) -> String {
        let mut res = String::new();
        for i in 0..self.0.len() {
            let x = self.0[i];
            if !x {
                res.push_str(&format!("-{} ", i + 1));
            } else {
                res.push_str(&format!("{} ", i + 1));
            }
        }
        res.push_str("0");
        res
    }
}

use std::ops::Index;
use std::ops::IndexMut;
use std::slice::SliceIndex;

impl<I: SliceIndex<[bool]>> Index<I> for SatAssignments {
    type Output = <I as SliceIndex<[bool]>>::Output;
    fn index(&self, index: I) -> &Self::Output {
        &self.0[index]
    }
}

impl<I: SliceIndex<[bool]>> IndexMut<I> for SatAssignments {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[test]
fn test_solve_sat_1() {
    let problem = SatProblem {
        n_variables: 1,
        clauses: Clauses::new_from_vec(vec![Clause::new_from_vec(vec![Literal::new(0, true)])]),
    };
    let mut solver = SatSolver::new(&problem);
    let res = solver.solve().unwrap();
    assert!(problem.check_assingemnt(&res));
}

#[test]
fn test_solve_sat_2() {
    let problem = SatProblem {
        n_variables: 1,
        clauses: Clauses::new_from_vec(vec![Clause::new_from_vec(vec![Literal::new(0, false)])]),
    };
    let mut solver = SatSolver::new(&problem);
    let res = solver.solve().unwrap();
    assert!(problem.check_assingemnt(&res));
}

#[test]
fn test_solve_sat_3() {
    let problem = SatProblem {
        n_variables: 2,
        clauses: Clauses::new_from_vec(vec![Clause::new_from_vec(vec![
            Literal::new(0, true),
            Literal::new(1, false),
        ])]),
    };
    let mut solver = SatSolver::new(&problem);
    let res = solver.solve().unwrap();
    assert!(problem.check_assingemnt(&res));
}

#[test]
fn test_solve_sat_4() {
    let problem = SatProblem {
        n_variables: 2,
        clauses: Clauses::new_from_vec(vec![Clause::new_from_vec(vec![
            Literal::new(0, false),
            Literal::new(1, true),
        ])]),
    };
    let mut solver = SatSolver::new(&problem);
    let res = solver.solve().unwrap();
    assert!(problem.check_assingemnt(&res));
}

#[test]
fn test_solve_sat_5() {
    let problem = SatProblem {
        n_variables: 2,
        clauses: Clauses::new_from_vec(vec![Clause::new_from_vec(vec![
            Literal::new(0, false),
            Literal::new(1, true),
        ])]),
    };
    let mut solver = SatSolver::new(&problem);
    let res = solver.solve().unwrap();
    assert!(problem.check_assingemnt(&res));
}

#[test]
fn test_solve_sat_6() {
    let problem = SatProblem {
        n_variables: 3,
        clauses: Clauses::new_from_vec(vec![Clause::new_from_vec(vec![
            Literal::new(0, false),
            Literal::new(1, true),
            Literal::new(2, false),
        ])]),
    };
    let mut solver = SatSolver::new(&problem);
    let res = solver.solve().unwrap();
    assert!(problem.check_assingemnt(&res));
}

#[test]
fn test_solve_sat_7() {
    let problem = SatProblem {
        n_variables: 1,
        clauses: Clauses::new_from_vec(vec![
            Clause::new_from_vec(vec![Literal::new(0, true)]),
            Clause::new_from_vec(vec![Literal::new(0, false)]),
        ]),
    };
    let mut solver = SatSolver::new(&problem);
    let res = solver.solve();
    assert!(res.is_none());
}

#[test]
fn test_solve_sat_8() {
    let problem = SatProblem {
        n_variables: 3,
        clauses: Clauses::new_from_vec(vec![
            Clause::new_from_vec(vec![
                Literal::new(0, true),
                Literal::new(1, true),
                Literal::new(2, false),
            ]),
            Clause::new_from_vec(vec![
                Literal::new(0, true),
                Literal::new(1, false),
                Literal::new(2, true),
            ]),
            Clause::new_from_vec(vec![
                Literal::new(0, false),
                Literal::new(1, true),
                Literal::new(2, true),
            ]),
            Clause::new_from_vec(vec![
                Literal::new(0, false),
                Literal::new(1, false),
                Literal::new(2, false),
            ]),
            Clause::new_from_vec(vec![Literal::new(2, true)]),
        ]),
    };
    let mut solver = SatSolver::new(&problem);
    let res = solver.solve().unwrap();
    assert!(problem.check_assingemnt(&res));
}

#[test]
#[ignore]
fn test_solve_sat_9() {
    for _ in 0..1000 {
        let problem = SatProblem::gen_random_sat(10000, 10000, 4, 0.2);
        // eprintln!("problem\n{}\n", problem.to_dimacs());
        let mut solver = SatSolver::new(&problem);
        let res = solver.solve().unwrap();
        assert!(problem.check_assingemnt(&res));
    }
}
