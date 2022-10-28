import {
    Keypair,
    Connection,
    LAMPORTS_PER_SOL,
    SystemProgram,
    TransactionInstruction,
    Transaction,
    sendAndConfirmTransaction,
    PublicKey,
    AccountMeta
} from "@solana/web3.js";
import fs from "mz/fs";
import path from "path";
import * as borsh from "borsh";
import * as BufferLayout from "@solana/buffer-layout";
import { getPayer, getRpcUrl, createKeypairFromFile } from "./utils";
import { TOKEN_PROGRAM_ID, Account, createMint, getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";
import * as Buff from "@solana/buffer-layout-utils";
let connection: Connection;
let programId: PublicKey;
let payer: Keypair;
let payerTokenAccount: Account;
let vault: PublicKey;
let vaultTokenAccount: Account;
let mintPubkey: PublicKey;
const PROGRAM_PATH = path.resolve(__dirname, '../dist/program');
const PROGRAM_SO_PATH = path.join(PROGRAM_PATH, 'tokentracing.so');
const PROGRAM_KEYPAIR_PATH = path.join(PROGRAM_PATH, 'tokentracing-keypair.json');



type InstructionData = {
    data: Buffer,
    keys: Array<AccountMeta>
}
    ;
export async function establishConnection(): Promise<void> {
    const rpcUrl = await getRpcUrl();
    connection = new Connection(rpcUrl, 'confirmed');
    const version = await connection.getVersion();
    console.log('Connection to cluster established:', rpcUrl, version);
}

export async function establishPayer(): Promise<void> {
    let fees = 0;
    if (!payer) {
        const { feeCalculator } = await connection.getRecentBlockhash();

        fees += feeCalculator.lamportsPerSignature * 100; 

        payer = await getPayer();
    }

    let curLamport = await connection.getBalance(payer.publicKey);
    console.log("Current SOL is ", curLamport / LAMPORTS_PER_SOL);
    fees += LAMPORTS_PER_SOL;
    if (curLamport < fees) {
        const sig = await connection.requestAirdrop(
            payer.publicKey,
            fees - curLamport,
        );
        await connection.confirmTransaction(sig);
        curLamport = await connection.getBalance(payer.publicKey);
    }

    console.log(
        'Using account',
        payer.publicKey.toBase58(),
        'containing',
        curLamport / LAMPORTS_PER_SOL,
        'SOL to pay for fees',
    );
}

export async function establishMint(): Promise<void> {
    mintPubkey = new PublicKey("BWQvrPzZZVndXNYPv6VB5P6bbQsHyEvtBpNgDwNxicWi");
}
export async function establishPayerTokenAcount(): Promise<void> {
    payerTokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        payer,
        mintPubkey,
        payer.publicKey
    )
}

export async function mintToPayer() {
    await mintTo(
        connection,
        payer,
        mintPubkey,
        payerTokenAccount.address,
        payer,
        1000 * LAMPORTS_PER_SOL
    );

    console.log("Mint 1000 tokens to payer ", payerTokenAccount.address.toBase58());
}

export async function checkProgram(): Promise<void> {
    try {
        const programKeypair = await createKeypairFromFile(PROGRAM_KEYPAIR_PATH);
        programId = programKeypair.publicKey;
    } catch (err) {
        const errMsg = (err as Error).message;
        throw new Error(
            `Failed to read program keypair at '${PROGRAM_KEYPAIR_PATH}' due to error: ${errMsg}. Program may need to be deployed with \`solana program deploy dist/program/helloworld.so\``,
        );
    }

    const programInfo = await connection.getAccountInfo(programId);
    if (programInfo === null) {
        if (fs.existsSync(PROGRAM_SO_PATH)) {
            throw new Error(
                'Program needs to be deployed with `solana program deploy dist/program/tokentracing.so`',
            );
        } else {
            throw new Error('Program needs to be built and deployed');
        }
    } else if (!programInfo.executable) {
        throw new Error(`Program is not executable`);
    }
    console.log(`Using program ${programId.toBase58()}`);
}

export async function establishVault(): Promise<void> {
    const [vaultAddress] = await PublicKey.findProgramAddress(
        [Buffer.from("vault"), mintPubkey.toBuffer()],
        programId
    );
    vault = vaultAddress;
    console.log("Vault: ", vault.toBase58());
}

