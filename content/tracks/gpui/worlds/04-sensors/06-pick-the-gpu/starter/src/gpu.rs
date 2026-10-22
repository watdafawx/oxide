//! The graphics card.

use nvml_wrapper::Nvml;
use nvml_wrapper::enum_wrappers::device::TemperatureSensor;

use crate::sensor::Sensor;
use crate::stat::StatKind;

/// An NVIDIA card through the driver's nvml.dll, or nothing when there is none.
pub struct NvidiaGpu {
    nvml: Option<Nvml>,
}

impl NvidiaGpu {
    /// Loads nvml.dll. On a PC without an NVIDIA driver this is a GPU with no readings.
    pub fn new() -> Self {
        NvidiaGpu { nvml: Nvml::init().ok() }
    }

    /// A GPU with no NVIDIA card behind it, for tests.
    pub fn none() -> Self {
        NvidiaGpu { nvml: None }
    }

    pub fn is_present(&self) -> bool {
        self.nvml.is_some()
    }

    /// The first card's usage, 0 to 100.
    pub fn usage(&self) -> Option<f64> {
        let nvml = self.nvml.as_ref()?;
        let device = nvml.device_by_index(0).ok()?;
        Some(device.utilization_rates().ok()?.gpu as f64)
    }

    /// The first card's temperature in °C.
    pub fn temperature(&self) -> Option<f64> {
        let device = self.nvml.as_ref()?.device_by_index(0).ok()?;
        Some(device.temperature(TemperatureSensor::Gpu).ok()? as f64)
    }
}

impl Default for NvidiaGpu {
    fn default() -> Self {
        NvidiaGpu::new()
    }
}

/// GPU usage or GPU temperature from an NvidiaGpu.
pub struct GpuSensor {
    gpu: NvidiaGpu,
    kind: StatKind,
}

impl GpuSensor {
    pub fn usage() -> Self {
        GpuSensor { gpu: NvidiaGpu::new(), kind: StatKind::GpuUsage }
    }

    pub fn temperature() -> Self {
        GpuSensor { gpu: NvidiaGpu::new(), kind: StatKind::GpuTemp }
    }
}

impl Sensor for GpuSensor {
    fn kind(&self) -> StatKind {
        self.kind
    }

    fn read(&mut self) -> Option<f64> {
        if self.kind == StatKind::GpuUsage { self.gpu.usage() } else { self.gpu.temperature() }
    }
}
