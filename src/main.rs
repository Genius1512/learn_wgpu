use learn_wgpu::run;

fn main() {
    env_logger::init(); // Init logging
    pollster::block_on(run()); // Run the application
}
