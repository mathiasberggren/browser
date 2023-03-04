use vulkano::VulkanLibrary;
use vulkano::{instance::Instance, instance::InstanceCreateInfo};
use winit::event_loop::{EventLoop};
use winit::window::{WindowBuilder};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
let instance = Instance::new(library, InstanceCreateInfo {
    enumerate_portability: true,
    ..Default::default()
}).expect("failed to create instance");

for physical_device in instance.enumerate_physical_devices().unwrap() {
    println!("Available device: {}", physical_device.properties().device_name);
};


let event_loop = EventLoop::new();  // ignore this for now

// The surface is a cross-platform abstraction over the actual window object, that Vulkano can use for rendering
let _surface = WindowBuilder::new()
    .build(&event_loop)
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

}

