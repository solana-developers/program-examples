use litesvm::LiteSVM;
use solana_instruction::AccountMeta;
use solana_keypair::Keypair;
use solana_message::Message;
use solana_native_token::LAMPORTS_PER_SOL;
use solana_pubkey::{pubkey, Pubkey};
use solana_signer::Signer;
use solana_transaction::Transaction;

#[derive(Debug)]
#[repr(C)]
pub struct Favorites {
    pub number: u64,
    pub color: [u8; 32],
    pub hobbies: [[u8; 32]; 4],
}

impl Favorites {
    pub fn new(number: u64, color: &str, hobbies: Vec<&str>) -> Self {
        let mut favaorites = Favorites {
            number,
            color: [0; 32],
            hobbies: [[0; 32]; 4],
        };

        let color_bytes = color.as_bytes();
        let copy_len = color_bytes.len().min(32);
        favaorites.color[..copy_len].copy_from_slice(&color_bytes[..copy_len]);

        for (i, e) in hobbies.iter().enumerate().take(4) {
            let bytes = e.as_bytes();
            let len = e.len().min(32);
            favaorites.hobbies[i][..len].copy_from_slice(&bytes[..len]);
        }

        favaorites
    }

    pub fn deserialize(bytes: &[u8]) -> (u64, &str, Vec<&str>) {
        let number = u64::from_le_bytes(bytes[0..8].try_into().unwrap());

        let color = std::str::from_utf8(bytes[8..40].try_into().unwrap())
            .unwrap()
            .trim_end_matches('\0');

        let hobbies = vec![
            std::str::from_utf8(bytes[40..72].try_into().unwrap())
                .unwrap()
                .trim_end_matches('\0'),
            std::str::from_utf8(bytes[72..104].try_into().unwrap())
                .unwrap()
                .trim_end_matches('\0'),
            std::str::from_utf8(bytes[104..136].try_into().unwrap())
                .unwrap()
                .trim_end_matches('\0'),
            std::str::from_utf8(bytes[136..168].try_into().unwrap())
                .unwrap()
                .trim_end_matches('\0'),
        ];

        (number, color, hobbies)
    }
}

#[test]
fn test() {
    let user_keypair = Keypair::new();

    let mut svm = LiteSVM::new();
    svm.airdrop(&user_keypair.pubkey(), LAMPORTS_PER_SOL * 10)
        .unwrap();

    let program_id = pubkey!("kxKpwU7ZKheCSCDqwHyQUBW86GDGxci4ceK1xgzsWUn");

    let program_bytes = include_bytes!("./fixtures/favorites_program.so");

    svm.add_program(program_id, program_bytes);

    let (favourites_account, _) =
        Pubkey::find_program_address(&[b"favorites", user_keypair.pubkey().as_ref()], &program_id);

    let accounts = vec![
        AccountMeta::new(user_keypair.pubkey(), true),
        AccountMeta::new(favourites_account, false),
        AccountMeta::new_readonly(solana_program::system_program::ID, false),
    ];

    let mut data = vec![0];
    let favorites_info_data = Favorites::new(200, "purple", vec!["tx", "ix", "mev", "bundle"]);
    // Convert struct to bytes using unsafe transmute since we have #[repr(C)]
    let favorites_info_bytes = unsafe {
        std::slice::from_raw_parts(
            &favorites_info_data as *const Favorites as *const u8,
            std::mem::size_of::<Favorites>(),
        )
    };
    data.extend_from_slice(favorites_info_bytes);

    let instruction = solana_instruction::Instruction {
        program_id,
        accounts,
        data,
    };

    let tx = Transaction::new(
        &[&user_keypair],
        Message::new(&[instruction], Some(&user_keypair.pubkey())),
        svm.latest_blockhash(),
    );
    let tx_res = svm.send_transaction(tx);
    match tx_res {
        Ok(res) => {
            dbg!(res.logs);
        }
        Err(e) => {
            dbg!(e.meta.logs);
            assert!(false);
        }
    };

    let data = svm.get_account(&favourites_account).unwrap().data;
    println!("{:?}", data);
    println!("{:?}", Favorites::deserialize(&data));

    let accounts = vec![
        AccountMeta::new(user_keypair.pubkey(), true),
        AccountMeta::new(favourites_account, false),
    ];
    let data = vec![1];

    let instruction = solana_instruction::Instruction {
        program_id,
        accounts,
        data,
    };

    let tx = Transaction::new(
        &[&user_keypair],
        Message::new(&[instruction], Some(&user_keypair.pubkey())),
        svm.latest_blockhash(),
    );
    let tx_res = svm.send_transaction(tx);
    match tx_res {
        Ok(res) => {
            dbg!(res.logs);
        }
        Err(e) => {
            dbg!(e.meta.logs);
            assert!(false);
        }
    }
}
