use solana_sdk::pubkey::Pubkey;
use serde::{Serialize, Deserialize};

pub const PROGRAM_ID: &str = "imgZzuUv47Wwy6aV39mAksorLYZkUfswwp74Bq9PPjX";

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub url: String,
    pub title: String
}

pub fn get_pda(signer: &Pubkey, image: &Image) -> String {
    let program_id = Pubkey::try_from(PROGRAM_ID).unwrap();

    let (pda, _bump_seed) = Pubkey::find_program_address(
        &[signer.as_ref(), image.title.as_bytes()],
        &program_id
    );

    pda.to_string()
}

pub fn get_data(image: &Image) -> Vec<u8> {
    bincode::serialize(&image).unwrap()
}

