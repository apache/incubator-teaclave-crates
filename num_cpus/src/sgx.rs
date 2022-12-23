extern crate sgx_oc;
extern crate sgx_trts;

pub fn get_num_cpus() -> usize {
    unsafe {
        if let Ok(cpu_set) = sgx_oc::ocall::sched_getaffinity(0) {
            sgx_oc::CPU_COUNT(&cpu_set) as usize
        } else if let Ok(cpus) = sgx_oc::ocall::sysconf(libc::_SC_NPROCESSORS_ONLN) {
            cpus as usize
        } else {
            1
        }
    }
}

pub fn get_num_physical_cpus() -> usize {
    sgx_trts::capi::sgx_get_cpu_core_num() as usize
}
