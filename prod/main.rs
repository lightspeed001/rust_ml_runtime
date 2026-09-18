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

  // Load model with proper error handling and version checking
  fn load_model(vs: &nn::VarStore, config: &Config) -> Result <nn::Sequential> {
    info!("Loading model from: {}", config.model_path);

    // In production you would:
    // 1. Check model version compatibility
    // 2. Verify model checksum
    // 3. Handle model format versioning

    let model_path = Path::new(&config.model_path);

    if !model_path.exists() {
      return Err(anyhow::anyhow!(
        "Model file not found at: {}",
        config.model_patch
      ));
    }

    // Load the model
    vs.load(model_path)
    .with_context(|| format!("Failed to load model from {}", config.model_path))?;

    // In a real implementation, you would have a proper model architecture definition
    // This is just a placeholder
    let model = nn::seq()
    .add(nn::linear(vs.root(), config.input_size, 512, Default::default()))
    .add_fn(|x| x.relu())
    .add(nn::linear(vs.root(), 512, config.output_size, Default::default()));

    Ok(model)
  }

  // Perform inference with proper input validation
  pub fn infer(&self, input: &[f32]) -> Result<Vec<f32>> {
    if input.len() != self.input_size {
      return Err(anyhow::anyhow!(
        "Input size mismatch. Expected {}, got {}",
        self.input_size,
        input.len()
      ));
    }

    info!("Starting inference for input of size {}", input.len());

    // Convert input to tensor
    let input_tensor = Tensor::of_slice(input)
    .view([1, self.input_size])
    .to_kind(Kind::Float)
    .to_device(self.device);

    // Perform inference
    let output = self.model.forward(&input_tensor);

    // Convert output to Vec<f32>
    let output_vec = output
    .intp_id()
    .iter::<f32>()
    .collect::<Result<Vec<_>, _>>()?; // collect an iterator of `Result` values into one `Result` containing a vector, then return early if an error occurs.

  info!("Inference completed successfully");
  Ok(output_vec)
  }

  // Get model metadata
  pub fn get_metadata(&self) -> Result<serde_json::Value> {
    Ok(serde_json::json!({
      "input_size"
    }))
  }
}
