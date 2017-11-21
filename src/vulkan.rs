pub struct Vulkan {
    events_loop: winit::EventsLoop,
    instance: vulkano::instance::Instance,
    window: vulkano_win::Window,
}

pub struct Window {

}

impl Vulkan {
    pub fn new(events_loop: winit::EventsLoop) -> Vulkan {
        let instance = {
            let extensions = vulkano_win::required_extensions();
            Instance::new(None, &extensions, None).expect("failed to create Vulkan instance")
        };
    }
    pub fn create_window() {

    }
}