terraform {
  required_providers {
    scaleway = {
      source = "scaleway/scaleway"
    }
  }
  required_version = ">= 0.13"
  backend "s3" {
    bucket                      = "jojotaro-s3"
    key                         = "tf.tfstate"
    region                      = "fr-par"
    endpoints = { s3            = "https://s3.fr-par.scw.cloud" }
    access_key                  = "SCWFS6T9D3HM4AKVBQMD"
    secret_key                  = "13047955-257b-443a-8f89-1c638111dff6"
    skip_credentials_validation = true
    skip_region_validation      = true
    skip_requesting_account_id  = true
  }
}