provider "aws" {
  region = var.region
}

// VPC
resource "aws_vpc" "recred" {
  cidr_block = var.vpc_cidr_block
  instance_tenancy = "default"
  enable_dns_hostnames = true

  tags = {
    Name = "recred"
  }
}

resource "aws_internet_gateway" "recred" {
  vpc_id = aws_vpc.recred.id

  tags = {
    Name = "recred"
  }
}

resource "aws_subnet" "recred_public" {
  count             = var.subnet_count.public
  vpc_id            = aws_vpc.recred.id
  cidr_block        = var.public_subnet_cidr_blocks[count.index]
  availability_zone = data.aws_availability_zones.available.names[count.index]

  tags = {
    Name = "recred_public_${count.index}"
  }
}

resource "aws_subnet" "recred_private" {
  count             = var.subnet_count.private
  vpc_id            = aws_vpc.recred.id
  cidr_block        = var.private_subnet_cidr_blocks[count.index]
  availability_zone = data.aws_availability_zones.available.names[count.index]

  tags = {
    Name = "recred_private_${count.index}"
  }
}

resource "aws_route_table" "recred_public" {
  vpc_id = aws_vpc.recred.id

  // Since this is the public route table, it will need
  // access to the internet. So we are adding a route with
  // a destination of 0.0.0.0/0 and targeting the Internet 	 
  // Gateway "recred"
  route {
    cidr_block = "0.0.0.0/0"
    gateway_id = aws_internet_gateway.recred.id
  }

  tags = {
    Name = "recred_public"
  }
}

resource "aws_route_table_association" "recred_public" {
  count          = var.subnet_count.public
  route_table_id = aws_route_table.recred_public.id
  subnet_id      = 	aws_subnet.recred_public[count.index].id
}

resource "aws_route_table" "recred_private" {
  vpc_id = aws_vpc.recred.id

  tags = {
    Name = "recred_private"
  }
}

resource "aws_route_table_association" "recred_private" {
  count          = var.subnet_count.private
  route_table_id = aws_route_table.recred_private.id
  subnet_id      = 	aws_subnet.recred_private[count.index].id
}

