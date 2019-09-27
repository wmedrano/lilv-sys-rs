/// Connect a port to a data location.
/// This may be called regardless of whether the plugin is activated,
/// activation and deactivation does not destroy port connections.
#[inline(always)]
pub unsafe fn lilv_instance_get_uri(
    instance: *const crate::LilvInstance,
) -> *const std::os::raw::c_char {
    instance
        .as_ref()
        .iter()
        .flat_map(|i| i.lv2_descriptor.as_ref())
        .map(|d| d.URI)
        .next()
        .unwrap_or(std::ptr::null())
}

/// Connect a port to a data location.
/// This may be called regardless of whether the plugin is activated,
/// activation and deactivation does not destroy port connections.
#[inline(always)]
pub unsafe fn lilv_instance_connect_port(
    instance: *mut crate::LilvInstance,
    port_index: u32,
    data_location: *mut ::std::os::raw::c_void,
) {
    if let Some(instance) = instance.as_mut() {
        if let Some(descriptor) = instance.lv2_descriptor.as_ref() {
            if let Some(connect_fn) = descriptor.connect_port {
                (connect_fn)(instance.lv2_handle, port_index, data_location);
            }
        }
    }
}

/// Activate a plugin instance.
/// This resets all state information in the plugin, except for port data
/// locations (as set by lilv_instance_connect_port()).  This MUST be called
/// before calling lilv_instance_run().
pub unsafe fn lilv_instance_activate(instance: *const crate::LilvInstance) {
    if let Some(instance) = instance.as_ref() {
        if let Some(descriptor) = instance.lv2_descriptor.as_ref() {
            if let Some(activate_fn) = descriptor.activate {
                (activate_fn)(instance.lv2_handle);
            }
        }
    }
}

/// Run `instance` for `sample_count` frames.
/// If the hint lv2:hardRTCapable is set for this plugin, this function is
/// guaranteed not to block.
#[inline(always)]
pub unsafe fn lilv_instance_run(instance: *const crate::LilvInstance, sample_count: u32) {
    if let Some(instance) = instance.as_ref() {
        if let Some(descriptor) = instance.lv2_descriptor.as_ref() {
            if let Some(run_fn) = descriptor.run {
                (run_fn)(instance.lv2_handle, sample_count);
            }
        }
    }
}

/// Deactivate a plugin instance.
/// Note that to run the plugin after this you must activate it, which will
/// reset all state information (except port connections).
#[inline(always)]
pub unsafe fn lilv_instance_deactivate(instance: *mut crate::LilvInstance) {
    if let Some(instance) = instance.as_ref() {
        if let Some(descriptor) = instance.lv2_descriptor.as_ref() {
            if let Some(deactivate_fn) = descriptor.deactivate {
                (deactivate_fn)(instance.lv2_handle);
            }
        }
    }
}

/// Get extension data from the plugin instance.
/// The type and semantics of the data returned is specific to the particular
/// extension, though in all cases it is shared and must not be deleted.
#[inline(always)]
pub unsafe fn lilv_instance_get_extension_data(
    instance: *mut crate::LilvInstance,
    uri: *const ::std::os::raw::c_char,
) -> *const ::std::os::raw::c_void {
    if let Some(instance) = instance.as_ref() {
        if let Some(descriptor) = instance.lv2_descriptor.as_ref() {
            if let Some(extension_data_fn) = descriptor.extension_data {
                return (extension_data_fn)(uri);
            }
        }
    };
    std::ptr::null()
}

/// Get the LV2_Descriptor of the plugin instance.
/// Normally hosts should not need to access the LV2_Descriptor directly,
/// use the lilv_instance_* functions.
#[inline(always)]
pub unsafe fn lilv_instance_get_descriptor(
    instance: *const crate::LilvInstance,
) -> *const crate::LV2_Descriptor {
    instance
        .as_ref()
        .map(|i| i.lv2_descriptor)
        .unwrap_or(::std::ptr::null())
}

/// Get the LV2_Handle of the plugin instance.
/// Normally hosts should not need to access the LV2_Handle directly,
/// use the lilv_instance_* functions.
///
/// The returned handle is shared and must not be deleted.
#[inline(always)]
pub unsafe fn lilv_instance_get_handle(instance: *const crate::LilvInstance) -> crate::LV2_Handle {
    instance
        .as_ref()
        .map(|i| i.lv2_handle)
        .unwrap_or(::std::ptr::null_mut())
}
