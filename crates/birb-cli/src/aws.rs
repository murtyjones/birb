use crate::{run_str_in_bash, Subcommand};

/// Deploy different applications / services to aws
#[derive(Debug, StructOpt)]
pub enum Aws {
    /// Deploy all infrastructure
    #[structopt(name = "all")]
    All(AwsAll),
    /// Deploy the Server
    #[structopt(name = "server")]
    Server(AwsServer),
    /// Deploy the Edgar worker
    #[structopt(name = "edgar")]
    Edgar(AwsEdgar),
    /// Deploy the bastion
    #[structopt(name = "bastion")]
    Bastion(AwsBastion),
    /// Deploy the Database
    #[structopt(name = "rds")]
    RDS(AwsRDS),
    /// Commands related to different S3 buckets we have
    #[structopt(name = "s3")]
    S3(AwsS3),
    /// Deploy whatever change is held by the "plan" file
    #[structopt(name = "plan")]
    Plan(TfPlan),
    /// Get output for local usage
    #[structopt(name = "output")]
    Output(AwsOutput),
    /// Deploy stateful resources (e.g. DB, ECR)
    #[structopt(name = "stateful")]
    Stateful(AwsStateful),
    /// Manage stateless resources (e.g. ECS)
    #[structopt(name = "stateless")]
    Stateless(AwsStateless),
}

impl Subcommand for Aws {
    fn run(&self) -> Result<(), failure::Error> {
        match self {
            Aws::All(all) => all.run(),
            Aws::Server(server) => server.run(),
            Aws::Edgar(edgar) => edgar.run(),
            Aws::Bastion(bastion) => bastion.run(),
            Aws::RDS(rds) => rds.run(),
            Aws::Plan(tf_plan) => tf_plan.run(),
            Aws::Output(output) => output.run(),
            Aws::Stateful(stateful) => stateful.run(),
            Aws::Stateless(stateless) => stateless.run(),
            Aws::S3(s3) => s3.run(),
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

/// Manage the Server infrastructure
#[derive(Debug, StructOpt)]
pub enum AwsServer {
    #[structopt(name = "up")]
    Up,
    #[structopt(name = "down")]
    Down,
}

/// Manage the Edgar Worker infrastructure
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
    #[structopt(name = "down")]
    Down,
    #[structopt(name = "redeploy")]
    ReDeploy,
}

/// Get the outputs
#[derive(Debug, StructOpt)]
pub struct AwsOutput {}

/// Stateful AWS resources that shouldn't be easy to delete
#[derive(Debug, StructOpt)]
pub enum AwsStateful {
    #[structopt(name = "up")]
    Up,
}

/// Stateless AWS resources that should be easy to create/delete
#[derive(Debug, StructOpt)]
pub enum AwsStateless {
    #[structopt(name = "up")]
    Up,
    #[structopt(name = "down")]
    Down,
}

/// Destroy and redeploy the bastion, used for example when you add a new SSH key
#[derive(Debug, StructOpt)]
pub enum ReDeploy {
    #[structopt(name = "up")]
    Up,
    #[structopt(name = "down")]
    Down,
}

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

/// Manage AWS infrastructure
#[derive(Debug, StructOpt)]
pub enum AwsS3 {
    #[structopt(name = "upload")]
    Upload(S3Buckets),
    #[structopt(name = "book")]
    Book(Book),
}

/// Which AWS buckets can be uploaded to
#[derive(Debug, StructOpt)]
pub enum S3Buckets {
    #[structopt(name = "edgar-indexes")]
    EdgarIndexes,
}

#[derive(Debug, StructOpt)]
pub enum Book {
  #[structopt(name = "up")]
  Up,
}

pub struct EdgarIndexes {}

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
                    AWS_SDK_LOAD_CONFIG=1   AWS_PROFILE=birb terraform destroy -var-file=terraform/production.secret.tfvars \
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
                run_str_in_bash(
                    "\
                     AWS_SDK_LOAD_CONFIG=1   AWS_PROFILE=birb terraform apply \"plan\" && rm -rf plan\
                     ",
                )?;
            }
        }
        Ok(())
    }
}

