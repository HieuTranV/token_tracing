import {
    establishConnection,
    establishMint,
    establishPayer,
    establishPayerTokenAcount,
    // mintToPayer,
    checkProgram,
    establishVault,
    establishVaultAta,
    initialize,
    execute

} from "./token_tracing";

async function main() {
    // Establish connection to the cluster
    await establishConnection();

    // Determine who pays for the fees
    await establishPayer();

    // get token
    await establishMint();

    // Get payer token account
    await establishPayerTokenAcount();

    // Sent token to payer
    // await mintToPayer();

    await checkProgram();

    await establishVault();

    await establishVaultAta();
    // await mintToVaultAta();
    // await initialize();
    console.log(process.argv)
    let result = await execute(process.argv.slice(2)[0] as string);

    if (result == false) {
        throw new Error("Run fail")
    }
    // switch (process.argv.slice(2)[0]) {
    //     case "1":
    //         await swapSolToToken();
    //         break;
    //     case "2":
    //         await swapTokenToSol();
    //         break;
    //     default:
    //         throw console.error("Invalid instruction");
    // }
    console.log("Success");
}


main().then(
    () => process.exit(),
    (err) => {
        console.error(err);
        process.exit(-1)
    }
);
