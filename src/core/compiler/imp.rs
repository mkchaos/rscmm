use super::{Compiler, Context};
use crate::core::types::nodes::*;
use crate::core::types::{only_pop_code, CalcItem, Code, MemAddr, Type, Value};

impl Compiler for FactorNd {
    fn compile(&self, cxt: &mut Context) {
        match self {
            FactorNd::Var(n) => n.compile(cxt),
            FactorNd::Value(v) => {
                if let Value::Int(num) = v {
                    cxt.add_code(Code::Push(MemAddr::Value(*num)));
                }
            }
            FactorNd::Func(n) => n.compile(cxt),
        }
    }
}

impl Compiler for ExprNd {
    fn compile(&self, cxt: &mut Context) {
        for it in self.stack.iter() {
            match it {
                CalcItem::Op(op) => {
                    cxt.add_code(Code::Op(op.clone()));
                }
                CalcItem::Factor(f) => {
                    f.compile(cxt);
                }
            }
        }
    }
}

impl Compiler for VarNd {
    fn compile(&self, cxt: &mut Context) {
        cxt.push(self.get_id());
    }
}

impl Compiler for AssignNd {
    fn compile(&self, cxt: &mut Context) {
        self.expr.compile(cxt);
        cxt.pop(self.var.get_id());
    }
}

impl Compiler for DeclareNd {
    fn compile(&self, cxt: &mut Context) {
        match &self.expr {
            Some(e) => {
                e.compile(cxt);
            }
            None => {
                cxt.add_code(Code::Push(MemAddr::Value(0)));
            }
        };
    }
}

impl Compiler for StmtNd {
    fn compile(&self, cxt: &mut Context) {
        match self {
            StmtNd::Assign(n) => {
                n.compile(cxt);
            }
            StmtNd::Declare(n) => n.compile(cxt),
            StmtNd::Expr(n) => {
                n.compile(cxt);
                cxt.add_code(only_pop_code());
            }
            StmtNd::Print(n) => {
                cxt.push(n.get_id());
                cxt.add_code(Code::Print);
                cxt.add_code(only_pop_code());
            }
            _ => {}
        }
    }
}

impl Compiler for ItemNd {
    fn compile(&self, cxt: &mut Context) {
        match self {
            ItemNd::Block(n) => n.compile(cxt),
            ItemNd::Stmt(n) => n.compile(cxt),
            _ => {}
        }
    }
}

impl Compiler for BlockNd {
    fn compile(&self, cxt: &mut Context) {
        for it in self.items.iter() {
            it.compile(cxt);
        }
    }
}

impl Compiler for FuncNd {
    fn compile(&self, cxt: &mut Context) {
        if self.is_impl() {
            let id = self.var.get_id();
            cxt.enter_func(id);
            self.block.as_ref().unwrap().compile(cxt);
            cxt.exit_func();
        }
    }
}

impl Compiler for FuncCallNd {
    fn compile(&self, cxt: &mut Context) {
        for p in self.params.iter() {
            p.compile(cxt);
        }
        let id = self.var.get_id();
        cxt.call(id);
    }
}

impl Compiler for GItemNd {
    fn compile(&self, cxt: &mut Context) {
        match self {
            GItemNd::Func(n) => n.compile(cxt),
            GItemNd::Declare(n) => {
                // add memory
                if n.ty == Type::Int {
                    let v = n.try_retrieve_const().unwrap();
                    cxt.add_memory(v);
                }
            }
        }
    }
}

impl Compiler for RootNd {
    fn compile(&self, cxt: &mut Context) {
        for it in self.items.iter() {
            it.compile(cxt);
        }
    }
}