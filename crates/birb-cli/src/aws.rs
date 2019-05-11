use crate::{run_str_in_bash, Subcommand};

/// Deploy different applications / services to aws
#[derive(Debug, StructOpt)]
pub enum Aws {
    /// Deploy all infrastructure
    #[structopt(name = "all")]
    All(AwsAll),
    /// Deploy the Edgar worker
    #[structopt(name = "edgar")]
    Edgar(AwsEdgar),
    /// Deploy the Edgar worker
    #[structopt(name = "bastion")]
    Bastion(AwsBastion),
    /// Deploy the Database
    #[structopt(name = "rds")]
    RDS(AwsRDS),
    /// Deploys whatever change is held by the "plan" file
    #[structopt(name = "plan")]
    Plan(TfPlan),
    /// Get output for local usage
    #[structopt(name = "output")]
    Output(AwsOutput),
}

impl Subcommand for Aws {
    fn run(&self) -> Result<(), failure::Error> {
        match self {
            Aws::All(all) => all.run(),
            Aws::Edgar(edgar) => edgar.run(),
            Aws::Bastion(bastion) => bastion.run(),
            Aws::RDS(rds) => rds.run(),
            Aws::Plan(tf_plan) => tf_plan.run(),
            Aws::Output(output) => output.run(),
        }
    }
}

/// Deploy eveeeerything
#[derive(Debug, StructOpt)]
pub enum AwsAll {
    #[structopt(name = "up")]
    Up,
    #[structopt(name = "down")]
    Down,
}

/// Deploy the Edgar Worker
#[derive(Debug, StructOpt)]
pub enum AwsEdgar {
    #[structopt(name = "up")]
    Up,
    #[structopt(name = "down")]
    Down,
}

/// Deploy the Bastion
#[derive(Debug, StructOpt)]
pub enum AwsBastion {
    #[structopt(name = "up")]
    Up,
    // TODO add down
}

/// Get the outputs
#[derive(Debug, StructOpt)]
pub struct AwsOutput {}

/// Deploy the Terraform Plan
#[derive(Debug, StructOpt)]
pub enum TfPlan {
    #[structopt(name = "up")]
    Up,
}

/// Destroys the Terraform Infrastructure (ex. RDS)
#[derive(Debug, StructOpt)]
pub enum AwsRDS {
    #[structopt(name = "up")]
    Up,
}

impl Subcommand for AwsAll {
    fn run(&self) -> Result<(), failure::Error> {
        match self {
            AwsAll::Up => {
                let _plan = run_str_in_bash(
                    "
                    bb plan all
                ",
                )?;

                let _result = run_str_in_bash(
                    "
                    bb aws plan up
                ",
                )?;
            }
            AwsAll::Down => {
                let _result = run_str_in_bash(
                    "
                    terraform destroy -var-file=terraform/production.secret.tfvars \
                        -auto-approve \
                        terraform/
                ",
                )?;
            }
        }
        Ok(())
    }
}

impl Subcommand for TfPlan {
    fn run(&self) -> Result<(), failure::Error> {
        match self {
            TfPlan::Up => {
                run_str_in_bash("terraform apply \"plan\" && rm -rf plan")?;
            }
        }
        Ok(())
    }
}

impl Subcommand for AwsEdgar {
    fn run(&self) -> Result<(), failure::Error> {
        match self {
            AwsEdgar::Up => {
                // Not currently worrying about whether or not the deploy was successful
                let _plan = run_str_in_bash(
                    "
                    terraform plan -var-file=terraform/production.secret.tfvars \
                       -out=plan \
                       -auto-approve \
                       -target=aws_launch_configuration.ecs-launch-configuration \
                       -target=aws_autoscaling_group.ecs-autoscaling-group \
                       -target=aws_ecs_cluster.birb-edgar-cluster \
                       -target=aws_ecs_task_definition.birb-edgar-task \
                       -target=aws_ecs_service.birb-edgar-service \
                       -target=aws_iam_role.ecs-instance-role \
                       -target=aws_iam_role_policy_attachment.ecs-instance-role-attachment \
                       -target=aws_iam_instance_profile.ecs-instance-profile \
                       -target=aws_iam_role.ecs-service-role \
                       -target=aws_iam_role_policy_attachment.ecs-service-role-attachment \
                       -target=aws_ecr_repository.birb_edgar_worker_repo \
                       terraform/
                ",
                )?;

                let _result = run_str_in_bash(
                    "
                    bb aws plan up
                ",
                )?;
            }
            AwsEdgar::Down => {
                let _reuslt = run_str_in_bash(
                    "
                    terraform destroy -var-file=terraform/production.secret.tfvars \
                       -auto-approve \
                       -target=aws_launch_configuration.ecs-launch-configuration \
                       -target=aws_autoscaling_group.ecs-autoscaling-group \
                       -target=aws_ecs_cluster.birb-edgar-cluster \
                       -target=aws_ecs_task_definition.birb-edgar-task \
                       -target=aws_ecs_service.birb-edgar-service \
                       -target=aws_iam_role.ecs-instance-role \
                       -target=aws_iam_role_policy_attachment.ecs-instance-role-attachment \
                       -target=aws_iam_instance_profile.ecs-instance-profile \
                       -target=aws_iam_role.ecs-service-role \
                       -target=aws_iam_role_policy_attachment.ecs-service-role-attachment \
                       -target=aws_ecr_repository.birb_edgar_worker_repo \
                       terraform/
                ",
                )?;
            }
        }

        Ok(())
    }
}

impl Subcommand for AwsBastion {
    fn run(&self) -> Result<(), failure::Error> {
        match self {
            AwsBastion::Up => {
                // Not currently worrying about whether or not the deploy was successful
                let _plan = run_str_in_bash(
                    "
                bb plan bastion
            ",
                )?;

                let _result = run_str_in_bash(
                    "
                bb deploy plan
            ",
                )?;
            }
        }

        Ok(())
    }
}

impl Subcommand for AwsOutput {
    fn run(&self) -> Result<(), failure::Error> {
        // Not currently worrying about whether or not the deploy was successful
        let _plan = run_str_in_bash(
            "
            bb plan output
        ",
        )?;

        let _result = run_str_in_bash(
            "
            bb deploy plan
        ",
        )?;
        Ok(())
    }
}

impl Subcommand for AwsRDS {
    fn run(&self) -> Result<(), failure::Error> {
        match self {
            AwsRDS::Up => {
                // Not currently worrying about whether or not the deploy was successful
                let _plan = run_str_in_bash(
                    "
            bb plan rds
        ",
                )?;

                let _result = run_str_in_bash(
                    "
            bb deploy plan
        ",
                )?;
            }
        }

        Ok(())
    }
}
