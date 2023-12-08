terraform{

  required_providers {
    scaleway = {
      source = "scaleway/scaleway"
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

resource "scaleway_rdb_instance" "main" {
  name           = "rust_bdd"
  node_type      = "DB-DEV-S"
  engine         = "PostgreSQL-15"
  is_ha_cluster  = true
  disable_backup = true
  user_name      = "admin"
  password       = "S3cret_word"

}



provider "scaleway" {
  access_key      = "SCWFS6T9D3HM4AKVBQMD"
  secret_key      = "13047955-257b-443a-8f89-1c638111dff6"
  project_id	    = "f6f799d5-fee4-4e2c-8b3b-0adc3a6eac6d"
  zone            = "fr-par-1"
  region          = "fr-par"
}




