use std::time::Instant;
use waves_protobuf_schemas::waves::node::grpc::accounts_api_client::AccountsApiClient;
use waves_protobuf_schemas::waves::node::grpc::DataRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // control mainnet
    // let address: Vec<u8> = vec![
    //     1, 87, 35, 179, 173, 18, 92, 147, 164, 202, 236, 57, 254, 79, 70, 214, 205, 86, 77, 45,
    //     251, 16, 251, 99, 152, 122,
    // ];

    // auction mainnet
    let address: Vec<u8> = vec![
        1, 87, 154, 181, 157, 184, 139, 62, 65, 106, 107, 228, 24, 157, 46, 59, 10, 118, 175, 6,
        34, 208, 53, 138, 117, 16,
    ];

    // liquidation mainnet
    // let address: Vec<u8> = vec![
    //     1, 87, 26, 234, 73, 203, 250, 10, 126, 202, 8, 127, 40, 213, 159, 149, 142, 12, 123, 105,
    //     181, 169, 56, 145, 31, 72,
    // ];

    assert_eq!(address.len(), 26);

    // remove 2 first and 4 last bytes from address for Go Node
    let address = Vec::from(&address[2..22]);
    assert_eq!(address.len(), 20);

    // let mut client = AccountsApiClient::connect("http://grpc.wavesnodes.com:6870").await?;
    let mut client =
        AccountsApiClient::connect("http://mainnet-go-htz-fsn1-1.wavesnodes.com:6870").await?;

    let request = tonic::Request::new(DataRequest {
        address: address,
        key: String::new(),
    });

    let request_start_time = Instant::now();

    // todo how does streaming in tonic work?
    let mut stream = client.get_data_entries(request).await?.into_inner();

    let mut data_entries = Vec::new();
    let mut counter: usize = 0;
    while let Some(entry) = stream.message().await? {
        data_entries.push(entry);
        counter += 1;
        if counter % 1000 == 0 {
            println!(
                "{} records received, average time per record: {:?}",
                &counter,
                request_start_time.elapsed() / counter as u32
            );
        }
    }

    println!(
        "Time elapsed: {:?}, response length {}",
        request_start_time.elapsed(),
        data_entries.len()
    );

    Ok(())
}
