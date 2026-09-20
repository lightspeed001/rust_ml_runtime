# Create service account for ML runtime
gcloud iam service-accounts create ml-runtime-sa \
  --display-name="ML Runtime Service Account"

# Grant necessary permissions
gcloud projects add-iam-policy-binding your-project \
  --member="serviceAccount:ml-runtime-sa@your-project.iam.gserviceaccount.com" \
  --role="roles/storage.objectViewer"

# Use the service account in deployment
kubectl patch deployment ml-runtime-prod \
  -p '{"spec":{"template":{"spec":{"serviceAccountName":"ml-runtime-sa"}}}}'

