use ruffle_render::pixel_bender::{
    parse_shader, PixelBenderParam, PixelBenderParamQualifier, OUT_COORD_NAME,
};

use crate::{
    avm2::{
        parameters::ParametersExt, string::AvmString, Activation, Error, Object, TObject, Value,
    },
    pixel_bender::PixelBenderTypeExt,
};

use super::shader_parameter::make_shader_parameter;

pub use crate::avm2::object::shader_data_allocator;

/// Implements `ShaderData.init`, which is called from the constructor
pub fn init<'gc>(
    activation: &mut Activation<'_, 'gc>,
    mut this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let bytecode = args.get_object(activation, 0, "bytecode")?;
    let bytecode = bytecode.as_bytearray().unwrap();
    let shader = parse_shader(bytecode.bytes()).expect("Failed to parse PixelBender");

    for meta in &shader.metadata {
        let name = AvmString::new_utf8(activation.context.gc_context, &meta.key);
        let value = meta.value.as_avm2_value(activation)?;
        this.set_public_property(name, value, activation)?;
    }
    this.set_public_property(
        "name",
        AvmString::new_utf8(activation.context.gc_context, &shader.name).into(),
        activation,
    )?;

    for (index, param) in shader.params.iter().enumerate() {
        let name = match &param {
            PixelBenderParam::Normal {
                name, qualifier, ..
            } => {
                // Neither of these show up in Flash Player
                if name == OUT_COORD_NAME || matches!(qualifier, PixelBenderParamQualifier::Output)
                {
                    continue;
                }
                name
            }
            PixelBenderParam::Texture { name, .. } => name,
        };

        let name = AvmString::new_utf8(activation.context.gc_context, name);
        let param_obj = make_shader_parameter(activation, param, index)?;
        this.set_public_property(name, param_obj, activation)?;
    }

    let shader_handle = activation
        .context
        .renderer
        .compile_pixelbender_shader(shader)
        .expect("Failed to compile PixelBender shader");

    this.as_shader_data()
        .unwrap()
        .set_pixel_bender_shader(shader_handle, activation.context.gc_context);
    Ok(Value::Undefined)
}