impl Subcommand for AwsServer {
    fn run(&self) -> Result<(), failure::Error> {
        match self {
            AwsServer::Up => {
                // Not currently worrying about whether or not the deploy was successful
                let _plan = run_str_in_bash(
                    "
                    AWS_SDK_LOAD_CONFIG=1   AWS_PROFILE=birb terraform plan -var-file=terraform/production.secret.tfvars \
                       -out=plan \
                       -target=aws_alb.server_load_balancer \
                       -target=aws_alb_target_group.server_target_group \
                       -target=aws_alb_listener.server_lb_listener \
                       -target=aws_alb_listener.redirect_to_ssl \
                           -target=aws_security_group.lb \
                       -target=aws_ecs_cluster.server_cluster \
                       -target=aws_ecs_service.server_service \
                       -target=aws_ecs_task_definition.server_task \
                       -target=aws_appautoscaling_target.server_autoscaling_target \
                       -target=aws_appautoscaling_policy.server_scale_up \
                       -target=aws_appautoscaling_policy.server_scale_down \
                       -target=aws_cloudwatch_metric_alarm.server_cpu_utilization_high \
                       -target=aws_cloudwatch_metric_alarm.server_cpu_utilization_low \
                       -target=aws_cloudwatch_log_group.server_log_group \
                       -target=aws_cloudwatch_log_stream.server_log_stream \
                       -target=aws_iam_role.autoscale_role \
                       -target=aws_iam_policy.autoscale_policy \
                       -target=aws_iam_role_policy_attachment.server_autoscale_attachment \
                       -target=aws_iam_role.task_execution_role \
                       -target=aws_iam_policy.task_execution_policy \
                       -target=aws_iam_role_policy_attachment.server_execution_attachment \
                       -target=aws_route53_record.birb \
                       -target=aws_security_group.lb \
                       -target=aws_security_group.ecs_tasks \
                       terraform/
                ",
                )?;

                let _result = run_str_in_bash(
                    "
                    bb aws plan up
                ",
                )?;
            }
            AwsServer::Down => {
                let _reuslt = run_str_in_bash(
                    "
                    AWS_SDK_LOAD_CONFIG=1   AWS_PROFILE=birb terraform destroy -var-file=terraform/production.secret.tfvars \
                       -auto-approve \
                       -target=aws_alb.server_load_balancer \
                       -target=aws_alb_target_group.server_target_group \
                       -target=aws_alb_listener.server_lb_listener \
                       -target=aws_alb_listener.redirect_to_ssl \
                           -target=aws_security_group.lb \
                       -target=aws_ecs_cluster.server_cluster \
                       -target=aws_ecs_service.server_service \
                       -target=aws_ecs_task_definition.server_task \
                       -target=aws_appautoscaling_target.server_autoscaling_target \
                       -target=aws_appautoscaling_policy.server_scale_up \
                       -target=aws_appautoscaling_policy.server_scale_down \
                       -target=aws_cloudwatch_metric_alarm.server_cpu_utilization_high \
                       -target=aws_cloudwatch_metric_alarm.server_cpu_utilization_low \
                       -target=aws_cloudwatch_log_group.server_log_group \
                       -target=aws_cloudwatch_log_stream.server_log_stream \
                       -target=aws_iam_role.autoscale_role \
                       -target=aws_iam_policy.autoscale_policy \
                       -target=aws_iam_role_policy_attachment.server_autoscale_attachment \
                       -target=aws_iam_role.task_execution_role \
                       -target=aws_iam_policy.task_execution_policy \
                       -target=aws_iam_role_policy_attachment.server_execution_attachment \
                       -target=aws_route53_record.birb \
                       -target=aws_security_group.lb \
                       -target=aws_security_group.ecs_tasks \
                       terraform/
                ",
                )?;
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
                    bb plan edgar
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
                    AWS_SDK_LOAD_CONFIG=1   AWS_PROFILE=birb terraform destroy -var-file=terraform/production.secret.tfvars \
                       -auto-approve \
                       -target=aws_launch_configuration.edgar_launch_configuration \
                       -target=aws_autoscaling_group.edgar_autoscaling \
                       -target=aws_ecs_cluster.edgar_cluster \
                       -target=aws_ecs_task_definition.edgar_task \
                       -target=aws_ecs_service.edgar_service \
                       -target=aws_iam_role.edgar_instance_role \
                       -target=aws_iam_policy.edgar_resource_access_policy \
                       -target=aws_iam_role_policy_attachment.edgar_instance_role_attachment \
                       -target=aws_iam_role_policy_attachment.edgar_resource_access_attachment \
                       -target=aws_iam_instance_profile.edgar_instance_profile \
                       -target=aws_iam_role.edgar_service_role \
                       -target=aws_iam_role_policy_attachment.edgar_service_role_attachment \
                       -target=aws_cloudwatch_log_group.edgar_log_group \
                       -target=aws_cloudwatch_log_stream.edgar_log_stream \
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
                bb aws plan up
            ",
                )?;
            }
            AwsBastion::Down => {
                let _result = run_str_in_bash(
                    "
                AWS_SDK_LOAD_CONFIG=1   AWS_PROFILE=birb terraform destroy -var-file=terraform/production.secret.tfvars \
                           -auto-approve \
                           -target=aws_instance.bastion \
                           -target=local_file.bastion_ip_address \
                           terraform/
            ",
                )?;
            }
            AwsBastion::ReDeploy => {
                let _result = run_str_in_bash(
                    "
                bb aws bastion down && bb aws bastion up
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
            bb aws plan up
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
            bb aws plan up
        ",
                )?;
            }
        }

        Ok(())
    }
}

impl Subcommand for AwsStateful {
    fn run(&self) -> Result<(), failure::Error> {
        match self {
            AwsStateful::Up => {
                // Not currently worrying about whether or not the deploy was successful
                let _plan = run_str_in_bash(
                    "
                    bb plan stateful
                ",
                )?;

                let _result = run_str_in_bash(
                    "
                    bb aws plan up
                ",
                )?;
            }
        }

        Ok(())
    }
}

