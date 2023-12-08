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

module "resources" {
  source = "./resources"
}
module "storage" {
  source = "./storage"
}

module "ingress" {
  source = "./ingress"
}




provider "scaleway" {
  access_key      = "SCWFS6T9D3HM4AKVBQMD"
  secret_key      = "13047955-257b-443a-8f89-1c638111dff6"
  project_id	    = "f6f799d5-fee4-4e2c-8b3b-0adc3a6eac6d"
  zone            = "fr-par-1"
  region          = "fr-par"
}

terraform {
  backend "s3" {
    bucket                      = "jojotaro-s8"
    key                         = "terraform.tfstate"
    region                      = "fr-par"
    endpoints = { s3            = "https://s3.fr-par.scw.cloud" }
    access_key                  = "SCWFS6T9D3HM4AKVBQMD"
    secret_key                  = "13047955-257b-443a-8f89-1c638111dff6"
    skip_credentials_validation = true
    skip_region_validation      = true
    skip_requesting_account_id  = true
  }
}








