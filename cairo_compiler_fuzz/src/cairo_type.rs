use crate::astnode::CairoCode;

pub(crate) trait CairoType<T>  {
    fn member_types(&mut self) -> Vec<T>{
        Vec::new()
    }

    fn life_time_parameters(&mut self) -> Vec<u8>{
        Vec::new()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

impl CairoCode for Type {
    fn to_cairo(&mut self) -> String{
        self.to_cairo()
    }
}

impl CairoType<Type> for Type {
    fn member_types(&mut self) -> Vec<Type>{
        self.member_types()
    }

    fn life_time_parameters(&mut self) -> Vec<u8>{
        self.life_time_parameters()
    }
}

pub struct U8Type;

impl CairoCode for U8Type {
    fn to_cairo(&mut self) -> String{
        "u8".to_string()
    }
}

impl CairoType<U8Type> for U8Type {
    fn member_types(&mut self) -> Vec<U8Type>{
        let list: Vec<U8Type> = Vec::new();
        list
    }

    fn life_time_parameters(&mut self) -> Vec<u8>{
        Vec::new()
    }
}

pub struct U16Type;

impl CairoCode for U16Type {
    fn to_cairo(&mut self) -> String{
        "u16".to_string()
    }
}

impl CairoType<U16Type> for U16Type {
    fn member_types(&mut self) -> Vec<U16Type>{
        let list: Vec<U16Type> = Vec::new();
        list
    }

    fn life_time_parameters(&mut self) -> Vec<u8>{
        Vec::new()
    }
}

pub struct U32Type;

impl CairoCode for U32Type {
    fn to_cairo(&mut self) -> String{
        "u32".to_string()
    }
}

impl CairoType<U32Type> for U32Type {
    fn member_types(&mut self) -> Vec<U32Type>{
        let list: Vec<U32Type> = Vec::new();
        list
    }

    fn life_time_parameters(&mut self) -> Vec<u8>{
        Vec::new()
    }
}

pub struct U64Type;

impl CairoCode for U64Type {
    fn to_cairo(&mut self) -> String{
        "u64".to_string()
    }
}

impl CairoType<U64Type> for U64Type {
    fn member_types(&mut self) -> Vec<U64Type>{
        let list: Vec<U64Type> = Vec::new();
        list
    }

    fn life_time_parameters(&mut self) -> Vec<u8>{
        Vec::new()
    }
}

pub struct U128Type;

impl CairoCode for U128Type {
    fn to_cairo(&mut self) -> String{
        "u128".to_string()
    }
}

impl CairoType<U128Type> for U128Type {
    fn member_types(&mut self) -> Vec<U128Type>{
        let list: Vec<U128Type> = Vec::new();
        list
    }

    fn life_time_parameters(&mut self) -> Vec<u8>{
        Vec::new()
    }
}

pub struct U256Type;

impl CairoCode for U256Type {
    fn to_cairo(&mut self) -> String{
        "u256".to_string()
    }
}

impl CairoType<U256Type> for U256Type {
    fn member_types(&mut self) -> Vec<U256Type>{
        let list: Vec<U256Type> = Vec::new();
        list
    }

    fn life_time_parameters(&mut self) -> Vec<u8>{
        Vec::new()
    }
}

pub struct Felt252Type;

impl CairoCode for Felt252Type {
    fn to_cairo(&mut self) -> String{
        "felt252".to_string()
    }
}

impl CairoType<Felt252Type> for Felt252Type {
    fn member_types(&mut self) -> Vec<Felt252Type>{
        let list: Vec<Felt252Type> = Vec::new();
        list
    }

    fn life_time_parameters(&mut self) -> Vec<u8>{
        Vec::new()
    }
}

pub struct USizeType;

impl CairoCode for USizeType {
    fn to_cairo(&mut self) -> String{
        "usize".to_string()
    }
}

impl CairoType<USizeType> for USizeType {
    fn member_types(&mut self) -> Vec<USizeType>{
        let list: Vec<USizeType> = Vec::new();
        list
    }

    fn life_time_parameters(&mut self) -> Vec<u8>{
        Vec::new()
    }
}

pub struct StringType;

impl CairoCode for StringType {
    fn to_cairo(&mut self) -> String{
        "".to_string()
    }
}

impl CairoType<StringType> for StringType {
    fn member_types(&mut self) -> Vec<StringType>{
        let list: Vec<StringType> = Vec::new();
        list
    }

    fn life_time_parameters(&mut self) -> Vec<u8>{
        Vec::new()
    }
}

pub struct BooleanType;

impl CairoCode for BooleanType {
    fn to_cairo(&mut self) -> String{
        "bool".to_string()
    }
}

impl CairoType<BooleanType> for BooleanType {
    fn member_types(&mut self) -> Vec<BooleanType>{
        let list: Vec<BooleanType> = Vec::new();
        list
    }

    fn life_time_parameters(&mut self) -> Vec<u8>{
        Vec::new()
    }
}

#[derive(Clone, Debug)]
pub struct TupleType {
    types: Vec<Type>
}

impl CairoCode for TupleType {
    fn to_cairo(&mut self) -> String{
        let cairo_types = self.types
        .clone()
        .iter_mut()
        .map(|e| e.to_cairo())
        .collect::<Vec<_>>()
        .join(" , ");

        format!("({})", cairo_types)
    }
}

impl CairoType<Type> for TupleType {
    fn member_types(&mut self) -> Vec<Type>{
        self.types.iter_mut().flat_map(|(t)| t.member_types()).collect()
    }

    fn life_time_parameters(&mut self) -> Vec<u8>{
        Vec::new()
    }

}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructType {
    struct_name: String,
    types: Vec<(String, Type)>,
}

impl StructType {
    pub fn struct_name(&mut self) -> String {
        self.struct_name.clone()
    }

    pub fn types(&mut self) -> Vec<(String, Type)> {
        self.types.clone()
    }
}

impl CairoCode for StructType {
    fn to_cairo(&mut self) -> String{
        self.struct_name.clone()
    }
}

impl CairoType<Type> for StructType {
    fn member_types(&mut self) -> Vec<Type>{
        self.types.iter_mut().flat_map(|(s, t)| t.member_types()).collect()
    }

    fn life_time_parameters(&mut self) -> Vec<u8>{
        Vec::new()
    }

}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArrayType {
    internal_type: Type,
    size: usize,
}

impl CairoCode for ArrayType {
    fn to_cairo(&mut self) -> String{
        format!("Array<{}>", self.internal_type.to_cairo())
    }
}

impl CairoType<Type> for ArrayType {
    fn member_types(&mut self) -> Vec<Type>{
        self.internal_type.member_types()
    }

    fn life_time_parameters(&mut self) -> Vec<u8>{
        Vec::new()
    }

}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionType {
    return_type: Type,
    args: Vec<Type>,
}

impl CairoCode for FunctionType {
    fn to_cairo(&mut self) -> String{
        let cairo_args = self.args
        .iter_mut()
        .map(|e| e.to_cairo())
        .collect::<Vec<_>>()
        .join(" , ");

        format!("fn({}) -> {}", cairo_args, self.return_type.to_cairo())
    }
}

impl CairoType<Type> for FunctionType {
    fn member_types(&mut self) -> Vec<Type>{
        self.args.iter_mut().flat_map(|(t)| t.member_types()).collect()
    }

    fn life_time_parameters(&mut self) -> Vec<u8>{
        Vec::new()
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

impl CairoType<VoidType> for VoidType {
    fn member_types(&mut self) -> Vec<VoidType>{
        let list: Vec<VoidType> = Vec::new();
        list
    }

    fn life_time_parameters(&mut self) -> Vec<u8>{
        Vec::new()
    }

}