export async function establishVaultAta(): Promise<void> {
    vaultTokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        payer,
        mintPubkey,
        vault,
        true
    );
}


export async function initialize() {
    const layout = BufferLayout.struct([BufferLayout.u8("instruction") as BufferLayout.Layout<never>]);

    const data = Buffer.alloc(layout.span);
    layout.encode({ instruction: 0 }, data);
    const instruction = new TransactionInstruction({
        keys: [
            {
                pubkey: payer.publicKey,
                isSigner: true,
                isWritable: true,
            },
            {
                pubkey: vault,
                isSigner: false,
                isWritable: true,
            },
            {
                pubkey: SystemProgram.programId,
                isSigner: false,
                isWritable: false,
            },
            {
                pubkey: mintPubkey,
                isSigner: false,
                isWritable: true,
            },
        ],
        programId,
        data: data,
    });

    const txSig = await sendAndConfirmTransaction(
        connection,
        new Transaction().add(instruction),
        [payer]
    );
}

function swapSolToToken(): InstructionData {
    const layout = BufferLayout.struct([BufferLayout.u8("instruction") as BufferLayout.Layout<never>, BufferLayout.u32("amount") as BufferLayout.Layout<never>]);
    const data = Buffer.alloc(layout.span);
    layout.encode({ instruction: 1, amount: LAMPORTS_PER_SOL }, data);
    let keys = [
        {
            pubkey: programId,
            isSigner: false,
            isWritable: true,
        },
        {
            pubkey: payer.publicKey,
            isSigner: true,
            isWritable: true,
        },
        {
            pubkey: payerTokenAccount.address,
            isSigner: false,
            isWritable: true,
        },
        {
            pubkey: mintPubkey,
            isSigner: false,
            isWritable: true,
        },
        {
            pubkey: vault,
            isSigner: false,
            isWritable: true,
        },
        {
            pubkey: vaultTokenAccount.address,
            isSigner: false,
            isWritable: true,
        },
        {
            pubkey: TOKEN_PROGRAM_ID,
            isSigner: false,
            isWritable: false,
        },
        {
            pubkey: SystemProgram.programId,
            isSigner: false,
            isWritable: false,
        },
    ]

    return { data, keys }
}

export async function execute(index: string): Promise<boolean> {
    console.log(index)
    let instructionData: InstructionData = { data: Buffer.alloc(0), keys: [] };
    if (index == "1") {

        instructionData = swapSolToToken();
    }
    else if (index == "2") {
        instructionData = swapTokenToSol();
    }
    else {
        return false;
    }

    let instruction = new TransactionInstruction({
        keys: instructionData.keys,
        programId,
        data: instructionData.data
    })

    await sendAndConfirmTransaction(
        connection,
        new Transaction().add(instruction),
        [payer]
    )

    return true;
}


function swapTokenToSol(): InstructionData {
    const layout = BufferLayout.struct([BufferLayout.u8("instruction") as BufferLayout.Layout<never>, BufferLayout.u32("amount") as BufferLayout.Layout<never>]);

    const data = Buffer.alloc(layout.span);
    layout.encode({ instruction: 2, amount: LAMPORTS_PER_SOL }, data);
    let keys = [
                {
                    pubkey: programId,
                    isSigner: false,
                    isWritable: true,
                },
                {
                    pubkey: payer.publicKey,
                    isSigner: true,
                    isWritable: true,
                },
                {
                    pubkey: payerTokenAccount.address,
                    isSigner: false,
                    isWritable: true,
                },
                {
                    pubkey: mintPubkey,
                    isSigner: false,
                    isWritable: true,
                },
                {
                    pubkey: vault,
                    isSigner: false,
                    isWritable: true,
                },
                {
                    pubkey: vaultTokenAccount.address,
                    isSigner: false,
                    isWritable: true,
                },
                {
                    pubkey: TOKEN_PROGRAM_ID,
                    isSigner: false,
                    isWritable: false,
                },
                {
                    pubkey: SystemProgram.programId,
                    isSigner: false,
                    isWritable: false,
                },
            ];
    return { data, keys }
}