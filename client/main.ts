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
    await establishConnection();

    await establishPayer();

    await establishMint();

    await establishPayerTokenAcount();

    await checkProgram();

    await establishVault();

    await establishVaultAta();
    
    let result = await execute(process.argv.slice(2)[0] as string);

    if (result == false) {
        throw new Error("Run fail")
    }
    console.log("Success");
}


main().then(
    () => process.exit(),
    (err) => {
        console.error(err);
        process.exit(-1)
    }
);
