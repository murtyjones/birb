use crate::{run_str_in_bash, Subcommand};

/// Deploy different applications / services
#[derive(Debug, StructOpt)]
pub enum Deploy {
    /// Deploy all infrastructure
    #[structopt(name = "all")]
    All(DeployAll),
    /// Deploy the Edgar worker
    #[structopt(name = "edgar")]
    Edgar(DeployEdgar),
    /// Deploy the Edgar worker
    #[structopt(name = "bastion")]
    Bastion(DeployBastion),
    /// Deploy the Database
    #[structopt(name = "rds")]
    RDS(DeployRDS),
    /// Deploys whatever change is held by the "plan" file
    #[structopt(name = "plan")]
    Plan(TfPlan),
    /// Destroys infrastructure (excluding RDS)
    #[structopt(name = "destroy")]
    Destroy(DeployDestroy),
    /// Gets outputs
    #[structopt(name = "output")]
    Output(DeployOutput),
}

impl Subcommand for Deploy {
    fn run(&self) -> Result<(), failure::Error> {
        match self {
            Deploy::All(deploy_all) => deploy_all.run(),
            Deploy::Edgar(deploy_edgar) => deploy_edgar.run(),
            Deploy::Bastion(deploy_bastion) => deploy_bastion.run(),
            Deploy::RDS(rds) => rds.run(),
            Deploy::Plan(tf_plan) => tf_plan.run(),
            Deploy::Output(output) => output.run(),
            Deploy::Destroy(destroy) => destroy.run(),
        }
    }
}

/// Deploy eveeeerything
#[derive(Debug, StructOpt)]
pub struct DeployAll {}

/// Deploy the Edgar Worker
#[derive(Debug, StructOpt)]
pub struct DeployEdgar {}

/// Deploy the Bastion
#[derive(Debug, StructOpt)]
pub struct DeployBastion {}

/// Get the outputs
#[derive(Debug, StructOpt)]
pub struct DeployOutput {}

/// Deploy the Terraform Plan
#[derive(Debug, StructOpt)]
pub struct TfPlan {}

/// Destroys the Terraform Infrastructure (ex. RDS)
#[derive(Debug, StructOpt)]
pub struct DeployDestroy {}

/// Destroys the Terraform Infrastructure (ex. RDS)
#[derive(Debug, StructOpt)]
pub struct DeployRDS {}

impl Subcommand for DeployAll {
    fn run(&self) -> Result<(), failure::Error> {
        let _plan = run_str_in_bash(
            "
            bb plan all
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

impl Subcommand for TfPlan {
    fn run(&self) -> Result<(), failure::Error> {
        let _result = run_str_in_bash("terraform apply \"plan\" && rm -rf plan").unwrap();
        Ok(())
    }
}

impl Subcommand for DeployEdgar {
    fn run(&self) -> Result<(), failure::Error> {
        let _build = run_str_in_bash(
            "
            bb build edgar
        ",
        )?;
        let _push = run_str_in_bash(
            "
            bb push edgar
        ",
        )?;

        // Not currently worrying about whether or not the deploy was successful
        let _plan = run_str_in_bash(
            "
            bb plan edgar
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

impl Subcommand for DeployBastion {
    fn run(&self) -> Result<(), failure::Error> {
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

        Ok(())
    }
}

impl Subcommand for DeployDestroy {
    fn run(&self) -> Result<(), failure::Error> {
        // Not currently worrying about whether or not the deploy was successful
        run_str_in_bash(
            "
                terraform destroy -auto-approve \
                    -var-file=terraform/production.secret.tfvars \
                    terraform/\
            ",
        )
        .unwrap();
        Ok(())
    }
}

impl Subcommand for DeployOutput {
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

impl Subcommand for DeployRDS {
    fn run(&self) -> Result<(), failure::Error> {
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

        Ok(())
    }
}
