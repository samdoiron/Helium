

struct Name(String);

enum Expr {
    Var(Name),
    App(Box<Expr>, Box<Expr>),
    Lam(Name, Box<Expr>)
};