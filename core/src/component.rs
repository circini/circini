use super::{Event, AnyEvent};

pub trait Component {
    type Receives: Event;

    fn on(&mut self, ev: Self::Receives);
}

pub trait Attach<C>
where
    C: Component,
{
    fn attach(&mut self, other: C);
}

enum State {}

type DynamicEventHandler = Box<dyn Fn(*mut State, AnyEvent)>;

// TODO: store destructor and stop leaking
struct DynamicComponent {
    handler: DynamicEventHandler,
    state: *mut State,
}

impl<E, C> From<C> for DynamicComponent
where
    E: Event,
    C: Component<Receives=E>
{
    fn from(comp: C) -> DynamicComponent {
        let handler = |state: *mut State, ev: AnyEvent| {
            match E::filter_any(ev) {
                Some(ev) => {
                    unsafe {
                        C::on(&mut *(state as *mut C), ev)
                    }
                },
                None => ()
            }
        };
        let handler = Box::new(handler) as DynamicEventHandler;

        let state = Box::into_raw(Box::new(comp)) as *mut State;
        DynamicComponent {
            handler,
            state,
        }
    }
}

impl DynamicComponent {
    fn handle(&mut self, ev: AnyEvent){
        (self.handler)(self.state,ev)
    }
}

struct Container {
    comps: Vec<DynamicComponent>,
}

impl Component for Container {
    type Receives = AnyEvent;

    fn on(&mut self, ev: AnyEvent) {
        for c in self.comps.iter_mut() {
            c.handle(ev.clone()) 
        }
    }
}