use crate::ASTnode::CairoCode;
use std::collections::LinkedList;

pub(crate) trait CairoType {
    fn member_types(&mut self) -> LinkedList<Type>{
        LinkedList::new()
    }

    fn life_time_parameters(&mut self) -> LinkedList<u8>{
        LinkedList::new()
    }
}

pub enum Type {
    U8Type,
    U16Type,
    U32Type,
    U64Type,
    U128Type,
    U256Type,
    Felt252Type,
    USizeType,
    StringType,
    BooleanType,
    TupleType,
    StructType,
    ArrayType,
    FunctionType,
    VoidType,
}

struct U8Type;

impl CairoCode for U8Type {
    fn to_cairo(&mut self) -> String{
        "u8".to_string()
    }
}

impl CairoType for U8Type {
    fn member_types() -> LinkedList<U8Type>{
        let list: LinkedList<U8Type> = LinkedList::new();
        list
    }

    fn life_time_parameters() -> LinkedList<u8>{
        LinkedList::new()
    }
}

struct U16Type;

impl CairoCode for U16Type {
    fn to_cairo(&mut self) -> String{
        "u16".to_string()
    }
}

impl CairoType for U16Type {
    fn member_types() -> LinkedList<U16Type>{
        let list: LinkedList<U16Type> = LinkedList::new();
        list
    }

    fn life_time_parameters() -> LinkedList<u8>{
        LinkedList::new()
    }
}

struct U32Type;

impl CairoCode for U32Type {
    fn to_cairo(&mut self) -> String{
        "u32".to_string()
    }
}

impl CairoType for U32Type {
    fn member_types(&mut self) -> LinkedList<U32Type>{
        let list: LinkedList<U32Type> = LinkedList::new();
        list
    }

    fn life_time_parameters() -> LinkedList<u8>{
        LinkedList::new()
    }
}

struct U64Type;

impl CairoCode for U64Type {
    fn to_cairo(&mut self) -> String{
        "u64".to_string()
    }
}

impl CairoType for U64Type {
    fn member_types(&mut self) -> LinkedList<U64Type>{
        let list: LinkedList<U64Type> = LinkedList::new();
        list
    }

    fn life_time_parameters() -> LinkedList<u8>{
        LinkedList::new()
    }
}

struct U128Type;

impl CairoCode for U128Type {
    fn to_cairo(&mut self) -> String{
        "u128".to_string()
    }
}

impl CairoType for U128Type {
    fn member_types() -> LinkedList<U128Type>{
        let list: LinkedList<U128Type> = LinkedList::new();
        list
    }

    fn life_time_parameters() -> LinkedList<u8>{
        LinkedList::new()
    }
}

struct U256Type;

impl CairoCode for U256Type {
    fn to_cairo(&mut self) -> String{
        "u256".to_string()
    }
}

impl CairoType for U256Type {
    fn member_types() -> LinkedList<U256Type>{
        let list: LinkedList<U256Type> = LinkedList::new();
        list
    }

    fn life_time_parameters() -> LinkedList<u8>{
        LinkedList::new()
    }
}

struct Felt252Type;

impl CairoCode for Felt252Type {
    fn to_cairo(&mut self) -> String{
        "felt252".to_string()
    }
}

impl CairoType for Felt252Type {
    fn member_types() -> LinkedList<Felt252Type>{
        let list: LinkedList<Felt252Type> = LinkedList::new();
        list
    }

    fn life_time_parameters() -> LinkedList<u8>{
        LinkedList::new()
    }
}

struct USizeType;

impl CairoCode for USizeType {
    fn to_cairo(&mut self) -> String{
        "usize".to_string()
    }
}

impl CairoType for USizeType {
    fn member_types() -> LinkedList<USizeType>{
        let list: LinkedList<USizeType> = LinkedList::new();
        list
    }

    fn life_time_parameters() -> LinkedList<u8>{
        LinkedList::new()
    }
}

struct StringType;

impl CairoCode for StringType {
    fn to_cairo(&mut self) -> String{
        "".to_string()
    }
}

impl CairoType for StringType {
    fn member_types() -> LinkedList<StringType>{
        let list: LinkedList<StringType> = LinkedList::new();
        list
    }

    fn life_time_parameters() -> LinkedList<u8>{
        LinkedList::new()
    }
}

struct BooleanType;

impl CairoCode for BooleanType {
    fn to_cairo(&mut self) -> String{
        "bool".to_string()
    }
}

impl CairoType for BooleanType {
    fn member_types() -> LinkedList<BooleanType>{
        let list: LinkedList<BooleanType> = LinkedList::new();
        list
    }

    fn life_time_parameters() -> LinkedList<u8>{
        LinkedList::new()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct TupleType {
    types: LinkedList<Type>
}

impl CairoCode for TupleType {
    fn to_cairo(&mut self) -> String{
        let cairo_types = self.types
        .iter()
        .map(|e| e.to_cairo())
        .collect::<Vec<_>>()
        .join(" , ");

        format!("({})", cairo_types)
    }
}

impl CairoType for TupleType {
    fn member_types(&mut self) -> LinkedList<TupleType>{
        let member_types = self.types
        .iter()
        .map(|e| e.member_types())
        .collect();
        
        member_types
    }

    fn life_time_parameters(&mut self) -> LinkedList<u8>{
        LinkedList::new()
    }

}

#[derive(Clone, Debug, PartialEq, Eq)]
struct StructType {
    struct_name: String,
    types: LinkedList<Type>,
}

impl CairoCode for StructType {
    fn to_cairo(&mut self) -> String{
        self.struct_name
    }
}

impl CairoType for StructType {
    fn member_types(&mut self) -> LinkedList<Type>{
        let member_types = self.types
        .iter()
        .map(|e| e.member_types())
        .collect();
        
        member_types
    }

    fn life_time_parameters(&mut self) -> LinkedList<u8>{
        LinkedList::new()
    }

}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ArrayType {
    internal_type: Type,
    size: usize,
}

impl CairoCode for ArrayType {
    fn to_cairo(&mut self) -> String{
        format!("Array<{}>", self.internal_type)
    }
}

impl CairoType for ArrayType {
    fn member_types(&mut self) -> LinkedList<Type>{
        self.internal_type.member_types()
    }

    fn life_time_parameters(&mut self) -> LinkedList<u8>{
        LinkedList::new()
    }

}

#[derive(Clone, Debug, PartialEq, Eq)]
struct FunctionType {
    return_type: Type,
    args: LinkedList<Type>,
}

impl CairoCode for FunctionType {
    fn to_cairo(&mut self) -> String{
        let cairo_args = self.args
        .iter()
        .map(|e| e.to_cairo())
        .collect()
        .join(" , ");

        format!("fn({}) -> {}", cairo_args, self.return_type.to_cairo())
    }
}

impl CairoType for FunctionType {
    fn member_types(&mut self) -> LinkedList<Type>{
        let member_types = self.types
        .iter()
        .map(|e| e.member_types())
        .collect();
        
        member_types
    }

    fn life_time_parameters(&mut self) -> LinkedList<u8>{
        LinkedList::new()
    }

}

#[derive(Clone, Debug, PartialEq, Eq)]
struct VoidType {
    internal_type: String,
    size: usize,
}

impl CairoCode for VoidType {
    fn to_cairo(&mut self) -> String{
        "()".to_string()
    }
}

impl CairoType for VoidType {
    fn member_types(&mut self) -> LinkedList<VoidType>{
        let list: LinkedList<VoidType> = LinkedList::new();
        list
    }

    fn life_time_parameters() -> LinkedList<u8>{
        LinkedList::new()
    }

}

