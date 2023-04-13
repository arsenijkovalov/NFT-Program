import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NftProgram } from "../target/types/nft_program";
import { PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID } from "@metaplex-foundation/mpl-token-metadata";

describe("nft_program", async () => {
  const provider = anchor.AnchorProvider.env();
  const payer = provider.wallet as anchor.Wallet;
  anchor.setProvider(provider);

  const program = anchor.workspace.NftProgram as Program<NftProgram>;

  const mintKeypair: anchor.web3.Keypair = anchor.web3.Keypair.generate();

  const associatedTokenAccountAddress =
    await anchor.utils.token.associatedAddress({
      mint: mintKeypair.publicKey,
      owner: payer.publicKey,
    });

  const metadataAccountAddress = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      mintKeypair.publicKey.toBuffer(),
    ],
    TOKEN_METADATA_PROGRAM_ID
  )[0];

  it("Create NFT", async () => {
    const nftName = "Solana Course NFT";
    const nftSymbol = "SOLС";
    const nftUri =
      "";

    try {
      await program.methods
        .createToken(nftName, nftSymbol, nftUri)
        .accounts({
          payer: payer.publicKey,
          mintAccount: mintKeypair.publicKey,
          mintAuthority: payer.publicKey,
          associatedTokenAccount: associatedTokenAccountAddress,
          metadataAccount: metadataAccountAddress,
          tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        })
        .signers([mintKeypair, payer.payer])
        .rpc();
    } catch (error) {
      console.log(error);
    }
  });

  it("Mint NFT", async () => {
    const masterEditionAccountAddress =
      anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("metadata"),
          TOKEN_METADATA_PROGRAM_ID.toBuffer(),
          mintKeypair.publicKey.toBuffer(),
          Buffer.from("edition"),
        ],
        TOKEN_METADATA_PROGRAM_ID
      )[0];

    try {
      await program.methods
        .mintToken()
        .accounts({
          payer: payer.publicKey,
          mintAccount: mintKeypair.publicKey,
          mintAuthority: payer.publicKey,
          associatedTokenAccount: associatedTokenAccountAddress,
          metadataAccount: metadataAccountAddress,
          masterEditionAccount: masterEditionAccountAddress,
          tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        })
        .signers([mintKeypair, payer.payer])
        .rpc();
    } catch (error) {
      console.log(error);
    }
  });
});
