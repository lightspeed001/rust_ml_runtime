# /basic/prod.md
## Production grade ML Runtime in Rust

### Key Considerations :pen:

1. **High Availability**: 3+ replicas with rolling updates.
2. **Auto-scaling**: Both horizontal and vertical.
3. **Monitoring**: Prometheus metrics + Cloud Monitoring.
4. **Security**: Non-root execution, IAM roles, network policies.
5. **Cost Optimization**: Preemptible nodes, resource requests.
6. **Reliability**: Liveness/readiness probes, health checks.
7. **Performance**: GPU support, native CPU optimizations.
8. **CI/CD**: Cloud Build pipeline with testing.
9. **TLS**: Managed certificates with HTTPS redirection.
10. **Storage**: SSD-backed PVC for models.

This configuration is production-ready for Google Kubernetes Engine and follows Google Cloud best practices.

