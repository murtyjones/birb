use crate::{run_str_in_bash, Subcommand};

/// Deploy different applications / services
#[derive(Debug, StructOpt)]
pub enum Deploy {
    /// Deploy the Edgar worker
    #[structopt(name = "edgar")]
    Edgar(DeployEdgar),
    /// Deploy the Edgar worker
    #[structopt(name = "bastion")]
    Bastion(DeployBastion),
    /// Deploys whatever change is held by the "plan" file
    #[structopt(name = "plan")]
    Plan(TfPlan),
    /// Destroys infrastructure (excluding RDS)
    #[structopt(name = "destroy")]
    Destroy,
}

impl Subcommand for Deploy {
    fn run(&self) -> Result<(), failure::Error> {
        match self {
            Deploy::Edgar(deploy_edgar) => deploy_edgar.run(),
            Deploy::Bastion(deploy_bastion) => deploy_bastion.run(),
            Deploy::Plan(up) => up.run(),
            Deploy::Destroy => {
                run_str_in_bash("terraform destroy -auto-approve -var-file=terraform/production.secret.tfvars terraform/");
                Ok(())
            }
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

/// Deploy the Edgar Worker
#[derive(Debug, StructOpt)]
pub struct DeployBastion {}

/// Deploy the Terraform Plan
#[derive(Debug, StructOpt)]
pub struct TfPlan {}

impl Subcommand for DeployEdgar {
    fn run(&self) -> Result<(), failure::Error> {
        // Not currently worrying about whether or not the deploy was successful
        let _plan = run_str_in_bash("
            bb plan edgar
        ")?;

        let _result = run_str_in_bash("
            bb deploy plan
        ")?;

        Ok(())
    }
}

impl Subcommand for DeployBastion {
    fn run(&self) -> Result<(), failure::Error> {
        // Not currently worrying about whether or not the deploy was successful
        let _plan = run_str_in_bash("
            bb plan bastion
        ")?;

        let _result = run_str_in_bash("
            bb deploy plan
        ")?;

        Ok(())
    }
}
