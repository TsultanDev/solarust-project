use std::ffi::{CStr, CString};

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
    pub fn initialize(app_name: &str, version: (u32, u32, u32), enable_debug: bool) -> Result<Self, Error> {
        unsafe{
            let entry = Entry::load().map_err(|_| LoaderInitializeFailed)?;

            let c_name = CStr::from_ptr(app_name.as_ptr() as *const i8);
            let app_info = ApplicationInfo::default().api_version(API_VERSION_1_1).application_name(c_name).application_version(make_api_version(0, version.0, version.1, version.2))
                .engine_name(c"Solarust Engine").engine_version(make_api_version(0, 0, 1, 0));

            let extension_names : Vec<*const u8>= Vec::new();
            let layer_names = [c"VK_LAYER_KHRONOS_validation".as_ptr()];

            let instance_info = InstanceCreateInfo::default().application_info(&app_info).enabled_extension_names(&extension_names).enabled_layer_names(&layer_names);

            let instance = entry.create_instance(&instance_info, None).map_err(|_| LoaderInitializeFailed)?;

            let surface_loader = ash::khr::surface::Instance::new(&entry, &instance);
            let debug_loader = ash::ext::debug_utils::Instance::new(&entry, &instance);

            let messenger_info = DebugUtilsMessengerCreateInfoEXT::default().message_severity(DebugUtilsMessageSeverityFlagsEXT::ERROR | DebugUtilsMessageSeverityFlagsEXT::INFO
                | DebugUtilsMessageSeverityFlagsEXT::VERBOSE | DebugUtilsMessageSeverityFlagsEXT::WARNING).message_type(DebugUtilsMessageTypeFlagsEXT::GENERAL | DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
                    | DebugUtilsMessageTypeFlagsEXT::VALIDATION).pfn_user_callback(Some(Loader::debug_callback));

            let messenger = debug_loader.create_debug_utils_messenger(&messenger_info, None).map_err(|_| LoaderInitializeFailed)?;

            Ok(
                Self{
                    entry: entry,
                    instance: instance,
                    surface_func: surface_loader,
                    debug_loader: Some(debug_loader),
                    debugger: Some(messenger)
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
