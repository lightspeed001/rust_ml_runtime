use tch::{nn, Device, Tensor, Kind};

fn main -> Result<(), Box<dyn std::error::Error>>{
  //set up device (CPU or Cuda if available)
  let device = Device::cuda_if_available();

  // Create a simple model (in practice, you'd load a pre-trained model)
  let vs = nn::VarStore::new(device);
  let model = nn::seq().add(nn::linear(&vs.root(), 10, 5, Default::default())).add_fn(|x| x.relu());

  // Load a pre-trained model (example path - replace with your actual model path)
  // vs.load("model.pt")?;

  //Create some dummy input data
  let input = Tensor::randn(&[1, 10], (Kind::Float, device));

  // Perform inference
  let output = model.forward(&input);

  println!("Input shape: {:?}", input.size());
  println!("Output shape: {:?}", output.size());
  println!("Output values: {:?}",output);

  Ok(())
}
