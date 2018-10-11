// TODO: module-level docs
use std::any::{Any, TypeId};
use std::fmt;

/// This is the trait implemented by application events and event 
/// families. Implementing this gives the necessary information to
/// allow `AnyEvent` conversions for this type.
/// 
/// 
/// For a struct type, a check is done for correspondence between the 
/// implementor's own `TypeId` and `ev`'s held id. Downcasting then 
/// happens with an unsafe pointer cast if the check was sucessful; 
/// otherwise, the downcast returns `None`.
/// 
/// 
/// An enum, however, encodes an event family. This means the check
/// has to consider `ev`'s id against all the `TypeId`s of members of
/// this event family. Afterwards, downcasting can happen likewise.
/// 
/// 
/// This is an unsafe trait that **should not be implemented manually**:
/// Use `#[derive(Event)]` on your types instead. See the [module-level 
/// documentation][mod] for more details.
/// 
/// ## Performance
/// 
/// Guaranteed, the runtime characteristics of this sort of reflection
/// aren't ideal. However, it seems this sort of flexibility is 
/// necessary for UI, if extensible (user-defined) events are desired.
/// 
/// 
/// For simple events, runtime characteristics should be similar to 
/// using trait objects. For event families, however, a check needs to
/// be done against all possible `TypeId`s. While this should be fast,
/// it is `O(n)`. For specially big event families where this could
/// matter, a possible optimization is to use a hash set to check for
/// `TypeId` matches instead.
/// 
/// [mod]: index.html
pub unsafe trait Event: Any + Sized {
    /// Try to downcast a dynamically typed event `ev` into an event of 
    /// this type. 
    /// 
    /// 
    /// Returns an event of this type if the cast is 
    /// successful, and `None` otherwise.
    fn filter_any(ev: AnyEvent) -> Option<Self>;

    /// Upcast this event to `AnyEvent`, making it dynamically-typed.
    fn upcast_to_any(self) -> AnyEvent;

    /// Check if the dynamically typed event `ev` typechecks against
    /// this type. 
    /// 
    /// 
    /// If `Self` is a simple event, this is a trivial check of `ev`'s
    /// dynamic type ID against `TypeId::of::<Self>()`. If `Self` is an
    /// event family, this checks `ev`'s dynamic id against the `TypeId`
    /// of all members of this family, returning `true` on any match.
    fn check_any(ev: &AnyEvent) -> bool;
}

/// This trait encodes an inheritance relationship between events. 
/// Implementors of `Subtype<E>` are members of the event family `E`.
/// 
/// 
/// You probably shouldn't implement this manually; accordingly, this 
/// trait may become unsafe in the future. Instead, use `#[derive(Event)]`
/// on an enum to create a event family, with subtyping relationships
/// automatically implemented. See the [module-level documentation][mod]
/// for more details.
/// 
/// 
/// [mod]: index.html
pub trait Subtype<E>: Event
where
    E: Event,
{
    fn filter(ev: E) -> Option<Self>;

    fn upcast(self) -> E;

    fn check(ev: &E) -> bool;
}

enum Void {}
type Destructor = Box<dyn Fn(*mut Void)>;

/// A dynamically typed event supporting downcasting. Values of this 
/// type can be created by upcasting on any implementor of `Event`.
/// 
/// 
/// The methods on this struct are mostly unsafe and low-level, meant
/// to be used inside automatic implementations of `Event`.
/// 
/// See the [module-level documentation][mod] for more info.
/// 
/// [mod]: index.html
pub struct AnyEvent {
    id: TypeId,
    data: *mut Void,
    destructor: Destructor,
}

impl AnyEvent {
    /// Unsafely creates an instance of `AnyEvent` with the given `data`.
    /// 
    /// This is should only be used inside implementations of `Event`, 
    /// which is nonetheless meant to be implemented automatically. To
    /// make your events dynamically typed, use one of the provided
    /// [upcasting methods][safe upcast] instead.
    /// 
    /// [safe upcast]: index.html
    pub unsafe fn new<E: Any>(data: E) -> Self {
        let boxed_data = Box::new(data);

        AnyEvent {
            id: TypeId::of::<E>(),
            data: Box::into_raw(boxed_data) as *mut Void,
            destructor: Box::new(|ptr: *mut Void| {
                ::std::ptr::drop_in_place(ptr as *mut E);        
            }),
        }
    }

    /// Get the `TypeId` of the value currently held by this instance
    /// of `AnyEvent`.
    pub fn get_id(&self) -> TypeId {
        self.id.clone()
    }

    /// Unsafely downcast this into an owned value of type `Ev` by doing
    /// an unchecked pointer cast. 
    /// 
    /// This is should only be used inside implementations of `Event`, 
    /// which is nonetheless meant to be implemented automatically. 
    /// Use a [checked downcast][safe downcast] instead.
    /// 
    /// [safe downcast]: index.html
    pub unsafe fn downcast_into_unchecked<Ev: Event>(self) -> Ev {
        (self.data as *mut Ev).read()
    }
}

impl fmt::Debug for AnyEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AnyEvent {{ id: {:?}, data: {:?}, destructor: {:?} }}", self.id, self.data, (&self.destructor) as *const _)
    }
}

unsafe impl Event for AnyEvent {
    fn filter_any(ev: AnyEvent) -> Option<Self> {
        Some(ev)
    }

    fn upcast_to_any(self) -> AnyEvent {
        self
    }

    fn check_any(_: &AnyEvent) -> bool {
        true
    }
}

impl<E> Subtype<AnyEvent> for E
where
    E: Event,
{
    fn filter(ev: AnyEvent) -> Option<E> {
        E::filter_any(ev)
    }

    fn upcast(self) -> AnyEvent {
        E::upcast_to_any(self)
    }

    fn check(ev: &AnyEvent) -> bool {
        E::check_any(ev)
    }
}