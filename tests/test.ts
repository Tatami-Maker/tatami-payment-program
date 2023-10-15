import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import * as idl from "../target/idl/tatami_program.json";
import secret from "../../../sol/id.json";
import secret2 from "../test.json";

const connection = new anchor.web3.Connection(anchor.web3.clusterApiUrl("devnet"));

const keypair = anchor.web3.Keypair.fromSecretKey(Uint8Array.from(secret));
const keypair2 = anchor.web3.Keypair.fromSecretKey(Uint8Array.from(secret2));

const wallet = new anchor.Wallet(keypair);
const wallet2 = new anchor.Wallet(keypair2);

const provider = new anchor.AnchorProvider(connection, wallet, {commitment: "confirmed"});
const provider2 = new anchor.AnchorProvider(connection, wallet2, {commitment: "confirmed"});

const programId = new anchor.web3.PublicKey(idl.metadata.address);

const program = new Program(idl as anchor.Idl, programId, provider);
const program2 = new Program(idl as anchor.Idl, programId, provider2);

(async() => {
    const index = new anchor.BN(1);

    const [receipt] = anchor.web3.PublicKey.findProgramAddressSync([
        index.toArrayLike(Buffer, "le", 8),
        Buffer.from("receipt")
    ], programId);

    const tx1 = await program.methods.initialize(index)
    .accounts({
        receipt 
    })
    .rpc()

    console.log("TX is successful: ", tx1);

    const tx2 = await program.methods.withdrawPayment(index)
    .accounts({
        receipt 
    })
    .rpc()

    console.log("TX is successful: ", tx2);

    // This TX should fail - wrong signer
    // try {
    //     const tx2 = await program2.methods.withdrawPayment(index)
    //     .accounts({
    //         receipt 
    //     })
    //     .rpc()

    //     console.log("TX is successful: ", tx2);
    // } catch(e) {
    //     console.log(e);
    // }

})();