impl Subcommand for AwsStateless {
    fn run(&self) -> Result<(), failure::Error> {
        match self {
            AwsStateless::Up => {
                // Not currently worrying about whether or not the deploy was successful
                let _plan = run_str_in_bash(
                    "
                    bb plan stateless
                ",
                )?;

                let _result = run_str_in_bash(
                    "
                    bb aws plan up
                ",
                )?;
            }
            AwsStateless::Down => {
                let _result = run_str_in_bash(
                    "
                       AWS_SDK_LOAD_CONFIG=1   AWS_PROFILE=birb terraform destroy -var-file=terraform/production.secret.tfvars \
                           -auto-approve \
                           -target=aws_alb.server_load_balancer \
                           -target=aws_alb_target_group.server_target_group \
                           -target=aws_alb_listener.server_lb_listener \
                           -target=aws_alb_listener.redirect_to_ssl \
                           -target=aws_security_group.lb \
                           -target=aws_instance.bastion \
                           -target=aws_ecs_cluster.server_cluster \
                           -target=aws_ecs_service.server_service \
                           -target=aws_ecs_task_definition.server_task \
                           -target=aws_launch_configuration.edgar_launch_configuration \
                           -target=aws_autoscaling_group.edgar_autoscaling \
                           -target=aws_appautoscaling_target.server_autoscaling_target \
                           -target=aws_appautoscaling_policy.server_scale_up \
                           -target=aws_appautoscaling_policy.server_scale_down \
                           -target=aws_cloudwatch_metric_alarm.server_cpu_utilization_high \
                           -target=aws_cloudwatch_metric_alarm.server_cpu_utilization_low \
                           -target=aws_ecs_cluster.edgar_cluster \
                           -target=aws_ecs_task_definition.edgar_task \
                           -target=aws_ecs_service.edgar_service \
                           -target=aws_iam_role.edgar_instance_role \
                           -target=aws_iam_role_policy_attachment.edgar_instance_role_attachment \
                           -target=aws_iam_role_policy_attachment.edgar_resource_access_attachment \
                           -target=aws_iam_instance_profile.edgar_instance_profile \
                           -target=aws_iam_role.edgar_service_role \
                           -target=aws_iam_role_policy_attachment.edgar_service_role_attachment \
                           -target=aws_cloudwatch_log_group.server_log_group \
                           -target=aws_cloudwatch_log_stream.server_log_stream \
                           -target=aws_cloudwatch_log_group.edgar_log_group \
                           -target=aws_cloudwatch_log_stream.edgar_log_stream \
                           -target=aws_vpc.main \
                           -target=aws_subnet.private \
                           -target=aws_subnet.public \
                           -target=aws_internet_gateway.gw \
                           -target=aws_route.internet_access \
                           -target=aws_eip.gw \
                           -target=aws_nat_gateway.gw \
                           -target=aws_route_table.private \
                           -target=aws_route_table_association.private \
                           -target=local_file.bastion_ip_address \
                           -target=local_file.rds_db_name \
                           -target=local_file.rds_db_port \
                           -target=local_file.rds_db_address \
                           -target=local_file.rds_db_username \
                           -target=local_file.rds_db_password \
                           -target=aws_iam_role.autoscale_role \
                           -target=aws_iam_policy.autoscale_policy \
                           -target=aws_iam_role_policy_attachment.server_autoscale_attachment \
                           -target=aws_iam_role.task_execution_role \
                           -target=aws_iam_policy.task_execution_policy \
                           -target=aws_iam_role_policy_attachment.server_execution_attachment \
                           -target=aws_route53_record.birb \
                           -target=aws_secretsmanager_secret.ROCKET_DATABASES \
                           -target=aws_secretsmanager_secret_version.ROCKET_DATABASES \
                           -target=aws_secretsmanager_secret.DATABASE_URI \
                           -target=aws_secretsmanager_secret_version.DATABASE_URI \
                           -target=aws_security_group.lb \
                           -target=aws_security_group.ecs_tasks \
                           -target=aws_security_group.rds_security_group \
                           -target=aws_security_group.bastion \
                           -target=aws_security_group.birb-edgar \
                           terraform/
                ",
                )?;
            }
        }

        Ok(())
    }
}

impl Subcommand for AwsS3 {
    fn run(&self) -> Result<(), failure::Error> {
        match self {
            AwsS3::Upload(s3_upload) => {
                match s3_upload {
                    S3Buckets::EdgarIndexes => {
                        // Not currently worrying about whether or not the deploy was successful
                        let _result = run_str_in_bash(
                            "aws s3 cp data/edgar-indexes s3://birb-edgar-indexes/ --recursive",
                        )?;
                    }
                }
            }
            AwsS3::Book(book) => {
                match book {
                    Book::Up => {
                      // Not currently worrying about whether or not the deploy was successful
                      let _plan = run_str_in_bash(
                        "
                          bb plan book
                      ",
                      )?;

                      let _result = run_str_in_bash(
                        "
                          bb aws plan up
                      ",
                      )?;
                    }
                }
            }
        }

        Ok(())
    }
}
