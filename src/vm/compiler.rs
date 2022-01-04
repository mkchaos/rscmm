use crate::node::*;
use crate::token::Value;
use super::Program;
use super::program::V;

pub trait Compiler {
    fn compile(&self, prog: &mut Program) -> V;
}

impl Compiler for FactorNd {
    fn compile(&self, prog: &mut Program) -> V {
        match self {
            FactorNd::Var(n) => n.compile(prog),
            FactorNd::Value(v) => prog.push(v.clone()),
            FactorNd::Expr(n) => n.compile(prog),
        }
    }
}

impl Compiler for TermNd {
    fn compile(&self, prog: &mut Program) -> V {
        if self.b.is_some() {
            let (b, op) = self.b.as_ref().unwrap();
            self.a.compile(prog);
            b.compile(prog);
            prog.bin_op(op.clone())
        } else {
            self.a.compile(prog)
        }
    }
}

impl Compiler for ExprNd {
    fn compile(&self, prog: &mut Program) -> V {
        if self.b.is_some() {
            let (b, op) = self.b.as_ref().unwrap();
            self.a.compile(prog);
            b.compile(prog);
            prog.bin_op(op.clone())
        } else {
            self.a.compile(prog)
        }
    }
}

// No Push Here
impl Compiler for VarNd {
    fn compile(&self, prog: &mut Program) -> V {
        let v = prog.get_v_from_var(self);
        if self.declared() {
            prog.update_offset(self);
            V::NoWhere
        } else {
            prog.push_var(v)
        }
    }
}

impl Compiler for StmtNd {
    fn compile(&self, prog: &mut Program) -> V {
        let mut w = V::NoWhere;
        if self.expr.is_some() {
            w = self.expr.as_ref().unwrap().compile(prog);
        }
        if self.var.is_some() {
            let var = self.var.as_ref().unwrap();
            let v = prog.get_v_from_var(var);
            if self.expr.is_none() {
                if var.declared() {
                    // just push default
                    prog.push(Value::Int(0));
                    w = var.compile(prog);
                } else {
                    // print
                    return prog.print_var(var);
                }
            } else {
                if var.declared() {
                    // just update offset
                    w = var.compile(prog);
                } else {
                    // mov to v
                    w = prog.pop(v);
                }
            }
        }
        w
    }
}

impl Compiler for RootNd {
    fn compile(&self, prog: &mut Program) -> V {
        for st in self.stmts.iter() {
            st.compile(prog);
            prog.reset_stack_off();
        }
        V::NoWhere
    }
}
