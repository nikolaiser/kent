use std::collections::BTreeMap;

use k8s_openapi::{api::core::v1::Secret, ByteString};
use kube::{api::ListParams, runtime::reflector::Lookup, Api, Client};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct SecretValue {
    data: Option<BTreeMap<String, ByteString>>,
    string_data: Option<BTreeMap<String, String>>,
}

fn extract_secret_values(secret: &Secret) -> (String, SecretValue) {
    let name: Option<String> = secret.name().map(|x| x.to_string());
    let data = secret.data.clone();
    let string_data = secret.string_data.clone();
    (name.unwrap_or_default(), SecretValue { data, string_data })
}

pub(crate) async fn get_secret_values(
    selectors: Vec<String>,
    maybe_namespace: Option<String>,
) -> anyhow::Result<BTreeMap<String, SecretValue>> {
    let client = Client::try_default().await?;

    let secrets_api: Api<Secret> = match maybe_namespace {
        Some(namespace) => Api::namespaced(client, &namespace),
        None => Api::default_namespaced(client),
    };

    let mut secret_values: BTreeMap<String, SecretValue> = BTreeMap::new();

    for selector in selectors {
        let lp = ListParams::default().fields(&selector);
        let secrets = secrets_api.list(&lp).await?.items;
        let mut selectored_values = secrets
            .iter()
            .map(extract_secret_values)
            .collect::<BTreeMap<String, SecretValue>>();
        secret_values.append(&mut selectored_values)
    }

    Ok(secret_values)
}
