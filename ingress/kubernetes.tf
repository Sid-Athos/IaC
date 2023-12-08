/**terraform {
  required_providers {
    scaleway = {
      source = "scaleway/scaleway"
    }
    kubernetes = {
      source  = "hashicorp/kubernetes"
    }
  }
  required_version = ">= 0.13"

}

resource "null_resource" "kubeconfig" {
  depends_on = [scaleway_k8s_pool.poopoo] # at least one pool here
  triggers = {
    host                   = scaleway_k8s_cluster.cluclu.kubeconfig[0].host
    token                  = scaleway_k8s_cluster.cluclu.kubeconfig[0].token
    cluster_ca_certificate = scaleway_k8s_cluster.cluclu.kubeconfig[0].cluster_ca_certificate
  }
}

provider "kubernetes" {
  host  = null_resource.kubeconfig.triggers.host
  token = null_resource.kubeconfig.triggers.token
  cluster_ca_certificate = base64decode(
    null_resource.kubeconfig.triggers.cluster_ca_certificate
  )
}

resource "scaleway_k8s_cluster" "cluclu" {
  name    = "clu"
  version = "1.24.3"
  cni     = "cilium"
  private_network_id = scaleway_vpc_private_network.pn.id
  delete_additional_resources = false
}

resource "scaleway_k8s_pool" "poopoo" {
  cluster_id = scaleway_k8s_cluster.cluclu.id
  name       = "poopoo"
  node_type  = "DEV1-M"
  size       = 1
}


resource "kubernetes_namespace" "namespace" {
  metadata {
    name = "nginx"
  }
}
resource "kubernetes_ingress" "ingress" {
  metadata {
    name = "ingress"
  }

  spec {
    backend {
      service_name = "rust-app"
      service_port = 8080
    }

    rule {
      http {
        path {
          backend {
            service_name = "rust-app"
            service_port = 7590
          }

          path = "/api/*"
        }
      }
    }

    tls {
      secret_name = "tls-secret"
    }
  }
}

resource "kubernetes_service" "service" {
  metadata {
    name = "rust-app"
  }
  spec {
    selector = {
      app = kubernetes_pod.pod.metadata.0.labels.app
    }
    session_affinity = "ClientIP"
    port {
      port        = 8080
      target_port = 80
    }
    type = "NodePort"
  }
}

resource "kubernetes_pod" "pod" {
  metadata {
    name = "terraform-pod"
    labels = {
      app = "rust"
    }
  }

  spec {
    container {
      image = "nginx:1.7.9"
      name  = "nginx"

      port {
        container_port = 8080
      }
    }
  }
}*/




