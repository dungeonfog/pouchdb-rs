use js_sys::{Array, Function, JsString, Reflect, Symbol};
use std::convert::AsRef;
use wasm_bindgen::{closure::Closure, JsValue};

pub mod changes_event_emitter;
pub mod replication_event_emitter;

#[derive(Debug, Clone, PartialEq)]
pub struct SequenceID(pub(crate) JsValue);

#[derive(Debug, Clone)]
pub enum EventName {
    String(JsString),
    Symbol(Symbol),
}

impl EventName {
    pub fn string(name: &str) -> Self {
        Self::String(JsString::from(name))
    }
    pub fn symbol(symbol: &Symbol) -> Self {
        Self::Symbol(symbol.clone())
    }
}

impl AsRef<JsValue> for EventName {
    fn as_ref(&self) -> &JsValue {
        match self {
            Self::String(s) => s.as_ref(),
            Self::Symbol(sym) => sym.as_ref(),
        }
    }
}

pub struct EventListener {
    emitter: JsValue,
    event_name: EventName,
    closure: Closure<dyn Fn(JsValue)>,
}

impl EventListener {
    fn new(
        emitter: &EventEmitter,
        event_name: &EventName,
        closure: impl Fn(JsValue) + 'static,
    ) -> Result<Self, JsValue> {
        let on = Reflect::get(&emitter.0, &JsValue::from_str("on"))?;
        if on.is_function() {
            let closure = Closure::wrap(Box::new(closure) as Box<dyn Fn(JsValue)>);
            Function::from(on).call2(&emitter.0, event_name.as_ref(), closure.as_ref())?;
            Ok(Self {
                emitter: emitter.0.clone(),
                event_name: event_name.clone(),
                closure,
            })
        } else {
            Err(JsValue::from_str("No `on` function found."))
        }
    }
}

impl Drop for EventListener {
    /// Removes the listener from the listener array for its event.
    fn drop(&mut self) {
        if let Ok(remove_listener) =
            Reflect::get(&self.emitter, &JsValue::from_str("remove_listener"))
        {
            if remove_listener.is_function() {
                Function::from(remove_listener)
                    .call2(
                        &self.emitter,
                        self.event_name.as_ref(),
                        self.closure.as_ref(),
                    )
                    .ok();
            }
        }
    }
}

/// All objects that emit events are instances of the EventEmitter class. These objects expose
/// an [add_listener] function that allows one or more functions to be attached to named events
/// emitted by the object. Typically, event names are camel-cased strings but any valid JavaScript
/// property key can be used.
///
/// When the EventEmitter object emits an event, all of the functions attached to that specific
/// event are called synchronously.
pub struct EventEmitter(JsValue);

impl EventEmitter {
    pub(crate) fn new(value: JsValue) -> Self {
        Self(value)
    }
    pub(crate) fn as_js(&self) -> &JsValue {
        &self.0
    }
    /// Synchronously calls each of the listeners registered for the event named `event_name`, in the order
    /// they were registered, passing the supplied arguments to each.
    pub fn emit(&self, event_name: &EventName, event_args: Vec<JsValue>) -> Result<bool, JsValue> {
        let args = Array::new();
        args.push(event_name.as_ref());
        for arg in event_args {
            args.push(&arg);
        }

        let emit = Reflect::get(&self.0, &JsValue::from_str("emit"))?;
        if emit.is_function() {
            Function::from(emit)
                .apply(&self.0, &args)
                .map(|result| result.is_truthy())
        } else {
            Err(JsValue::from_str("No emit function found."))
        }
    }
    /// Returns an array listing the events for which the emitter has registered listeners.
    pub fn event_names(&self) -> Result<Vec<EventName>, JsValue> {
        let event_names = Reflect::get(&self.0, &JsValue::from_str("eventNames"))?;
        if event_names.is_function() {
            Function::from(event_names).call0(&self.0).map(|result| {
                Array::from(&result)
                    .iter()
                    .filter_map(|event| {
                        if event.is_string() {
                            Some(EventName::String(event.into()))
                        } else if event.is_symbol() {
                            Some(EventName::Symbol(event.into()))
                        } else {
                            None
                        }
                    })
                    .collect()
            })
        } else {
            Err(JsValue::from_str("No eventNames function found."))
        }
    }
    /// Adds the listener function to the end of the listeners array for the event named eventName. No
    /// checks are made to see if the listener has already been added. Multiple calls passing the same
    /// combination of `event_name` and `listener` will result in the listener being added, and called,
    /// multiple times.
    pub fn add_listener(
        &self,
        event_name: &EventName,
        listener: impl Fn(JsValue) + 'static,
    ) -> Result<EventListener, JsValue> {
        EventListener::new(self, event_name, listener)
    }
    pub fn once(
        &self,
        _event_name: &EventName,
        _listener: impl FnOnce(JsValue) + 'static,
    ) -> Result<(), JsValue> {
        unimplemented!()
    }
    /// Removes all listeners, or those of the specified `event_name`.
    ///
    /// It is bad practice to remove listeners added elsewhere in the code, particularly when the [EventEmitter]
    /// instance was created by some other component or module (e.g. sockets or file streams).
    pub fn remove_all_listeners(&self, event_name: Option<&EventName>) -> Result<(), JsValue> {
        let remove_all_listeners = Reflect::get(&self.0, &JsValue::from_str("removeAllListeners"))?;
        if remove_all_listeners.is_function() {
            if let Some(event_name) = event_name {
                Function::from(remove_all_listeners).call1(&self.0, event_name.as_ref())?;
            } else {
                Function::from(remove_all_listeners).call0(&self.0)?;
            }
            Ok(())
        } else {
            Err(JsValue::from_str("No remove_all_listeners function found."))
        }
    }
}
