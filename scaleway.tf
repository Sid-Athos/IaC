  terraform {
    required_providers {
      scaleway = {
        source = "scaleway/scaleway"
      }
    }
    required_version = ">= 0.13"
  }
  provider "scaleway" {
    access_key      = "dc76b5b5-1d86-4201-93a3-589d06a61d82"
    secret_key      = "13047955-257b-443a-8f89-1c638111dff6"
    project_id	    = "<SCW_DEFAULT_PROJECT_ID>"
    zone            = "fr-par-1"
    region          = "fr-par"
  }
  resource "scaleway_instance_ip" "public_ip" {}
  resource "scaleway_instance_volume" "data" {
    size_in_gb = 30
    type = "l_ssd"
  }
  resource "scaleway_instance_server" "my-instance" {
    type  = "DEV1-L"
    image = "ubuntu_focal"

    tags = [ "terraform instance", "my-instance" ]

    ip_id = scaleway_instance_ip.public_ip.id

    additional_volume_ids = [ scaleway_instance_volume.data.id ]

    root_volume {
      # The local storage of a DEV1-L Instance is 80 GB, subtract 30 GB from the additional l_ssd volume, then the root volume needs to be 50 GB.
      size_in_gb = 50
    }
  }
