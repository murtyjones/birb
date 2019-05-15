use crate::{run_str_in_bash, Subcommand};

/// Deploy different applications / services to aws
#[derive(Debug, StructOpt)]
pub enum Aws {
    /// Deploy all infrastructure
    #[structopt(name = "all")]
    All(AwsAll),
    /// Deploy the API
    #[structopt(name = "api")]
    Api(AwsApi),
    /// Deploy the Edgar worker
    #[structopt(name = "edgar")]
    Edgar(AwsEdgar),
    /// Deploy the Edgar worker
    #[structopt(name = "bastion")]
    Bastion(AwsBastion),
    /// Deploy the Database
    #[structopt(name = "rds")]
    RDS(AwsRDS),
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
            Aws::Api(api) => api.run(),
            Aws::Edgar(edgar) => edgar.run(),
            Aws::Bastion(bastion) => bastion.run(),
            Aws::RDS(rds) => rds.run(),
            Aws::Plan(tf_plan) => tf_plan.run(),
            Aws::Output(output) => output.run(),
            Aws::Stateful(stateful) => stateful.run(),
            Aws::Stateless(stateless) => stateless.run(),
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

/// Manage the API infrastructure
#[derive(Debug, StructOpt)]
pub enum AwsApi {
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
    // TODO add down
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

impl Subcommand for AwsApi {
    fn run(&self) -> Result<(), failure::Error> {
        match self {
            AwsApi::Up => {
                // Not currently worrying about whether or not the deploy was successful
                let _plan = run_str_in_bash(
                    "
                    terraform plan -var-file=terraform/production.secret.tfvars \
                       -out=plan \
                       -target=aws_alb.main \
                       -target=aws_alb_target_group.app \
                       -target=aws_alb_listener.front_end \
                       -target=aws_security_group.lb \
                       -target=aws_security_group.lb \
                       -target=aws_ecs_cluster.main \
                       -target=aws_ecs_service.main \
                       -target=aws_ecs_task_definition.app \
                       -target=aws_appautoscaling_target.target \
                       -target=aws_appautoscaling_policy.up \
                       -target=aws_appautoscaling_policy.down \
                       -target=aws_cloudwatch_metric_alarm.service_cpu_high \
                       -target=aws_cloudwatch_metric_alarm.service_cpu_low \
                       -target=aws_cloudwatch_log_group.birb_api_log_group \
                       -target=aws_cloudwatch_log_stream.birb_api_log_stream \
                       -target=aws_iam_role.autoscale_role \
                       -target=aws_iam_policy.autoscale_policy \
                       -target=aws_iam_role_policy_attachment.autoscale-attach \
                       -target=aws_iam_role.task_execution_role \
                       -target=aws_iam_policy.task_execution_policy \
                       -target=aws_iam_role_policy_attachment.task-execution-attach \
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
            AwsApi::Down => {
                let _reuslt = run_str_in_bash(
                    "
                    terraform destroy -var-file=terraform/production.secret.tfvars \
                       -auto-approve \
                       -target=aws_alb.main \
                       -target=aws_alb_target_group.app \
                       -target=aws_alb_listener.front_end \
                       -target=aws_security_group.lb \
                       -target=aws_security_group.lb \
                       -target=aws_ecs_cluster.main \
                       -target=aws_ecs_service.main \
                       -target=aws_ecs_task_definition.app \
                       -target=aws_appautoscaling_target.target \
                       -target=aws_appautoscaling_policy.up \
                       -target=aws_appautoscaling_policy.down \
                       -target=aws_cloudwatch_metric_alarm.service_cpu_high \
                       -target=aws_cloudwatch_metric_alarm.service_cpu_low \\
                       -target=aws_cloudwatch_log_group.birb_api_log_group \
                       -target=aws_cloudwatch_log_stream.birb_api_log_stream \
                       -target=aws_iam_role.autoscale_role \
                       -target=aws_iam_policy.autoscale_policy \
                       -target=aws_iam_role_policy_attachment.autoscale-attach \
                       -target=aws_iam_role.task_execution_role \
                       -target=aws_iam_policy.task_execution_policy \
                       -target=aws_iam_role_policy_attachment.task-execution-attach \
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
                    terraform destroy -var-file=terraform/production.secret.tfvars \
                       -auto-approve \
                       -target=aws_launch_configuration.ecs-launch-configuration \
                       -target=aws_autoscaling_group.ecs-autoscaling-group \
                       -target=aws_ecs_cluster.birb-edgar-cluster \
                       -target=aws_ecs_task_definition.birb-edgar-task \
                       -target=aws_ecs_service.birb-edgar-service \
                       -target=aws_iam_role.ecs-instance-role \
                       -target=aws_iam_policy.ecs-instance-policy-secrets \
                       -target=aws_iam_role_policy_attachment.ecs-instance-role-attachment \
                       -target=aws_iam_role_policy_attachment.ecs-instance-role-attachment-secrets \
                       -target=aws_iam_instance_profile.ecs-instance-profile \
                       -target=aws_iam_role.ecs-service-role \
                       -target=aws_iam_role_policy_attachment.ecs-service-role-attachment \
                       -target=aws_cloudwatch_log_group.birb_edgar_worker_log_group \
                       -target=aws_cloudwatch_log_stream.birb_edgar_worker_log_stream \
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
                       terraform destroy -var-file=terraform/production.secret.tfvars \
                           -auto-approve \
                           -target=aws_alb.main \
                           -target=aws_alb_target_group.app \
                           -target=aws_alb_listener.front_end \
                           -target=aws_security_group.lb \
                           -target=aws_security_group.lb \
                           -target=aws_instance.bastion \
                           -target=aws_key_pair.bastion_key \
                           -target=aws_ecs_cluster.main \
                           -target=aws_ecs_service.main \
                           -target=aws_ecs_task_definition.app \
                           -target=aws_launch_configuration.ecs-launch-configuration \
                           -target=aws_autoscaling_group.ecs-autoscaling-group \
                           -target=aws_appautoscaling_target.target \
                           -target=aws_appautoscaling_policy.up \
                           -target=aws_appautoscaling_policy.down \
                           -target=aws_cloudwatch_metric_alarm.service_cpu_high \
                           -target=aws_cloudwatch_metric_alarm.service_cpu_low \
                           -target=aws_ecs_cluster.birb-edgar-cluster \
                           -target=aws_ecs_task_definition.birb-edgar-task \
                           -target=aws_ecs_service.birb-edgar-service \
                           -target=aws_iam_role.ecs-instance-role \
                           -target=aws_iam_role_policy_attachment.ecs-instance-role-attachment \
                           -target=aws_iam_role_policy_attachment.ecs-instance-role-attachment-secrets \
                           -target=aws_iam_instance_profile.ecs-instance-profile \
                           -target=aws_iam_role.ecs-service-role \
                           -target=aws_iam_role_policy_attachment.ecs-service-role-attachment \
                           -target=aws_cloudwatch_log_group.birb_api_log_group \
                           -target=aws_cloudwatch_log_stream.birb_api_log_stream \
                           -target=aws_cloudwatch_log_group.birb_edgar_worker_log_group \
                           -target=aws_cloudwatch_log_stream.birb_edgar_worker_log_stream \
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
                           -target=aws_iam_role_policy_attachment.autoscale-attach \
                           -target=aws_iam_role.task_execution_role \
                           -target=aws_iam_policy.task_execution_policy \
                           -target=aws_iam_role_policy_attachment.task-execution-attach \
                           -target=aws_route53_record.birb \
                           -target=aws_secretsmanager_secret.ROCKET_DATABASES \
                           -target=aws_secretsmanager_secret_version.ROCKET_DATABASES \
                           -target=aws_secretsmanager_secret.DATABASE_URI \
                           -target=aws_secretsmanager_secret_version.DATABASE_URI \
                           -target=aws_security_group.lb \
                           -target=aws_security_group.ecs_tasks \
                           -target=aws_security_group.birb_rds \
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
