#![forbid(unsafe_code)]

use std::sync::Arc;

use eframe::{NativeOptions, NativeViewportHooks};

#[test]
fn native_viewport_hooks_are_absent_by_default() {
    let options = NativeOptions::default();
    assert!(options.native_viewport_hooks.is_none());
    assert!(options.clone().native_viewport_hooks.is_none());
}

#[test]
fn native_viewport_hooks_survive_options_clone_without_retaining_the_old_once_hook() {
    let hooks = NativeViewportHooks {
        window_attributes: Some(Arc::new(|_, _, attributes| attributes)),
        window_created: Some(Arc::new(|_, _, _| {})),
    };
    let options = NativeOptions {
        native_viewport_hooks: Some(hooks.clone()),
        window_builder: Some(Box::new(|builder| builder)),
        ..Default::default()
    };
    let cloned = options.clone();
    let cloned_hooks = cloned
        .native_viewport_hooks
        .as_ref()
        .expect("new hook bundle survives clone");
    assert!(Arc::ptr_eq(
        hooks.window_attributes.as_ref().expect("attributes hook"),
        cloned_hooks
            .window_attributes
            .as_ref()
            .expect("cloned attributes hook"),
    ));
    assert!(Arc::ptr_eq(
        hooks.window_created.as_ref().expect("created hook"),
        cloned_hooks
            .window_created
            .as_ref()
            .expect("cloned created hook"),
    ));
    assert!(options.window_builder.is_some());
    assert!(
        cloned.window_builder.is_none(),
        "legacy consumed hook behavior stays unchanged"
    );
}

#[test]
fn either_native_viewport_hook_can_be_supplied_independently() {
    let attributes_only = NativeViewportHooks {
        window_attributes: Some(Arc::new(|_, _, attributes| attributes)),
        ..Default::default()
    };
    let created_only = NativeViewportHooks {
        window_created: Some(Arc::new(|_, _, _| {})),
        ..Default::default()
    };
    assert!(attributes_only.window_created.is_none());
    assert!(created_only.window_attributes.is_none());
}
