use crate::{run_str_in_bash, Subcommand};

/// Deploy different applications / services
#[derive(Debug, StructOpt)]
pub enum Deploy {
    /// Deploy the Edgar worker
    #[structopt(name = "edgar")]
    Edgar(DeployEdgar),
    /// Deploys whatever change is held by the "plan" file
    #[structopt(name = "plan")]
    Plan(TfPlan),
}

impl Subcommand for Deploy {
    fn run(&self) -> Result<(), failure::Error> {
        match self {
            Deploy::Edgar(deploy_edgar) => deploy_edgar.run(),
            Deploy::Plan(up) => up.run(),
        }
    }
}

impl Subcommand for TfPlan {
    fn run(&self) -> Result<(), failure::Error> {
        let _result = run_str_in_bash(
            "terraform apply \"plan\" && rm -rf plan"
        ).unwrap();
        Ok(())
    }
}

/// Deploy the Edgar Worker
#[derive(Debug, StructOpt)]
pub struct DeployEdgar {}

/// Deploy the Terraform Plan
#[derive(Debug, StructOpt)]
pub struct TfPlan {}

impl Subcommand for DeployEdgar {
    fn run(&self) -> Result<(), failure::Error> {
        // Not currently worrying about whether or not the deploy was successful
        let _plan = run_str_in_bash("\
            terraform plan -var-file=terraform/production.secret.tfvars \
                           -out=plan -target=aws_lambda_function.edgar_worker \
                           -target=aws_iam_role.edgar_worker terraform
        ")?;

        let _result = run_str_in_bash("
            bb deploy plan
        ");

        Ok(())
    }
}
