# /basic/basic.md
## Basic ML Runtime in Rust

```mermaid
graph TD
    subgraph "Basic Rust ML Runtime"
        A[Rust Application] -->|Loads| B[PyTorch Model]
        A -->|Creates| C[Random Input Tensor]
        A -->|Performs| D[Inference]
        D -->|Outputs| E[Prints Results]
    end

    style A fill:#BA673D,stroke:#333
    style B fill:#3D8ABA,stroke:#333
    style C fill:#3D8ABA,stroke:#333
    style D fill:#3D8ABA,stroke:#333
    style E fill:#3D8ABA,stroke:#333
```

### Key Considerations :pen:

*  __Device Handling__: Automatically uses CUDA if available.
*  __Model Definition__: Simple sequential model with one linear layer.
*  __Inference__: Forward pass through the model
*  __Tensor Operations__: Basic tensor creation and manipulation.


### How to use this :rescue_worker_helmet:

-  Replace the model definition with your actual model architecture
-  Uncomment and use the **vs.load** line to load your pre-trained weights.
-  Adjust input dimensions to match your model's expected input.



