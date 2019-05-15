use crate::bash_completion::BashCompletionGenerator;
use crate::bb_filesystem::bb_cli_dir;
use crate::{run_str_in_bash, Subcommand};

#[derive(Debug, StructOpt)]
pub enum Plan {
    /// Plan all infrastructure (ex. SSL certificates)
    #[structopt(name = "all")]
    All,
    /// Plan all infrastructure (ex. SSL certificates)
    #[structopt(name = "api")]
    Api,
    /// Plan all infrastructure (ex. SSL certificates)
    #[structopt(name = "edgar")]
    Edgar,
    /// Plan all infrastructure (ex. SSL certificates)
    #[structopt(name = "bastion")]
    Bastion,
    /// Plan all local outputs needed
    #[structopt(name = "output")]
    Output,
    /// Plan the DB infrastructure
    #[structopt(name = "rds")]
    RDS,
    /// Plan stateful pieces of architecture
    #[structopt(name = "stateful")]
    Stateful,
    /// Plan stateless pieces of architecture
    #[structopt(name = "stateless")]
    Stateless,
}

impl Subcommand for Plan {
    fn run(&self) -> Result<(), failure::Error> {
        match self {
            Plan::All => {
                run_str_in_bash(
                    "
                    terraform plan -var-file=terraform/production.secret.tfvars \
                                   -out=plan \
                                   terraform/
                ",
                )
                .unwrap();
                Ok(())
            }
            Plan::Api => {
                // TODO make this actually plan the right things
                run_str_in_bash(
                    "
                    terraform plan -var-file=terraform/production.secret.tfvars \
                           -out=plan \
                           -target=aws_lambda_function.edgar_worker \
                           -target=aws_iam_role.edgar_worker \
                           terraform/
                ",
                )
                .unwrap();
                Ok(())
            }
            Plan::Edgar => {
                run_str_in_bash(
                    "
                    terraform plan -var-file=terraform/production.secret.tfvars \
                           -out=plan \
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
                           -target=aws_ecr_repository.birb_edgar_worker_repo \
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
                           -target=aws_secretsmanager_secret.DATABASE_URI \
                           -target=aws_secretsmanager_secret_version.DATABASE_URI \
                           terraform/
                ",
                )
                .unwrap();
                Ok(())
            }
            Plan::Bastion => {
                run_str_in_bash(
                    "
                    terraform plan -var-file=terraform/production.secret.tfvars \
                           -out=plan \
                           -target=aws_instance.bastion \
                           terraform/
                ",
                )
                .unwrap();
                Ok(())
            }
            Plan::Output => {
                run_str_in_bash(
                    "
                    terraform plan -var-file=terraform/production.secret.tfvars \
                           -out=plan \
                           -target=local_file.bastion_ip_address \
                           -target=local_file.rds_db_name \
                           -target=local_file.rds_db_port \
                           -target=local_file.rds_db_address \
                           -target=local_file.rds_db_username \
                           -target=local_file.rds_db_password \
                           terraform/
                ",
                )
                .unwrap();
                Ok(())
            }
            Plan::RDS => {
                run_str_in_bash(
                    "
                    terraform plan -var-file=terraform/production.secret.tfvars \
                           -out=plan \
                           -target=aws_db_instance.birb \
                           terraform/
                ",
                )
                .unwrap();
                Ok(())
            }
            Plan::Stateful => {
                run_str_in_bash(
                    "
                    terraform plan -var-file=terraform/production.secret.tfvars \
                           -out=plan \
                           -target=aws_db_instance.birb \
                           -target=aws_ecr_repository.birb_api_repo \
                           -target=aws_ecr_repository.birb_edgar_worker_repo \
                           terraform/
                ",
                )
                .unwrap();
                Ok(())
            }
            Plan::Stateless => {
                run_str_in_bash(
                    "
                      terraform plan -var-file=terraform/production.secret.tfvars \
                           -out=plan \
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
                )
                .unwrap();
                Ok(())
            }
        }
    }
}
