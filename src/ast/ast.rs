use std::{
    any::{Any, TypeId},
    collections::HashMap,
    marker::PhantomData,
};

enum Expression {
    // Statement-like expressions
    FunctionDeclaration(String, Vec<String>, Vec<Expression>),
    VariableDeclaration(String, Option<Type>, Box<Expression>),
    Assignment(String, Box<Expression>),
    Return(Box<Expression>),
    // Control flow
    If(Box<Expression>, Vec<Expression>),
    While(Box<Expression>, Vec<Expression>),

    // Subexpressions
    Literal(Literal),
    Ident(String),
    Binary(Box<Expression>, BinaryOperator, Box<Expression>),
}

enum Literal {
    Int(i64),
    Float(f64),
}

enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

enum Type {
    Int,
    Float,
}

struct Node<T> {
    node: T,
    // This is kinda cursed, but only accessed from typesafe functions
    annotations: HashMap<TypeId, Box<dyn Any>>,
}

impl From<Expression> for Node<Expression> {
    fn from(node: Expression) -> Self {
        Node {
            node,
            annotations: HashMap::new(),
        }
    }
}

impl<T> Node<T> {
    fn add_annotation<U: Annotation>(&mut self, annotation: U) {
        self.annotations
            .insert(TypeId::of::<U>(), Box::new(annotation));
    }

    fn get_annotation<U: Annotation>(&self) -> Option<&U> {
        self.annotations
            .get(&TypeId::of::<U>())
            .and_then(|boxed| boxed.downcast_ref::<U>())
    }
}

trait Annotation: 'static {}
enum AnnotationKey<T: Annotation> {
    Type,
    Loop,
    Phantom(PhantomData<T>),
}

impl Annotation for Type {}
impl AnnotationKey<Type> {
    fn key() -> Self {
        AnnotationKey::Type
    }
}

struct LoopInfo {
    definitly_terminates: bool,
}
impl Annotation for LoopInfo {}
impl AnnotationKey<LoopInfo> {
    fn key() -> Self {
        AnnotationKey::Loop
    }
}
