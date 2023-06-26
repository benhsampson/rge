use vulkano::{
    instance::{Instance, InstanceCreateInfo},
    VulkanLibrary,
};
use vulkano_win::VkSurfaceBuild;
use winit::{event_loop::EventLoop, window::WindowBuilder};

fn main() {
    let library = VulkanLibrary::new().unwrap();
    // Window-drawing requires some non-core extensions that we need to enable
    // manually. Ask the `vulkano_win` crate for the list of extensions
    // required to draw a window
    let required_extensions = vulkano_win::required_extensions(&library);
    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            enabled_extensions: required_extensions,
            enumerate_portability: true,
            ..Default::default()
        },
    )
    .unwrap();

    // Create the window. First create a `WindowBuilder` from the `winit` crate
    // Then call the `build_vk_surface` provided by the VkSurfaceBuild trait
    // from `vulkano_win`. This returns a `vulkano::swapchain::Surface` object
    // that contains both a cross-platform winit window and a cross-platform
    // Vulkan surface that represents the surface of the window
    let event_loop = EventLoop::new();
    let surface = WindowBuilder::new()
        .build_vk_surface(&event_loop, instance.clone())
        .unwrap();
}
