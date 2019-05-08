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
        }
    }
}
