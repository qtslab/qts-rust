use opencl3::command_queue::CommandQueue;
use opencl3::context::Context;
use opencl3::device::{get_device_ids, Device, CL_DEVICE_TYPE_GPU};
use opencl3::kernel::Kernel;
use opencl3::memory::{Buffer, CL_MEM_READ_WRITE};
use opencl3::program::Program;
use opencl3::types::{cl_float, cl_int};
use std::ptr;

pub struct OpenCL {
    context: Context,
    queue: CommandQueue,
    kernel_measure: Kernel,
}

impl OpenCL {
    pub fn new() -> opencl3::Result<Self> {
        let platform = opencl3::platform::get_platforms()?
            .first()
            .cloned()
            .unwrap();
        let device_ids = get_device_ids(platform.id(), CL_DEVICE_TYPE_GPU)?;
        let device = Device::new(device_ids[0]);

        let context = Context::from_device(&device)?;
        let queue = CommandQueue::create_with_properties(&context, device.id(), 0, 0)?;

        let source = r#"
            __kernel void measure(__global float *qubits_alpha, __global int *solution, const int num_qubits) {
                int id = get_global_id(0);
                if (id < num_qubits) {
                    // 使用線性同餘生成器 (LCG) 來生成隨機數
                    uint seed = id;  // 使用 id 作為隨機種子
                    seed = (1103515245 * seed + 12345) & 0x7fffffff;
                    float r = (float)seed / 2147483648.0f;  // 正規化為 [0, 1) 的浮點數

                    solution[id] = (r < qubits_alpha[id] * qubits_alpha[id]) ? 1 : 0;
                }
            }
        "#;

        let program = Program::create_and_build_from_source(&context, source, "")
            .expect("Failed to build OpenCL program");
        let kernel_measure = Kernel::create(&program, "measure")?;

        Ok(Self {
            context,
            queue,
            kernel_measure,
        })
    }

    pub fn measure_qubits(
        &self,
        qubits_alpha: &[f32],
        solution: &mut [i32],
    ) -> opencl3::Result<()> {
        let num_qubits = qubits_alpha.len();

        let mut buffer_qubits_alpha = Buffer::<cl_float>::create(
            &self.context,
            CL_MEM_READ_WRITE,
            num_qubits,
            ptr::null_mut(),
        )?;
        let mut buffer_solution = Buffer::<cl_int>::create(
            &self.context,
            CL_MEM_READ_WRITE,
            num_qubits,
            ptr::null_mut(),
        )?;

        self.kernel_measure.set_arg(0, &buffer_qubits_alpha)?;
        self.kernel_measure.set_arg(1, &buffer_solution)?;
        self.kernel_measure.set_arg(2, &(num_qubits as cl_int))?;

        self.queue.enqueue_write_buffer(
            &mut buffer_qubits_alpha,
            0,
            qubits_alpha.len(),
            qubits_alpha,
            &[],
        )?;

        let global_work_size = [num_qubits];
        self.queue.enqueue_nd_range_kernel(
            self.kernel_measure.get(),
            1,
            ptr::null(),
            global_work_size.as_ptr(),
            ptr::null(),
            &[],
        )?;

        self.queue
            .enqueue_read_buffer(&mut buffer_solution, 0, solution.len(), solution, &[])?;

        Ok(())
    }
}
