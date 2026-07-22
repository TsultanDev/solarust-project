use std::{ffi::{CStr, CString}, sync::Arc};

use ash::{Entry, vk::{API_VERSION_1_1, ApplicationInfo, Bool32, DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagsEXT, DebugUtilsMessengerCallbackDataEXT, DebugUtilsMessengerCreateInfoEXT, DebugUtilsMessengerEXT, EXT_DEBUG_UTILS_NAME, FALSE, InstanceCreateInfo, KHR_SURFACE_NAME, make_api_version}};

use crate::vulkan::Error::LoaderInitializeFailed;


enum Error{
    LoaderInitializeFailed,
    DeviceInitializeFailed,
}

struct Loader{
    entry: ash::Entry,
    instance: ash::Instance,
    debugger: Option<DebugUtilsMessengerEXT>,
    debug_loader: Option<ash::ext::debug_utils::Instance>,
    surface_func: ash::khr::surface::Instance
}
impl Loader {
    pub fn initialize(display: raw_window_handle::RawDisplayHandle, window: raw_window_handle::RawWindowHandle, app_name: &str, version: (u32, u32, u32), enable_debug: bool) -> Result<Self, Error> {
        unsafe{
            let entry = Entry::load().map_err(|_| LoaderInitializeFailed)?;

            let c_name = CStr::from_ptr(app_name.as_ptr() as *const i8);
            let app_info = ApplicationInfo::default().api_version(API_VERSION_1_1).application_name(c_name).application_version(make_api_version(0, version.0, version.1, version.2))
                .engine_name(c"Solarust Engine").engine_version(make_api_version(0, 0, 1, 0));

            let surface_extensions = ash_window::enumerate_required_extensions(display).map_err(|_| LoaderInitializeFailed)?;
            let mut extensions : Vec<*const i8> = Vec::from(surface_extensions);
            let mut layers : Vec<*const i8> = Vec::new();
            if enable_debug {
                extensions.push(EXT_DEBUG_UTILS_NAME.as_ptr());
                layers.push(c"VK_LAYER_KHRONOS_validation".as_ptr());
            }

            let mut messenger_info = if enable_debug{ Some(DebugUtilsMessengerCreateInfoEXT::default().message_severity(DebugUtilsMessageSeverityFlagsEXT::ERROR | DebugUtilsMessageSeverityFlagsEXT::INFO
                | DebugUtilsMessageSeverityFlagsEXT::VERBOSE | DebugUtilsMessageSeverityFlagsEXT::WARNING).message_type(DebugUtilsMessageTypeFlagsEXT::GENERAL | DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
                    | DebugUtilsMessageTypeFlagsEXT::VALIDATION).pfn_user_callback(Some(Loader::debug_callback)))}
            else{
                None
            };

            let instance_info = if enable_debug{
                InstanceCreateInfo::default().application_info(&app_info).enabled_extension_names(&extensions).enabled_layer_names(&layers)
            }
            else{
                InstanceCreateInfo::default().application_info(&app_info).enabled_extension_names(&extensions).enabled_layer_names(&layers).push_next(&mut messenger_info.unwrap())
            };

            let instance = entry.create_instance(&instance_info, None).map_err(|_| LoaderInitializeFailed)?;

            let surface_loader = ash::khr::surface::Instance::new(&entry, &instance);
            let debug_loader = if enable_debug{
                Some(ash::ext::debug_utils::Instance::new(&entry, &instance))}
            else{
                None
            };
            let messenger = if enable_debug{
                Some(debug_loader.as_ref().unwrap().create_debug_utils_messenger(&messenger_info.unwrap(), None).map_err(|_| LoaderInitializeFailed)?)
            }
            else{
                None
            };

            Ok(
                Self{
                    entry: entry,
                    instance: instance,
                    surface_func: surface_loader,
                    debug_loader: debug_loader,
                    debugger: messenger,
                }
            )
        }
    }
    unsafe extern "system" fn debug_callback(msg_severity: DebugUtilsMessageSeverityFlagsEXT, msg_type: DebugUtilsMessageTypeFlagsEXT, msg_data: *const DebugUtilsMessengerCallbackDataEXT, _p_user_data: *mut std::ffi::c_void) -> Bool32{
        unsafe{
            let callback_data = *msg_data;
            let message = CStr::from_ptr(callback_data.p_message).to_string_lossy();
            println!("[Vulkan {:?}] [{:?}]: {}", msg_severity, msg_type, message);

            FALSE
        }
    }
}
