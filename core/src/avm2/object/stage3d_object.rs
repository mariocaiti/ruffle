//! Object representation for Stage3D objects

use crate::avm2::activation::Activation;
use crate::avm2::object::script_object::ScriptObjectData;
use crate::avm2::object::{ClassObject, Object, ObjectPtr, TObject};
use crate::avm2::value::Value;
use crate::avm2::Error;
use core::fmt;
use gc_arena::{Collect, GcCell, GcWeakCell, MutationContext};
use std::cell::{Ref, RefMut};

/// A class instance allocator that allocates Stage3D objects.
pub fn stage_3d_allocator<'gc>(
    class: ClassObject<'gc>,
    activation: &mut Activation<'_, 'gc>,
) -> Result<Object<'gc>, Error<'gc>> {
    let base = ScriptObjectData::new(class);

    Ok(Stage3DObject(GcCell::new(
        activation.context.gc_context,
        Stage3DObjectData {
            base,
            context3d: None,
            visible: true,
        },
    ))
    .into())
}

#[derive(Clone, Collect, Copy)]
#[collect(no_drop)]
pub struct Stage3DObject<'gc>(pub GcCell<'gc, Stage3DObjectData<'gc>>);

#[derive(Clone, Collect, Copy, Debug)]
#[collect(no_drop)]
pub struct Stage3DObjectWeak<'gc>(pub GcWeakCell<'gc, Stage3DObjectData<'gc>>);

impl fmt::Debug for Stage3DObject<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Stage3DObject")
            .field("ptr", &self.0.as_ptr())
            .finish()
    }
}

impl<'gc> Stage3DObject<'gc> {
    pub fn context3d(self) -> Option<Object<'gc>> {
        self.0.read().context3d
    }

    pub fn set_context3d(self, context3d: Object<'gc>, mc: MutationContext<'gc, '_>) {
        self.0.write(mc).context3d = Some(context3d);
    }

    pub fn visible(self) -> bool {
        self.0.read().visible
    }

    pub fn set_visible(self, visible: bool, mc: MutationContext<'gc, '_>) {
        self.0.write(mc).visible = visible;
    }
}

#[derive(Clone, Collect)]
#[collect(no_drop)]
pub struct Stage3DObjectData<'gc> {
    /// Base script object
    base: ScriptObjectData<'gc>,

    /// The context3D object associated with this Stage3D object,
    /// if it's been created with `requestContext3D`
    context3d: Option<Object<'gc>>,

    visible: bool,
}

impl<'gc> TObject<'gc> for Stage3DObject<'gc> {
    fn base(&self) -> Ref<ScriptObjectData<'gc>> {
        Ref::map(self.0.read(), |read| &read.base)
    }

    fn base_mut(&self, mc: MutationContext<'gc, '_>) -> RefMut<ScriptObjectData<'gc>> {
        RefMut::map(self.0.write(mc), |write| &mut write.base)
    }

    fn as_ptr(&self) -> *const ObjectPtr {
        self.0.as_ptr() as *const ObjectPtr
    }

    fn value_of(&self, _mc: MutationContext<'gc, '_>) -> Result<Value<'gc>, Error<'gc>> {
        Ok(Value::Object(Object::from(*self)))
    }

    fn as_stage_3d(&self) -> Option<Stage3DObject<'gc>> {
        Some(*self)
    }
}
