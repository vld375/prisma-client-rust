mod client;
mod enums;
mod models;

use super::GeneratorArgs;

pub fn generate_prisma_client(root: &GeneratorArgs) -> String {
    let mut client = client::generate(root);

    client.extend(models::generate(root));
    client.extend(enums::generate(root));

    client.to_string()
}
