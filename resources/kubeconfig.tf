resource "local_file" "kubeconfig" {
  filename        = "${path.module}/kubeconfig"
  file_permission = "0600"
  content         = scaleway_k8s_cluster.cluclu.kubeconfig[0].config_file
}