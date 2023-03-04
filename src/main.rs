use std::sync::Arc;

use rush::swapchain::{select_physical_device};
use vulkano::VulkanLibrary;
use vulkano::swapchain::Surface;
use vulkano::{instance::Instance, instance::InstanceCreateInfo};
use vulkano_win::{create_surface_from_winit, VkSurfaceBuild};
use winit::event_loop::{EventLoop};
use winit::window::{WindowBuilder};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
    let required_extensions = vulkano_win::required_extensions(&library);
    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            enumerate_portability: true,
            enabled_extensions: required_extensions,
            ..Default::default()
        }
    ).expect("failed to create instance");;


    for physical_device in instance.enumerate_physical_devices().unwrap() {
        println!("Available device: {}", physical_device.properties().device_name);
    };

    // ignore this for now
    let event_loop = EventLoop::new();  

    // let window = Arc::new(WindowBuilder::new()
    //     .build(&event_loop)
    //     .unwrap());


    // Had to downgrade winit to 0.27.5 to get this to work
    let surface = WindowBuilder::new()
        .build_vk_surface(&event_loop, instance.clone())
        .unwrap();

    // let surface = create_surface_from_winit(window, instance).unwrap();
    use winit::event::{Event, WindowEvent};
    use winit::event_loop::ControlFlow;
        
    // Create infinity loop
    event_loop.run(|event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            },
            _ => ()
        }
    });

    // To ensure that only complete images are shown, Vulkan uses what is called a swapchain.
    // Basically we draw everything that is going to be rendered on a separate screen before displaying it.
    use vulkano::device::DeviceExtensions;

    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::empty()
    };

    // make sure we select the best device possible for rendering, prefereably a GPU
    let (physical_device, queue_family_index) = select_physical_device(&instance, &surface, &device_extensions);



}

