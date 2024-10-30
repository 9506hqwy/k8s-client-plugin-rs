use clap::Parser;
use kube::api::Api;
use kube::api::PostParams;
use kube::Resource;
use kube::{Client, ResourceExt};
use kubectl_create_sample::{Sample, SampleSpec};
use std::error::Error;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    name: String,

    #[arg(short, long)]
    namespace: Option<String>,

    #[arg(short, long = "spec")]
    spec_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let args = Args::parse();

    let name = args.name;
    let namespace = args.namespace.unwrap_or("default".to_string());
    let spec_name = args.spec_name;

    let client = Client::try_default().await?;
    let samples: Api<Sample> = Api::namespaced(client.clone(), &namespace);

    let resource = Sample {
        metadata: kube::api::ObjectMeta {
            name: Some(name),
            namespace: Some(namespace),
            ..Default::default()
        },
        spec: SampleSpec { name: spec_name },
    };

    let result = samples.create(&PostParams::default(), &resource).await?;

    let singular = Sample::kind(&()).to_ascii_lowercase();
    let group = Sample::group(&());
    let name = result.name_any();
    println!("{singular}.{group}/{name} created");

    Ok(())
}
