use bevy::{
    asset::load_internal_asset,
    prelude::{AddAsset, HandleUntyped, MaterialPlugin, Plugin, Shader},
    reflect::TypeUuid,
};
pub mod radial_dissolve_pbr;
pub use radial_dissolve_pbr::{
    RadialDissolveStandardMaterial, RadialDissolveStandardMaterialKey,
    RadialDissolveStandardMaterialUniform,
};

pub const NOISE_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 8222222222227029775);
pub const MY_PBR_TYPES_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 8822225359337029775);
pub const MY_PBR_BINDINGS_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 8777777779337029775);

pub const RADIAL_DISSOLVE_PBR_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 6666666666693416751);

pub struct ShaderPlaygroundPlugin;
impl Plugin for ShaderPlaygroundPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        load_internal_asset!(
            app,
            NOISE_SHADER_HANDLE,
            "../assets/noise.wgsl",
            Shader::from_wgsl
        );

        load_internal_asset!(
            app,
            MY_PBR_TYPES_HANDLE,
            "../assets/my_pbr_types.wgsl",
            Shader::from_wgsl
        );

        load_internal_asset!(
            app,
            MY_PBR_BINDINGS_HANDLE,
            "../assets/my_pbr_bindings.wgsl",
            Shader::from_wgsl
        );

        load_internal_asset!(
            app,
            RADIAL_DISSOLVE_PBR_HANDLE,
            "../assets/radial_dissolve_standard_material.wgsl",
            Shader::from_wgsl
        );
        app.add_plugin(MaterialPlugin::<RadialDissolveStandardMaterial>::default());
        app.register_asset_reflect::<RadialDissolveStandardMaterial>();
    }
}
