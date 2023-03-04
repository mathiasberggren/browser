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

    let event_loop = EventLoop::new();  


    // Had to downgrade winit to 0.27.5 to get this to work
    let surface = WindowBuilder::new()
        .build_vk_surface(&event_loop, instance.clone())
        .unwrap();

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

    use vulkano::device::DeviceExtensions;

    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::empty()
    };

    // make sure we select the best device possible for rendering, prefereably a GPU
    let (physical_device, queue_family_index) = select_physical_device(&instance, &surface, &device_extensions);

    use vulkano::device::{Device, DeviceCreateInfo, QueueCreateInfo};

    let (device, mut queues) = Device::new(
        physical_device.clone(),
        DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            enabled_extensions: device_extensions,
            ..Default::default()
        },
    )
    .expect("failed to create device");
    
    let queue = queues.next().unwrap();

    let caps = physical_device
        .surface_capabilities(&surface, Default::default())
        .expect("failed to get surface capabilities");

    let dimensions = surface.window().inner_size();
    let composite_alpha = caps.supported_composite_alpha.iter().next().unwrap();
    let image_format = Some(
        physical_device
            .surface_formats(&surface, Default::default())
            .unwrap()[0]
            .0,
    );


    use vulkano::image::ImageUsage;
    use vulkano::swapchain::{Swapchain, SwapchainCreateInfo};

    // To ensure that only complete images are shown, Vulkan uses what is called a swapchain.
    // Basically we draw everything that is going to be rendered on a separate screen before displaying it.
    let (swapchain, images) = Swapchain::new(
        device.clone(),
        surface.clone(),
        SwapchainCreateInfo {
            min_image_count: caps.min_image_count + 1, // How many buffers to use in the swapchain
            image_format,
            image_extent: dimensions.into(),
            image_usage: ImageUsage {
                color_attachment: true,  // What the images are going to be used for
                ..Default::default()
            },
            composite_alpha,
            ..Default::default()
        },
    ).unwrap();


}

