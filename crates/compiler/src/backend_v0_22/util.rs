use super::naga;
use docs::{
    AddressSpace, Binding, BuiltIn, Expression, Interpolation, Literal, ResourceBinding, Sampling,
};

pub fn build_expression(expression: &naga::Expression) -> Expression {
    match expression {
        naga::Expression::Literal(lit) => Expression::Literal(match *lit {
            naga::Literal::F64(v) => Literal::F64(v),
            naga::Literal::F32(v) => Literal::F32(v),
            naga::Literal::F16(v) => Literal::F16(v.into()),
            naga::Literal::U32(v) => Literal::U32(v),
            naga::Literal::I32(v) => Literal::I32(v),
            naga::Literal::Bool(v) => Literal::Bool(v),
            naga::Literal::I64(v) => Literal::I64(v),
            naga::Literal::U64(v) => Literal::U64(v),
            naga::Literal::AbstractInt(v) => Literal::AbstractInt(v),
            naga::Literal::AbstractFloat(v) => Literal::AbstractFloat(v),
        }),
        _ => Expression::Unknown,
    }
}

pub fn build_resource_binding(binding: &naga::ResourceBinding) -> ResourceBinding {
    ResourceBinding {
        group: binding.group,
        binding: binding.binding,
    }
}

pub fn build_address_space(address_space: &naga::AddressSpace) -> AddressSpace {
    match address_space {
        naga::AddressSpace::Function => AddressSpace::Function,
        naga::AddressSpace::Private => AddressSpace::Private,
        naga::AddressSpace::WorkGroup => AddressSpace::WorkGroup,
        naga::AddressSpace::Uniform => AddressSpace::Uniform,
        naga::AddressSpace::Storage { access } => AddressSpace::Storage {
            load: access.contains(naga::StorageAccess::LOAD),
            store: access.contains(naga::StorageAccess::STORE),
        },
        naga::AddressSpace::Handle => AddressSpace::Handle,
        naga::AddressSpace::Immediate => AddressSpace::Immediate,
        naga::AddressSpace::TaskPayload => AddressSpace::TaskPayload,
        naga::AddressSpace::RayPayload => AddressSpace::RayPayload,
        naga::AddressSpace::IncomingRayPayload => AddressSpace::IncomingRayPayload,
    }
}

pub fn build_binding(binding: &naga::Binding) -> Binding {
    match binding {
        naga::Binding::BuiltIn(builtin) => Binding::BuiltIn(build_builtin(builtin)),
        naga::Binding::Location {
            location,
            blend_src: second_blend_source,
            interpolation,
            sampling,
            per_primitive: _, // TODO: ?
        } => Binding::Location {
            location: *location,
            second_blend_source: second_blend_source.is_some(), // TODO: pass index
            interpolation: interpolation.as_ref().map(build_interpolation),
            sampling: sampling.as_ref().map(build_sampling),
        },
    }
}

