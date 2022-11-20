variable "my_ip" {
  description = "my IP address"
  default = "38.242.177.249"
}

variable "region" {
  description = "AWS region"
  default     = "eu-west-3"
}

// CIDR block for the VPC
variable "vpc_cidr_block" {
  description = "CIDR block for VPC"
  type        = string
  default     = "10.0.0.0/16"
}

// number of public and private subnets
variable "subnet_count" {
  description = "Number of subnets"
  type        = map(number)
  default = {
    public  = 1,
    private = 2
  }
}

// This variable contains the CIDR blocks for
// the public subnet. I have only included 4
// for this tutorial, but if you need more you 
// would add them here
variable "public_subnet_cidr_blocks" {
  description = "Available CIDR blocks for public subnets"
  type        = list(string)
  default = [
    "10.0.1.0/24",
    "10.0.2.0/24",
    "10.0.3.0/24",
    "10.0.4.0/24"
  ]
}

// This variable contains the CIDR blocks for
// the public subnet. I have only included 4 
// for this tutorial, but if you need more you
// would add them here
variable "private_subnet_cidr_blocks" {
  description = "Available CIDR blocks for private subnets"
  type        = list(string)
  default = [
    "10.0.101.0/24",
    "10.0.102.0/24",
    "10.0.103.0/24",
    "10.0.104.0/24",
  ]
}

variable "db_password" {
  description = "RDS root user password"
  default = "HkVdigcry0T4CGf00Xt1g3g1ZlZkORgHxRrxQBt7M8y0yQ54EQ"
  sensitive   = true
}

variable "settings" {
  description = "Configuration settings"
  type        = map(any)
  default = {
    "recred_service" = {
      count         = 1          // the number of EC2 instances
    }
  }
}
