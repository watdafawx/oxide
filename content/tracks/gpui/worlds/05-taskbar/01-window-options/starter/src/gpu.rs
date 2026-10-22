//! The graphics card.

use nvml_wrapper::Nvml;
use nvml_wrapper::enum_wrappers::device::TemperatureSensor;

use crate::sensor::{Missing, Sensor};
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

    /// A GPU sensor with no card behind it.
    pub fn none_for_tests() -> Self {
        GpuSensor { gpu: NvidiaGpu::none(), kind: StatKind::GpuUsage }
    }
}

impl Sensor for GpuSensor {
    fn kind(&self) -> StatKind {
        self.kind
    }

    fn read(&mut self) -> Option<f64> {
        if self.kind == StatKind::GpuUsage { self.gpu.usage() } else { self.gpu.temperature() }
    }

    fn missing(&self) -> Option<Missing> {
        if self.gpu.is_present() { None } else { Some(Missing::NoDevice) }
    }
}

/// Who made a graphics card.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vendor {
    Nvidia,
    Amd,
    Intel,
}

/// One graphics card in this PC.
#[derive(Debug, Clone, PartialEq)]
pub struct GpuInfo {
    pub name: String,
    pub vendor: Vendor,
    pub vram_bytes: u64,
}

/// Kinetik's "Auto" choice: the NVIDIA card if there is one, else the most video memory.
pub fn pick_gpu(gpus: &[GpuInfo]) -> Option<usize> {
    if let Some(i) = gpus.iter().position(|g| g.vendor == Vendor::Nvidia) {
        return Some(i);
    }
    gpus.iter().enumerate().max_by_key(|(_, g)| g.vram_bytes).map(|(i, _)| i)
}
