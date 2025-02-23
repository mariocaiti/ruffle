use crate::avm2::activation::Activation;
pub use crate::avm2::object::error_allocator;
use crate::avm2::object::Object;
use crate::avm2::string::AvmString;
use crate::avm2::value::Value;
use crate::avm2::Error;
use crate::avm2::TObject;

pub fn get_stack_trace<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(error) = this.as_error_object() {
        return Ok(AvmString::new(activation.context.gc_context, error.display_full()?).into());
    }
    Ok(Value::Undefined)
}
