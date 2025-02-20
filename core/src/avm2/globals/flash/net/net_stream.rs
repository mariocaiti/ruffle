use crate::avm2::{Activation, Error, Object, TObject, Value};

pub use crate::avm2::object::netstream_allocator as net_stream_allocator;

pub fn get_bytes_loaded<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(ns) = this.as_netstream() {
        return Ok(ns.bytes_loaded().into());
    }

    Ok(Value::Undefined)
}

pub fn get_bytes_total<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(ns) = this.as_netstream() {
        return Ok(ns.bytes_total().into());
    }

    Ok(Value::Undefined)
}

pub fn play<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(ns) = this.as_netstream() {
        let name = args
            .get(0)
            .cloned()
            .filter(|v| !matches!(v, Value::Null))
            .map(|v| v.coerce_to_string(activation))
            .transpose()?;

        ns.play(&mut activation.context, name);
    }

    Ok(Value::Undefined)
}

pub fn pause<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(ns) = this.as_netstream() {
        ns.pause(&mut activation.context);
    }

    Ok(Value::Undefined)
}

pub fn resume<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(ns) = this.as_netstream() {
        ns.resume(&mut activation.context);
    }

    Ok(Value::Undefined)
}

pub fn toggle_pause<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(ns) = this.as_netstream() {
        ns.toggle_paused(&mut activation.context);
    }

    Ok(Value::Undefined)
}