resource "aws_security_group" "recred_service" {
  name        = "recred_service"
  description = "Security group for recred service"
  vpc_id      = aws_vpc.recred.id

  // The first requirement we need to meet is "EC2 instances should 
  // be accessible anywhere on the internet via HTTP." So we will 
  // create an inbound rule that allows all traffic through
  // TCP port 80.
  ingress {
    description = "Allow all traffic through HTTP"
    from_port   = "8080"
    to_port     = "8080"
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  // The second requirement we need to meet is "Only you should be 
  // "able to access the EC2 instances via SSH." So we will create an 
  // inbound rule that allows SSH traffic ONLY from your IP address
  ingress {
    description = "Allow SSH from my computer"
    from_port   = "22"
    to_port     = "22"
    protocol    = "tcp"
    // This is using the variable "my_ip"
    cidr_blocks = ["${var.my_ip}/32"]
  }

  // This outbound rule is allowing all outbound traffic
  // with the EC2 instances
  egress {
    description = "Allow all outbound traffic"
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Name = "recred_service"
  }
}

resource "aws_security_group" "recred_db" {
  // Basic details like the name and description of the SG
  name        = "recred_db"
  description = "Security group for recred databases"
  // We want the SG to be in the "tutorial_vpc" VPC
  vpc_id      = aws_vpc.recred.id

  // The third requirement was "RDS should be on a private subnet and 	
  // inaccessible via the internet." To accomplish that, we will 
  // not add any inbound or outbound rules for outside traffic.
  
  // The fourth and finally requirement was "Only the EC2 instances 
  // should be able to communicate with RDS." So we will create an
  // inbound rule that allows traffic from the EC2 security group
  // through TCP port 3306, which is the port that MySQL 
  // communicates through
  ingress {
    description     = "Allow postgres traffic from only recred service"
    from_port       = "5432"
    to_port         = "5432"
    protocol        = "tcp"
    security_groups = [aws_security_group.recred_service.id]
  }

  // Here we are tagging the SG with the name "tutorial_db_sg"
  tags = {
    Name = "recred_db"
  }
}

resource "aws_db_subnet_group" "recred_private" {
  // The name and description of the db subnet group
  name        = "recred_private"
  description = "DB subnet group"
  
  // Since the db subnet group requires 2 or more subnets, we are going to
  // loop through our private subnets in "tutorial_private_subnet" and
  // add them to this db subnet group
  subnet_ids  = [for subnet in aws_subnet.recred_private: subnet.id]

  tags = {
    Name = "recred_private"
  }
}


// RDS instance
resource "aws_db_parameter_group" "recred" {
  name   = "recred"
  family = "postgres14"

  parameter {
    name  = "log_connections"
    value = "1"
  }
}

resource "aws_db_instance" "recred" {
  identifier             = "recred"
  instance_class         = "db.t3.micro"
  storage_type           = "gp2"
  allocated_storage      = 200
  engine                 = "postgres"
  engine_version         = "14.4"
  username               = "recred"
  db_name                = "recred"
  password               = var.db_password
  db_subnet_group_name   = aws_db_subnet_group.recred_private.name
  vpc_security_group_ids = [aws_security_group.recred_db.id]
  parameter_group_name   = aws_db_parameter_group.recred.name
  skip_final_snapshot    = true
}


// Key file
resource "tls_private_key" "recred" {
  algorithm = "RSA"
  rsa_bits  = 4096
}

resource "aws_key_pair" "recred" {
  key_name = "recred"
  public_key = tls_private_key.recred.public_key_openssh
}

resource "local_file" "recred" {
  filename = "${aws_key_pair.recred.key_name}.pem"
  content = tls_private_key.recred.private_key_pem
}

// EC2 instance
resource "aws_instance" "recred" {
  count         = var.settings.recred_service.count
  ami           = data.aws_ami.amazon_linux.id
  instance_type = "t2.micro"
  subnet_id     = aws_subnet.recred_public[count.index].id
  key_name               = aws_key_pair.recred.key_name
  
  // The security groups of the EC2 instance. This takes a list, however we only
  // have 1 security group for the EC2 instances.
  vpc_security_group_ids = [aws_security_group.recred_service.id]

  // We are tagging the EC2 instance with the name "tutorial_db_" followed by
  // the count index
  tags = {
    Name = "recred_${count.index}"
  }
}

// Create an Elastic IP named "tutorial_web_eip" for each
// EC2 instance
resource "aws_eip" "recred" {
	// count is the number of Elastic IPs to create. It is
	// being set to the variable settings.web_app.count which
	// refers to the number of EC2 instances. We want an
	// Elastic IP for every EC2 instance
  count    = var.settings.recred_service.count

	// The EC2 instance. Since tutorial_web is a list of 
	// EC2 instances, we need to grab the instance by the 
	// count index. Since the count is set to 1, it is
	// going to grab the first and only EC2 instance
  instance = aws_instance.recred[count.index].id

	// We want the Elastic IP to be in the VPC
  vpc      = true

	// Here we are tagging the Elastic IP with the name
	// "tutorial_web_eip_" followed by the count index
  tags = {
    Name = "recred_eip_${count.index}"
  }
}

// resource "aws_ecr_repository" "recred" {
//   name                 = "recred"
//   image_tag_mutability = "MUTABLE"
// 
//   tags = {
//     project = "recred"
//   }
// }

// resource "aws_iam_role" "recred" {
//   name = "recred"
// 
//   assume_role_policy = <<EOF
// {
//   "Version": "2012-10-17",
//   "Statement": [
//     {
//       "Action": "sts:AssumeRole",
//       "Principal": {
//         "Service": "ec2.amazonaws.com"
//       },
//       "Effect": "Allow"
//     }
//   ]
// }
// EOF
// 
//   tags = {
//     project = "recred"
//   }
// }

// resource "aws_iam_instance_profile" "recred" {
//   name = "recred"
//   role = aws_iam_role.recred.name
// }
// 
// resource "aws_iam_role_policy" "recred" {
//   name = "recred"
//   role = aws_iam_role.recred.id
// 
//   policy = <<EOF
// {
//   "Version": "2012-10-17",
//   "Statement": [
//     {
//       "Action": [
//         "ecr:GetAuthorizationToken",
//         "ecr:BatchGetImage",
//         "ecr:GetDownloadUrlForLayer"
//       ],
//       "Effect": "Allow",
//       "Resource": "*"
//     }
//   ]
// }
// EOF
// }
