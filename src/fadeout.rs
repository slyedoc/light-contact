use bevy::prelude::*;

use crate::AppState;
pub struct FadeoutPlugin;

#[derive(Debug, Component, Clone, Copy)]
pub enum Fadeout {
    Push(AppState),
    Pop,
}

//

impl Plugin for FadeoutPlugin {
    fn build(&self, app: &mut App) {
        app //.add_plugin(Material2dPlugin::<FadeoutMaterial>::default())
            //.add_startup_system(setup)
            .add_event::<Fadeout>()
            // .add_system(fadeout)
            // .add_system(ui_fadeout)
            .add_system(create_fadeout);
    }
}

// fn ui_fadeout(
//     fade_query: Query<&ScreenFade>,
//     mut ui_query: Query<&mut UiColor>,
//     mut text_query: Query<&mut Text>,
// ) {
//     if let Some(fade) = fade_query.iter().next() {
//         for mut ui_color in ui_query.iter_mut() {
//             ui_color.0.set_a(1.0 - fade.alpha);
//         }
//         for mut text in text_query.iter_mut() {
//             for section in text.sections.iter_mut() {
//                 section.style.color.set_a(1.0 - fade.alpha);
//             }
//         }
//     }
// }

// fn fadeout(
//     mut commands: Commands,
//     mut fade_query: Query<(Entity, &mut ScreenFade, &mut Sprite)>,
//     mut state: ResMut<State<AppState>>,
//     time: Res<Time>,
// ) {
//     for (entity, mut fade, mut sprite) in fade_query.iter_mut() {
//         fade.timer.tick(time.delta());
//         if fade.timer.percent() < 0.5 {
//             fade.alpha = fade.timer.percent() * 2.0;
//         } else {
//             fade.alpha = fade.timer.percent_left() * 2.0;
//         }
//         sprite.color.set_a(fade.alpha);

//         if fade.timer.percent() > 0.5 && !fade.sent {
//             if let Some(next_state) = fade.next_state {
//                 state.push(next_state).unwrap();
//             } else {
//                 state.pop().unwrap();
//             }
//             fade.sent = true;
//         }

//         if fade.timer.just_finished() {
//             commands.entity(entity).despawn_recursive();
//         }
//     }
// }

pub fn create_fadeout(mut fadeout_event: EventReader<Fadeout>, mut state: ResMut<State<AppState>>) {
    for e in fadeout_event.iter() {
        match e {
            Fadeout::Push(target) => {
                state.push(*target).unwrap();
            }
            Fadeout::Pop => state.pop().unwrap(),
        }
    }
}

// fn setup(
//     mut commands: Commands,
//     camera_image: Res<CameraImage>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut post_processing_materials: ResMut<Assets<FadeoutMaterial>>,
// ) {
//     // This specifies the layer used for the post processing camera, which will be attached to the post processing camera and 2d quad.
//     let post_processing_pass_layer = RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 1) as u8);

//     let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
//         camera_image.width as f32,
//         camera_image.height as f32,
//     ))));

//     // This material has the texture that has been rendered.
//     let material_handle = post_processing_materials.add(FadeoutMaterial {
//         source_image: camera_image.image.clone(),
//     });

//     // Post processing 2d quad, with material using the render texture done by the main camera, with a custom shader.
//     commands
//         .spawn_bundle(MaterialMesh2dBundle {
//             mesh: quad_handle.into(),
//             material: material_handle,
//             transform: Transform {
//                 translation: Vec3::new(0.0, 0.0, 1.5),
//                 ..default()
//             },
//             ..default()
//         })
//         .insert(post_processing_pass_layer)
//         .insert(Keep);

//     // The post-processing pass camera.
//     commands
//         .spawn_bundle(Camera2dBundle {
//             camera: Camera {
//                 // renders after the first main camera which has default value: 0.
//                 priority: 1,
//                 ..default()
//             },
//             ..Camera2dBundle::default()
//         })
//         .insert(post_processing_pass_layer)
//         .insert(Keep);
// }

// Region below declares of the custom material handling post processing effect

// Our custom post processing material
// #[derive(TypeUuid, Clone)]
// #[uuid = "bc2f08eb-a0fb-43f1-a908-54871ea597d5"]
// struct FadeoutMaterial {
//     /// In this example, this image will be the result of the main camera.
//     source_image: Handle<Image>,
// }

// struct FadeoutMaterialGPU {
//     bind_group: BindGroup,
// }

// impl Material2d for FadeoutMaterial {
//     fn bind_group(material: &FadeoutMaterialGPU) -> &BindGroup {
//         &material.bind_group
//     }

//     fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
//         render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
//             label: None,
//             entries: &[
//                 BindGroupLayoutEntry {
//                     binding: 0,
//                     visibility: ShaderStages::FRAGMENT,
//                     ty: BindingType::Texture {
//                         multisampled: false,
//                         view_dimension: TextureViewDimension::D2,
//                         sample_type: TextureSampleType::Float { filterable: true },
//                     },
//                     count: None,
//                 },
//                 BindGroupLayoutEntry {
//                     binding: 1,
//                     visibility: ShaderStages::FRAGMENT,
//                     ty: BindingType::Sampler(SamplerBindingType::Filtering),
//                     count: None,
//                 },
//             ],
//         })
//     }

//     fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
//         asset_server.watch_for_changes().unwrap();
//         Some(asset_server.load("shaders/fadeout.wgsl"))
//     }
// }

// impl RenderAsset for FadeoutMaterial {
//     type ExtractedAsset = FadeoutMaterial;
//     type PreparedAsset = FadeoutMaterialGPU;
//     type Param = (
//         SRes<RenderDevice>,
//         SRes<Material2dPipeline<FadeoutMaterial>>,
//         SRes<RenderAssets<Image>>,
//     );

//     fn prepare_asset(
//         extracted_asset: FadeoutMaterial,
//         (render_device, pipeline, images): &mut SystemParamItem<Self::Param>,
//     ) -> Result<FadeoutMaterialGPU, PrepareAssetError<FadeoutMaterial>> {
//         let (view, sampler) = if let Some(result) = pipeline
//             .mesh2d_pipeline
//             .get_image_texture(images, &Some(extracted_asset.source_image.clone()))
//         {
//             result
//         } else {
//             return Err(PrepareAssetError::RetryNextUpdate(extracted_asset));
//         };

//         let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
//             label: None,
//             layout: &pipeline.material2d_layout,
//             entries: &[
//                 BindGroupEntry {
//                     binding: 0,
//                     resource: BindingResource::TextureView(view),
//                 },
//                 BindGroupEntry {
//                     binding: 1,
//                     resource: BindingResource::Sampler(sampler),
//                 },
//             ],
//         });
//         Ok(FadeoutMaterialGPU { bind_group })
//     }

//     fn extract_asset(&self) -> FadeoutMaterial {
//         self.clone()
//     }
// }
