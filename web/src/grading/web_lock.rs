use std::cell::RefCell;
use std::future::Future;
use std::rc::Rc;

use js_sys::{Function, Object, Promise, Reflect};
use shared::{AppError, AppErrorStatic};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

use crate::artifact_client::js_interop;

const LOCK_MANAGER_PROPERTY: &str = "locks";
const IF_AVAILABLE_OPTION: &str = "ifAvailable";

#[wasm_bindgen]
extern "C" {
    type LockManager;

    #[wasm_bindgen(method, js_name = request)]
    fn request(this: &LockManager, name: &str, on_granted: &Function) -> Promise;

    #[wasm_bindgen(method, js_name = request)]
    fn request_with_options(this: &LockManager, name: &str, options: &Object, on_granted: &Function) -> LockRequest;

    type LockRequest;

    #[wasm_bindgen(method, js_name = catch)]
    fn catch_with(this: &LockRequest, on_rejected: &Function) -> Promise;
}

/// A Web Lock this page holds until `release`.
pub struct HeldWebLock {
    release: Function,
}

impl HeldWebLock {
    pub fn release(self) {
        _ = self.release.call0(&JsValue::NULL);
    }
}

/// The lock named `name` across every page of this origin, or `None` while another request holds it.
pub async fn request_if_available(name: &str) -> Result<Option<HeldWebLock>, AppErrorStatic> {
    let options: Object = Object::new();
    Reflect::set(&options, &JsValue::from_str(IF_AVAILABLE_OPTION), &JsValue::TRUE).map_err(js_interop::error)?;

    let (granted, resolve_granted, reject_granted): (Promise, Function, Function) = create_settleable_promise()?;
    let reject_granted_when_unheld: Function = reject_granted.clone();
    let on_granted: JsValue = Closure::once_into_js(move |lock: JsValue| -> Promise {
        if lock.is_null() {
            _ = resolve_granted.call1(&JsValue::NULL, &JsValue::NULL);
            return Promise::resolve(&JsValue::UNDEFINED);
        }

        // The lock is held until this promise resolves.
        let held: Result<(Promise, Function, Function), AppError> = create_settleable_promise();

        match held {
            Ok((held, release, _)) => {
                _ = resolve_granted.call1(&JsValue::NULL, &release);
                held
            },
            Err(error) => {
                _ = reject_granted_when_unheld.call1(&JsValue::NULL, &JsValue::from_str(&error.to_string()));
                Promise::resolve(&JsValue::UNDEFINED)
            },
        }
    });
    let requested: LockRequest = get_lock_manager()?.request_with_options(name, &options, on_granted.unchecked_ref());

    _ = requested.catch_with(&reject_granted);

    let granted_release: JsValue = JsFuture::from(granted).await.map_err(js_interop::error)?;

    if granted_release.is_null() {
        return Ok(None);
    }

    Ok(Some(HeldWebLock {
        release: js_interop::dyn_into(granted_release)?,
    }))
}

/// Runs `operation` once this page holds the lock named `name`, waiting for any other holder, and holds it until
/// `operation` ends.
pub async fn run_while_holding<T: 'static>(name: &str, operation: impl Future<Output = T> + 'static) -> Result<T, AppErrorStatic> {
    let outcome: Rc<RefCell<Option<T>>> = Rc::new(RefCell::new(None));
    let recorded_outcome: Rc<RefCell<Option<T>>> = Rc::clone(&outcome);
    let on_granted: JsValue = Closure::once_into_js(move |_lock: JsValue| -> Promise {
        wasm_bindgen_futures::future_to_promise(async move {
            let operation_outcome: T = operation.await;

            recorded_outcome.replace(Some(operation_outcome));
            Ok(JsValue::UNDEFINED)
        })
    });

    let requested: Promise = get_lock_manager()?.request(name, on_granted.unchecked_ref());
    JsFuture::from(requested).await.map_err(js_interop::error)?;

    let operation_outcome: Option<T> = outcome.take();

    operation_outcome.ok_or_else(|| AppErrorStatic::from(AppError::from(format!("the Web Lock's holder did not finish; [name={name}]"))))
}

fn get_lock_manager() -> Result<LockManager, AppError> {
    let navigator: web_sys::Navigator = js_interop::get_window()?.navigator();
    let lock_manager: JsValue = Reflect::get(&navigator, &JsValue::from_str(LOCK_MANAGER_PROPERTY)).map_err(js_interop::error)?;

    if lock_manager.is_undefined() {
        return Err(AppError::from("this browser has no Web Locks".to_string()));
    }

    Ok(lock_manager.unchecked_into::<LockManager>())
}

/// Runs the promise's executor, which the constructor calls before it returns, to keep its resolve and reject functions.
fn create_settleable_promise() -> Result<(Promise, Function, Function), AppError> {
    let mut settle_functions: Option<(Function, Function)> = None;
    let promise: Promise = Promise::new(&mut |resolve: Function, reject: Function| settle_functions = Some((resolve, reject)));

    let (resolve, reject): (Function, Function) =
        settle_functions.ok_or_else(|| AppError::from("the promise's executor did not run".to_string()))?;

    Ok((promise, resolve, reject))
}
