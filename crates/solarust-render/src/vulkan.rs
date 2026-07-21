use ash::Entry;

use crate::vulkan::Error::LoaderInitializeFailed;


enum Error{
    LoaderInitializeFailed,
    DeviceInitializeFailed,
}

struct Loader{
    entry: ash::Entry,
    instance: ash::Instance,
    surface_func: ash::khr::surface::Instance
}
impl Loader {
    pub fn initialize(app_name: &str, version: (u32, u32, u32), enable_debug: bool) -> Result<Self, Error> {
        unsafe{
            let entry = Entry::load().map_err(|_| LoaderInitializeFailed);
            Ok(
                Self{

                }
            )
        }
    }
}
