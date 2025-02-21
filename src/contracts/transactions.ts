import type { AssembledTransaction } from '../../packages/retainer/node_modules/@stellar/stellar-sdk/lib/contract';


export function signAndSendWithModal(
    tx: AssembledTransaction<null>,
    document: Document,
    onSuccess: () => void,
    onError: (error: any) => void
) {
    console.log("signAndSendWithModal", tx);
    const modal = document.getElementById("modal") as HTMLDivElement;
    tx.sign().then(() => {
        console.log("signed");
        modal.classList.add("visible");
        tx.send()
            .then(() => {
                console.log("sent");
                onSuccess();
            })
            .finally(() => {
                modal.classList.remove("visible");
                console.log("finally");
            });
    })
        .catch((e) => {
            console.log("User canceled transaction");
            onError(e);
        });
};