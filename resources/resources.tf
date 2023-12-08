terraform {
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
resource "scaleway_instance_ip" "public_ip" {}



resource "scaleway_instance_volume" "data" {
  size_in_gb = 30
  type = "l_ssd"
}
resource "scaleway_object_bucket" "jojotaro-s8" {
  name = "jojotaro-s8"
  tags = {
    project = "esgi-iac"
  }
}
resource "scaleway_instance_server" "server" {
  type  = "DEV1-L"
  image = "ubuntu_focal"

  tags = ["terraform instance", "server"]

  ip_id = scaleway_instance_ip.public_ip.id

  additional_volume_ids = [scaleway_instance_volume.data.id]

  root_volume {
    # The local storage of a DEV1-L Instance is 80 GB, subtract 30 GB from the additional l_ssd volume, then the root volume needs to be 50 GB.
    size_in_gb = 50
  }
}

resource "scaleway_vpc_public_gateway" "pg" {
  type = "VPC-GW-S"
}

resource "scaleway_vpc_public_gateway_dhcp" "dhcp" {
  subnet = "192.168.1.0/24"
}

resource "scaleway_vpc_private_network" "pn" {
  name = "network"
}
output "privete" {
  value = scaleway_vpc_private_network.pn
}
resource "scaleway_vpc_gateway_network" "gn" {
  gateway_id = scaleway_vpc_public_gateway.pg.id
  private_network_id = scaleway_vpc_private_network.pn.id
  dhcp_id = scaleway_vpc_public_gateway_dhcp.dhcp.id
  cleanup_dhcp = true
}

resource "scaleway_vpc_public_gateway_pat_rule" "main" {
  gateway_id = scaleway_vpc_public_gateway.pg.id
  private_ip = scaleway_vpc_public_gateway_dhcp.dhcp.address
  private_port = 42
  public_port = 42
  protocol = "both"
  depends_on = [scaleway_vpc_gateway_network.gn, scaleway_vpc_private_network.pn]
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
resource "kubernetes_deployment" "deployment" {
  metadata {
    name   = "deployment"
    labels = {
      test = "RustApp"
    }
  }

  spec {
    selector {
      match_labels = {
        test = "RustApp"
      }
    }

    template {
      metadata {
        labels = {
          test = "RustApp"
        }
      }

      spec {
        container {
          image = "sabennaceur135/iacesgi:latest"
          name  = "api"
          port {
            container_port = 80
          }

          resources {
            limits = {
              cpu    = "0.5"
              memory = "512Mi"
            }
            requests = {
              cpu    = "250m"
              memory = "50Mi"
            }
          }

          liveness_probe {
            http_get {
              path = "/"
              port = 80
            }

            initial_delay_seconds = 3
            period_seconds        = 3
          }
        }
      }
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
}







