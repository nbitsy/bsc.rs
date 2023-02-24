pub enum EThreadType {
    /// IO密集型
    IOIntensive = 0,
    /// CPU密集型
    CPUIntensive = 1,
    /// 默认型 cpu*2+1
    DefaultIntensive = 2,
}

/// 取得当前系统中识别的CPU核心数
pub fn get() -> usize {
    num_cpus::get()
}

/// 默认的阻塞系数
static mut k_blocking_coefficient: f32 = 0.8;

/// 设置阻塞系数
pub fn set_blocking_coefficient(bc: f32) {
    if bc >= 1.0 || bc < 0.0 {
        return;
    }

    unsafe {
        k_blocking_coefficient = bc
    }
}

/// 通过计算类型取得合理的线程数
pub fn get_cpu_num_by_type(t: EThreadType) -> usize {
    return match t {
        EThreadType::IOIntensive => unsafe {
            (get() as f32 / (1.0 - k_blocking_coefficient)) as usize
        }
        EThreadType::CPUIntensive => {
            get() + 1
        }
        EThreadType::DefaultIntensive => {
            get() * 2 + 1
        }
    };
}
