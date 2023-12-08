terraform {
  required_providers {
    scaleway = {
      source = "scaleway/scaleway"
    }
  }
  required_version = ">= 0.13"

}
resource "scaleway_instance_ip" "public_ip" {}
resource "scaleway_instance_volume" "data" {
  size_in_gb = 30
  type = "l_ssd"
}
resource "scaleway_object_bucket" "jojotaro-s4" {
  name = "jojotaro-s4"
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

resource "scaleway_vpc_private_network" "hedy" {}

resource "scaleway_k8s_cluster" "cluclu" {
  name    = "clu"
  version = "1.24.3"
  cni     = "cilium"
  private_network_id = scaleway_vpc_private_network.hedy.id
  delete_additional_resources = false
}

resource "scaleway_k8s_pool" "poopoo" {
  cluster_id = scaleway_k8s_cluster.cluclu.id
  name       = "poopoo"
  node_type  = "DEV1-M"
  size       = 1
}