// Production-grade ML runtime example that includes proper error handling, logging, configuration, model versioning and performance optimizations
use anyhow::{Context, Result};
use log::{info, warn, error};
use serde::Deserialize;
use std::path::Path;
use tch::{nn, Device, Tensor, Kind};

// need a struct for this, structs group related pieces of data together
#[derive(Debug, Deseriaze)]
struct Config {
  model_path: String, // defines field: model_path with a String type
  input_size: usize, // input_size field using usize an unsigned integer type, used for sizes and collections
  output_size: usize, //  stores a non negative size or count
  device: Option<String>, // defines and optional string field
  //None,  This is useful when a device setting is not required or has a default
  log_level: Option<String>, // another optional string
  
}

struct MLRuntime {
  model: nn::Sequential,
  device: Device,
  input_size: usize,
}

impl MLRuntime {
  // initialize the ML runtime with proper error handling
  pub fn new(config: &Config) -> Result<Self> {
    // Set up logging
    let log_level =  config.log_level.as_deref().unwrap_or("info");
    env_logger::Builder::from_default_env().filter_level(log::LevelFilter::from_str(log_level)?)
    .init();

    info!("Initializing ML runtime with config: {:?}", config);

    // Determine device
    let device = match config.device.as_deref() {
      Some("cuda") => Device::cuda_if_available(),
      Some("cpu") => Device::Cpu,
      _ => Device::cuda_if_available(),
    };

    if device.is_cuda() {
      info!("Using CUDA device")
    } else {
      warn!("Using CPU device - consider using CUDA for better performance");
    }

    // Load model
    let vs = nn::VarStore::new(device);
    let model = Self::load_model(&vs, config);

    Ok(Self {
      model,
      device,
      input_size: config.input_size,
    })
    
  }
}
