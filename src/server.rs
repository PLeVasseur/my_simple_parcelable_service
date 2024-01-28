//! MySimpleParcelable service.
use mysimpleparcelableservice::MySimpleParcelableService;
use com_example_mysimpleparcelableservice::aidl::com::example::mysimpleparcelableservice::IMySimpleParcelableService::BnMySimpleParcelableService;
use com_example_mysimpleparcelableservice::binder;

const SERVICE_IDENTIFIER: &str = "mysimpleparcelableservice";

/// Entry point for mysimpleparcelable service.
fn main() {
    let my_simple_parcelable_service = MySimpleParcelableService;
    let my_simple_parcelable_service_binder = BnMySimpleParcelableService::new_binder(
        my_simple_parcelable_service,
        binder::BinderFeatures::default(),
    );
    binder::add_service(SERVICE_IDENTIFIER, my_simple_parcelable_service_binder.as_binder())
        .expect("Failed to register service");
    binder::ProcessState::join_thread_pool()
}