pub fn build_builtin(builtin: &naga::BuiltIn) -> BuiltIn {
    match builtin {
        naga::BuiltIn::Position { invariant } => BuiltIn::Position {
            invariant: *invariant,
        },
        naga::BuiltIn::ViewIndex => BuiltIn::ViewIndex,
        naga::BuiltIn::BaseInstance => BuiltIn::BaseInstance,
        naga::BuiltIn::BaseVertex => BuiltIn::BaseVertex,
        naga::BuiltIn::ClipDistance => BuiltIn::ClipDistance,
        naga::BuiltIn::CullDistance => BuiltIn::CullDistance,
        naga::BuiltIn::InstanceIndex => BuiltIn::InstanceIndex,
        naga::BuiltIn::PointSize => BuiltIn::PointSize,
        naga::BuiltIn::VertexIndex => BuiltIn::VertexIndex,
        naga::BuiltIn::FragDepth => BuiltIn::FragDepth,
        naga::BuiltIn::PointCoord => BuiltIn::PointCoord,
        naga::BuiltIn::FrontFacing => BuiltIn::FrontFacing,
        naga::BuiltIn::PrimitiveIndex => BuiltIn::PrimitiveIndex,
        naga::BuiltIn::SampleIndex => BuiltIn::SampleIndex,
        naga::BuiltIn::SampleMask => BuiltIn::SampleMask,
        naga::BuiltIn::GlobalInvocationId => BuiltIn::GlobalInvocationId,
        naga::BuiltIn::LocalInvocationId => BuiltIn::LocalInvocationId,
        naga::BuiltIn::LocalInvocationIndex => BuiltIn::LocalInvocationIndex,
        naga::BuiltIn::WorkGroupId => BuiltIn::WorkGroupId,
        naga::BuiltIn::WorkGroupSize => BuiltIn::WorkGroupSize,
        naga::BuiltIn::NumWorkGroups => BuiltIn::NumWorkGroups,
        naga::BuiltIn::NumSubgroups => BuiltIn::NumSubgroups,
        naga::BuiltIn::SubgroupId => BuiltIn::SubgroupId,
        naga::BuiltIn::SubgroupSize => BuiltIn::SubgroupSize,
        naga::BuiltIn::SubgroupInvocationId => BuiltIn::SubgroupInvocationId,
        naga::BuiltIn::DrawIndex => BuiltIn::DrawID,

        naga::BuiltIn::Barycentric { perspective } => BuiltIn::Barycentric {
            perspective: *perspective,
        },
        naga::BuiltIn::MeshTaskSize => BuiltIn::MeshTaskSize,
        naga::BuiltIn::CullPrimitive => BuiltIn::CullPrimitive,
        naga::BuiltIn::PointIndex => BuiltIn::PointIndex,
        naga::BuiltIn::LineIndices => BuiltIn::LineIndices,
        naga::BuiltIn::TriangleIndices => BuiltIn::TriangleIndices,
        naga::BuiltIn::VertexCount => BuiltIn::VertexCount,
        naga::BuiltIn::Vertices => BuiltIn::Vertices,
        naga::BuiltIn::PrimitiveCount => BuiltIn::PrimitiveCount,
        naga::BuiltIn::Primitives => BuiltIn::Primitives,
        naga::BuiltIn::RayInvocationId => BuiltIn::RayInvocationId,
        naga::BuiltIn::NumRayInvocations => BuiltIn::NumRayInvocations,
        naga::BuiltIn::InstanceCustomData => BuiltIn::InstanceCustomData,
        naga::BuiltIn::GeometryIndex => BuiltIn::GeometryIndex,
        naga::BuiltIn::WorldRayOrigin => BuiltIn::WorldRayOrigin,
        naga::BuiltIn::WorldRayDirection => BuiltIn::WorldRayDirection,
        naga::BuiltIn::ObjectRayOrigin => BuiltIn::ObjectRayOrigin,
        naga::BuiltIn::ObjectRayDirection => BuiltIn::ObjectRayDirection,
        naga::BuiltIn::RayTmin => BuiltIn::RayTmin,
        naga::BuiltIn::RayTCurrentMax => BuiltIn::RayTCurrentMax,
        naga::BuiltIn::ObjectToWorld => BuiltIn::ObjectToWorld,
        naga::BuiltIn::WorldToObject => BuiltIn::WorldToObject,
        naga::BuiltIn::HitKind => BuiltIn::HitKind,
    }
}

pub fn build_interpolation(interpolation: &naga::Interpolation) -> Interpolation {
    match interpolation {
        naga::Interpolation::Perspective => Interpolation::Perspective,
        naga::Interpolation::Linear => Interpolation::Linear,
        naga::Interpolation::Flat => Interpolation::Flat,
        naga::Interpolation::PerVertex => Interpolation::PerVertex,
    }
}

pub fn build_sampling(sampling: &naga::Sampling) -> Sampling {
    match sampling {
        naga::Sampling::Center => Sampling::Center,
        naga::Sampling::Centroid => Sampling::Centroid,
        naga::Sampling::Sample => Sampling::Sample,
        naga::Sampling::First => Sampling::First,
        naga::Sampling::Either => Sampling::Either,
    }
}
