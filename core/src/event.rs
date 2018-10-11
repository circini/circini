use std::any::{Any, TypeId};
use std::fmt;

pub unsafe trait Event: Any + Sized {
    fn filter_any(ev: AnyEvent) -> Option<Self>;

    fn upcast_to_any(self) -> AnyEvent;

    fn check_any(ev: &AnyEvent) -> bool;
}

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

pub struct AnyEvent {
    id: TypeId,
    data: *mut Void,
    destructor: Destructor,
}

impl AnyEvent {
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

    pub fn get_id(&self) -> TypeId {
        self.id.clone()
    }

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