const CONTRACT_ID = "CCQPT6X5KAMUO6P4DBBWOSQTDNQZ54WV27JQTWTAZI3OFGJHS5CDPEIT";

const RPC_URL = "https://soroban-testnet.stellar.org";

let userPublicKey = "";

const server = new StellarSdk.SorobanRpc.Server(RPC_URL);

async function connectWallet() {
  try {
    if (!window.freighterApi && !window.freighter) {
      alert(
        "Freighter not detected. Make sure extension is installed and enabled.",
      );
      return;
    }

    const publicKey = window.freighterApi
      ? await window.freighterApi.requestAccess()
      : await window.freighter.requestAccess();

    userPublicKey = publicKey;

    document.getElementById("walletStatus").innerText =
      "Wallet: " + userPublicKey;
  } catch (e) {
    console.error(e);
    alert("Wallet connection failed: " + e.message);
  }
}

/* PLACE BID */
async function placeBid() {
  try {
    const account = await server.getAccount(userPublicKey);

    const tx = new StellarSdk.TransactionBuilder(account, {
      fee: StellarSdk.BASE_FEE,
      networkPassphrase: "Test SDF Network ; September 2015",
    })
      .addOperation(
        StellarSdk.Operation.invokeContractFunction({
          contract: CONTRACT_ID,
          function: "place_bid",
          args: [],
        }),
      )
      .setTimeout(30)
      .build();

    const xdr = tx.toXDR();

    const signedXdr = await window.freighter.signTransaction(xdr, "TESTNET");

    const signedTx = StellarSdk.TransactionBuilder.fromXDR(
      signedXdr,
      "Test SDF Network ; September 2015",
    );

    const result = await server.sendTransaction(signedTx);

    document.getElementById("auctionStatus").innerText = "Bid submitted ✔";

    console.log(result);
  } catch (err) {
    console.error(err);
    alert("Bid failed (check console)");
  }
}

/* PLACEHOLDERS (we will upgrade next) */
async function createAuction() {
  alert("Create auction will be connected next");
}

async function finalizeAuction() {
  alert("Finalize auction will be connected next");
}

async function claimRefund() {
  alert("Refund will be connected next");
}
