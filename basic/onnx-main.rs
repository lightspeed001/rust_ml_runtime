use tract_onnx::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>>{
  // Load ONNX model
  let model = tract_onnx::onnx()
  .model_for_path("model.onnx")?
  .into_optimized()?
  .into_runnable()?;

  // create input tensor
  let input = Tensor::zero::<f32>(&[1, 2, 224, 224])?;

  // Run inference
  let outputs = model.run(tvec!(input.into()))?;

  println!("Outputs: {:?}", outputs);

  Ok(())
}
