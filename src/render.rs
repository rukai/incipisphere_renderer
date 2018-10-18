use winit::{Window, WindowBuilder, EventsLoop};
use vulkano_win;
use vulkano_win::VkSurfaceBuild;
use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::command_buffer::{DynamicState, AutoCommandBufferBuilder};
use vulkano::descriptor::pipeline_layout::PipelineLayoutAbstract;
use vulkano::device::{Device, Queue, DeviceExtensions};
use vulkano::framebuffer::{Framebuffer, FramebufferAbstract, Subpass, RenderPassAbstract};
use vulkano::image::SwapchainImage;
use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::vertex::SingleBufferDefinition;
use vulkano::pipeline::viewport::Viewport;
use vulkano::swapchain::{Surface, Swapchain, SurfaceTransform, AcquireError, PresentMode, SwapchainCreationError};
use vulkano::swapchain;
use vulkano::sync;
use vulkano::sync::{GpuFuture, FlushError};
use vulkano_shaders::vulkano_shader;
use genmesh::generators::IcoSphere;
use genmesh::{MapToVertices, Vertices};

use std::iter;
use std::sync::Arc;
use std::mem;

use state::State;
use state::RenderMode;

#[derive(Debug, Clone)]
struct Vertex { position: [f32; 3] }
impl_vertex!(Vertex, position);

vulkano_shader!{
    mod_name: vs,
    ty: "vertex",
    path: "shaders/vertex.glsl",
}

vulkano_shader!{
    mod_name: fs,
    ty: "fragment",
    path: "shaders/fragment.glsl",
}

pub struct Render {
    surface:              Arc<Surface<Window>>,
    device:               Arc<Device>,
    future:               Box<GpuFuture>,
    swapchain:            Arc<Swapchain<Window>>,
    queue:                Arc<Queue>,
    vs:                   vs::Shader,
    fs:                   fs::Shader,
    pipelines:            Pipelines,
    render_pass:          Arc<RenderPassAbstract + Send + Sync>,
    framebuffers:         Vec<Arc<FramebufferAbstract + Send + Sync>>,
    width:                u32,
    height:               u32,
    vertex_buffer_sphere: Arc<CpuAccessibleBuffer<[Vertex]>>,
}

struct Pipelines {
    standard:  Arc<GraphicsPipeline<SingleBufferDefinition<Vertex>, Box<PipelineLayoutAbstract + Send + Sync>, Arc<RenderPassAbstract + Send + Sync>>>,
    wireframe: Arc<GraphicsPipeline<SingleBufferDefinition<Vertex>, Box<PipelineLayoutAbstract + Send + Sync>, Arc<RenderPassAbstract + Send + Sync>>>,
}

