// // Complete from https://github.com/ForesightMiningSoftwareCorporation/bevy_infinite_grid
// // Wanted to update for my branch and use as example

// use bevy::{
//     ecs::system::{lifetimeless::SRes, SystemParamItem},
//     pbr::{MaterialPipeline, NotShadowCaster},
//     prelude::*,
//     reflect::TypeUuid,
//     render::{
//         render_resource::{AsBindGroup, ShaderRef},
//         view::NoFrustumCulling,
//     },
// };

// static SHADER: &str = include_str!("grid.wgsl");

// const SHADER_HANDLE: HandleUntyped =
//     HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 15204473893972682982);

// const GRID_MESH_HANDLE: HandleUntyped =
//     HandleUntyped::weak_from_u64(Mesh::TYPE_UUID, 10583255013429636210);

// pub struct InfiniteGridPlugin;

// impl Plugin for InfiniteGridPlugin {
//     fn build(&self, app: &mut App) {
//         app.world
//             .resource_mut::<Assets<Shader>>()
//             .set_untracked(SHADER_HANDLE, Shader::from_wgsl(SHADER));

//         app.world
//             .resource_mut::<Assets<Mesh>>()
//             .set_untracked(GRID_MESH_HANDLE, Mesh::from(shape::Plane { size: 1.0 }));

//         app.add_plugin(MaterialPlugin::<InfiniteGridMaterial>::default());
//     }
// }

// #[derive(AsBindGroup, Debug, Clone, TypeUuid)]
// #[uuid = "dc369438-2cf9-4934-883e-59b3db6f8a9d"]
// pub struct InfiniteGridMaterial {
//     #[uniform(0)]
//     pub rot_matrix: Mat3,    
//     pub offset: Vec3,
//     pub normal: Vec3,
//     pub scale: f32, 
//     pub x_axis_color: Color,
//     pub z_axis_color: Color,
//     pub shadow_color: Color,
// }

// impl Default for InfiniteGridMaterial {
//     fn default() -> Self {
//         Self {
//             offset: Vec3::ZERO,
//             normal: Vec3::Y,
//             scale: 1.,
//             rot_matrix: Mat3::IDENTITY,
//             x_axis_color: Color::rgb(1.0, 0.2, 0.2),
//             z_axis_color: Color::rgb(0.2, 0.2, 1.0),
//             shadow_color: Color::rgba(0.2, 0.2, 0.2, 0.7),
//         }
//     }
// }

// impl Material for InfiniteGridMaterial {
//     fn fragment_shader() -> ShaderRef {
//         ShaderRef::Handle(SHADER_HANDLE.typed::<Shader>())
//     }

//     fn vertex_shader() -> ShaderRef {
//         ShaderRef::Handle(SHADER_HANDLE.typed::<Shader>())
//     }
    
//     fn alpha_mode(&self) -> AlphaMode {
//         AlphaMode::Blend
//     }
// }



// #[derive(Bundle)]
// pub struct InfiniteGridBundle {
//     #[bundle]
//     material_mesh_bundle: MaterialMeshBundle<InfiniteGridMaterial>,
//     no_frustum_culling: NoFrustumCulling,
//     not_shadow_caster: NotShadowCaster,
// }

// impl InfiniteGridBundle {
//     pub fn new(grid_material_handle: Handle<InfiniteGridMaterial>) -> Self {
//         Self {
//             material_mesh_bundle: MaterialMeshBundle {
//                 material: grid_material_handle,
//                 mesh: GRID_MESH_HANDLE.typed(),
//                 ..Default::default()
//             },
//             no_frustum_culling: NoFrustumCulling,
//             not_shadow_caster: NotShadowCaster,
//         }
//     }
// }
