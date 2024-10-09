use std::fmt;

pub use indexmap::{IndexMap, IndexSet};
pub use semver::Version;

#[derive(Debug)]
pub struct Doc {
    pub version: Version,
    pub root: Module,
    pub compiled_with: IndexMap<String, ShaderDefValue>,
}

#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub source_url: Option<String>,
    pub modules: Vec<Module>,
    pub constants: Vec<Constant>,
    pub global_variables: Vec<GlobalVariable>,
    pub structs: Vec<Struct>,
    pub functions: Vec<Function>,
    pub shader_defs: IndexSet<String>,
}

impl Module {
    pub fn new(name: String) -> Module {
        Self {
            name,
            source_url: None,
            modules: Vec::new(),
            constants: Vec::new(),
            global_variables: Vec::new(),
            structs: Vec::new(),
            functions: Vec::new(),
            shader_defs: IndexSet::new(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ShaderDefValue {
    Bool(bool),
    Int(i32),
    UInt(u32),
}

impl fmt::Display for ShaderDefValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShaderDefValue::Bool(value) => write!(f, "{}", value),
            ShaderDefValue::Int(value) => write!(f, "{}i", value),
            ShaderDefValue::UInt(value) => write!(f, "{}u", value),
        }
    }
}

#[derive(Debug)]
pub struct Constant {
    pub name: Ident,
    pub ty: Type,
    pub init: Expression,
    /// Comment is stored as a single string, without starting or trailing comment markers.
    /// It is fair to expect markdown.
    pub comment: Option<String>,
}

#[derive(Debug)]
pub struct GlobalVariable {
    pub name: Ident,
    pub space: AddressSpace,
    pub binding: Option<ResourceBinding>,
    pub ty: Type,
    pub init: Option<Expression>,
    /// Comment is stored as a single string, without starting or trailing comment markers.
    /// It is fair to expect markdown.
    pub comment: Option<String>,
}

#[derive(Debug)]
pub enum AddressSpace {
    Function,
    Private,
    WorkGroup,
    Uniform,
    Storage { load: bool, store: bool },
    Handle,
    PushConstant,
}

impl fmt::Display for AddressSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AddressSpace::Function => write!(f, "<function>"),
            AddressSpace::Private => write!(f, "<private>"),
            AddressSpace::WorkGroup => write!(f, "<workgroup>"),
            AddressSpace::Uniform => write!(f, "<uniform>"),
            AddressSpace::Storage { store, .. } => {
                if *store {
                    write!(f, "<storage, read_write>")
                } else {
                    write!(f, "<storage>")
                }
            }
            AddressSpace::Handle => write!(f, ""),
            AddressSpace::PushConstant => write!(f, "<push_constant>"),
        }
    }
}

#[derive(Debug)]
pub struct ResourceBinding {
    pub group: u32,
    pub binding: u32,
}

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    Unknown,
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Literal(literal) => write!(f, "{}", literal),
            Expression::Unknown => write!(f, ".."),
        }
    }
}

#[derive(Debug)]
pub enum Literal {
    F64(f64),
    F32(f32),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    Bool(bool),

    AbstractInt(i64),
    AbstractFloat(f64),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::F64(value) => write!(f, "{}", value),
            Literal::F32(value) => write!(f, "{}", value),
            Literal::U32(value) => write!(f, "{}", value),
            Literal::I32(value) => write!(f, "{}", value),
            Literal::U64(value) => write!(f, "{}", value),
            Literal::I64(value) => write!(f, "{}", value),
            Literal::Bool(value) => write!(f, "{}", value),

            Literal::AbstractInt(value) => write!(f, "{}", value),
            Literal::AbstractFloat(value) => write!(f, "{}", value),
        }
    }
}

#[derive(Debug)]
pub struct Struct {
    pub name: Ident,
    pub members: Vec<StructMember>,
    /// Comment is stored as a single string, without starting or trailing comment markers.
    /// It is fair to expect markdown.
    pub comment: Option<String>,
}

#[derive(Debug)]
pub struct StructMember {
    pub name: Ident,
    pub ty: Type,
    pub binding: Option<Binding>,
    /// Comment is stored as a single string, without starting or trailing comment markers.
    /// It is fair to expect markdown.
    pub comment: Option<String>,
}

#[derive(Debug)]
pub enum Type {
    Named {
        name: String,
        def_path: Option<Vec<String>>,
    },
    Pointer(Box<Type>),
    PointerWithAddressSpace {
        base: Box<Type>,
        address_space: &'static str,
        maybe_access: Option<&'static str>,
    },
    ArrayConstant(Box<Type>, Option<u32>),
    ArrayDynamic(Box<Type>),
    BindingArrayConstant(Box<Type>, Option<u32>),
    BindingArrayDynamic(Box<Type>),
    Unnamed,
}

#[derive(Debug)]
pub struct Function {
    pub name: Ident,
    pub arguments: Vec<FunctionArgument>,
    pub ret: Option<Type>,
    /// Comment is stored as a single string, without starting or trailing comment markers.
    /// It is fair to expect markdown.
    pub comment: Option<String>,
}

#[derive(Debug)]
pub struct FunctionArgument {
    pub name: Ident,
    pub ty: Type,
    pub binding: Option<Binding>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Ident {
    Named(String),
    Unnamed,
}

impl<T> From<Option<T>> for Ident
where
    T: Into<String>,
{
    fn from(name: Option<T>) -> Self {
        match name {
            Some(name) => Ident::Named(name.into()),
            None => Ident::Unnamed,
        }
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ident::Named(name) => write!(f, "{}", name),
            Ident::Unnamed => write!(f, "unknown"),
        }
    }
}

#[derive(Debug)]
pub enum Binding {
    BuiltIn(BuiltIn),
    Location {
        location: u32,
        second_blend_source: bool,
        interpolation: Option<Interpolation>,
        sampling: Option<Sampling>,
    },
}

#[derive(Debug)]
pub enum BuiltIn {
    Position { invariant: bool },
    ViewIndex,
    BaseInstance,
    BaseVertex,
    ClipDistance,
    CullDistance,
    InstanceIndex,
    PointSize,
    VertexIndex,
    FragDepth,
    PointCoord,
    FrontFacing,
    PrimitiveIndex,
    SampleIndex,
    SampleMask,
    GlobalInvocationId,
    LocalInvocationId,
    LocalInvocationIndex,
    WorkGroupId,
    WorkGroupSize,
    NumWorkGroups,
    NumSubgroups,
    SubgroupId,
    SubgroupSize,
    SubgroupInvocationId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Interpolation {
    Perspective,
    Linear,
    Flat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Sampling {
    Center,
    Centroid,
    Sample,
}