impl Render {
    pub fn new(events_loop: &EventsLoop) -> Render {
        let instance = {
            let extensions = vulkano_win::required_extensions();
            Instance::new(None, &extensions, None).expect("failed to create Vulkan instance")
        };

        let physical_device = PhysicalDevice::enumerate(&instance).next().expect("no device available");
        println!("Using device: {} (type: {:?})", physical_device.name(), physical_device.ty());

        let surface = WindowBuilder::new().build_vk_surface(&events_loop, instance.clone()).unwrap();
        surface.window().set_title("Medium Renderer");

        let queue_family = physical_device.queue_families().find(|&q| {
            q.supports_graphics() && surface.is_supported(q).unwrap_or(false)
        }).unwrap();

        let (device, mut queues) = {
            let device_ext = DeviceExtensions { khr_swapchain: true, .. DeviceExtensions::none() };
            Device::new(physical_device, physical_device.supported_features(), &device_ext, [(queue_family, 0.5)].iter().cloned()).unwrap()
        };

        let future = Box::new(sync::now(device.clone())) as Box<GpuFuture>;

        let queue = queues.next().unwrap();

        let (swapchain, images) = {
            let caps = surface.capabilities(physical_device)
                             .expect("failed to get surface capabilities");
            let dimensions = caps.current_extent.unwrap_or([1024, 768]);
            let alpha = caps.supported_composite_alpha.iter().next().unwrap();
            let format = caps.supported_formats[0].0;
            Swapchain::new(device.clone(), surface.clone(), caps.min_image_count, format, dimensions, 1,
                caps.supported_usage_flags, &queue, SurfaceTransform::Identity, alpha, PresentMode::Fifo, true, None
            ).unwrap()
        };

        let vs = vs::Shader::load(device.clone()).expect("failed to create shader module");
        let fs = fs::Shader::load(device.clone()).expect("failed to create shader module");

        let (render_pass, pipelines, framebuffers) = Render::pipelines(&vs, &fs, device.clone(), swapchain.clone(), &images);

        let sphere: Vec<_> = IcoSphere::subdivide(4)
            .vertex(|v| Vertex { position: v.pos.into() })
            .vertices()
            .collect();
        let vertex_buffer_sphere = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), sphere.iter().cloned()).unwrap();

        Render { surface, device, future, swapchain, queue, vs, fs, pipelines, render_pass, framebuffers, width: 0, height: 0, vertex_buffer_sphere }
    }

    fn pipelines(
        vs: &vs::Shader,
        fs: &fs::Shader,
        device: Arc<Device>,
        swapchain: Arc<Swapchain<Window>>,
        images: &[Arc<SwapchainImage<Window>>]
    ) -> (Arc<RenderPassAbstract + Send + Sync>, Pipelines, Vec<Arc<FramebufferAbstract + Send + Sync>>) {
        let render_pass = Arc::new(single_pass_renderpass!(device.clone(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: swapchain.format(),
                    samples: 1,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {}
            }
        ).unwrap()) as Arc<RenderPassAbstract + Send + Sync>;

        let dimensions = images[0].dimensions();
        let framebuffers = images.iter().map(|image| {
            Arc::new(
                Framebuffer::start(render_pass.clone())
                .add(image.clone()).unwrap()
                .build().unwrap()
            ) as Arc<FramebufferAbstract + Send + Sync>
        }).collect::<Vec<_>>();

        let dimensions = [dimensions[0] as f32, dimensions[1] as f32];
        let standard  = Render::pipeline(vs, fs, device.clone(), render_pass.clone(), dimensions, false);
        let wireframe = Render::pipeline(vs, fs, device.clone(), render_pass.clone(), dimensions, true);
        let pipelines = Pipelines { standard, wireframe };

        (render_pass, pipelines, framebuffers)
    }

    fn pipeline(
        vs: &vs::Shader,
        fs: &fs::Shader,
        device: Arc<Device>,
        render_pass: Arc<RenderPassAbstract + Send + Sync>,
        dimensions: [f32; 2],
        wireframe: bool)
        -> Arc<GraphicsPipeline<SingleBufferDefinition<Vertex>, Box<PipelineLayoutAbstract + Send + Sync>, Arc<RenderPassAbstract + Send + Sync>>>
    {
        let builder = GraphicsPipeline::start()
            .vertex_input_single_buffer()
            .vertex_shader(vs.main_entry_point(), ())
            .triangle_list()
            .viewports(iter::once(Viewport {
                origin:      [0.0, 0.0],
                depth_range: 0.0..1.0,
                dimensions
            }))
            .fragment_shader(fs.main_entry_point(), ())
            .render_pass(Subpass::from(render_pass.clone(), 0).unwrap());

        let builder = if wireframe {
            builder.polygon_mode_line()
        } else {
            builder
        };

        Arc::new(builder.build(device.clone()).unwrap())
    }

    fn recreate_swapchain(&mut self) {
        // Dont unwrap because MS Windows removes the window immediately on close before the process ends
        if let Some(resolution) = self.surface.window().get_inner_size() {
            let resolution: (u32, u32) = resolution.to_physical(self.surface.window().get_hidpi_factor()).into();
            let width = resolution.0;
            let height = resolution.1;
            match self.swapchain.recreate_with_dimension([width, height]) {
                Ok((new_swapchain, new_images)) => {
                    self.width = width;
                    self.height = height;
                    self.swapchain = new_swapchain.clone();

                    let (render_pass, pipelines, framebuffers) = Render::pipelines(&self.vs, &self.fs, self.device.clone(), new_swapchain, &new_images);
                    self.render_pass = render_pass;
                    self.pipelines = pipelines;
                    self.framebuffers = framebuffers;
                }
                Err(SwapchainCreationError::UnsupportedDimensions) => { } // Occurs when minimized on MS Windows as dimensions are (0, 0)
                Err(err) => { panic!("resize error: width={}, height={}, err={:?}", width, height, err) }
            }
        }
    }

    pub fn draw(&mut self, state: &State) {
        loop {
            if self.draw_inner(state) {
                return
            }
            self.recreate_swapchain();
        }
    }

    /// returns true if it succeeds
    /// returns false if the swapchain needs to be recreated
    fn draw_inner(&mut self, state: &State) -> bool {
        self.future.cleanup_finished();

        let (image_num, acquire_future) = match swapchain::acquire_next_image(self.swapchain.clone(), None) {
            Ok(r) => r,
            Err(AcquireError::OutOfDate) => { return false }
            Err(err) => panic!("{:?}", err)
        };

        let pipeline = match state.render_mode {
            RenderMode::Standard => self.pipelines.standard.clone(),
            RenderMode::Wireframe => self.pipelines.wireframe.clone(),
        };

        let command_buffer = AutoCommandBufferBuilder::primary_one_time_submit(self.device.clone(), self.queue.family()).unwrap()
            .begin_render_pass(self.framebuffers[image_num].clone(), false, vec![[0.0, 0.0, 0.0, 1.0].into()]).unwrap()
            .draw(pipeline, &DynamicState::none(), self.vertex_buffer_sphere.clone(), (), ()).unwrap()
            .end_render_pass().unwrap()
            .build().unwrap();

        let mut old_future = Box::new(sync::now(self.device.clone())) as Box<GpuFuture>; // TODO: Can I avoid making this dummy future?
        mem::swap(&mut self.future, &mut old_future);
        let future = old_future.join(acquire_future)
            .then_execute(self.queue.clone(), command_buffer).unwrap()
            .then_swapchain_present(self.queue.clone(), self.swapchain.clone(), image_num)
            .then_signal_fence_and_flush();

        self.future = match future {
            Ok(future) => {
                future.wait(None).unwrap(); // TODO: Pretty sure this is working around a bug in vulkano and shouldnt be here
                Box::new(future) as Box<_>
            }
            Err(FlushError::OutOfDate) => { return false }
            Err(e) => panic!("{:?}", e)
        };

        true
    }
}
