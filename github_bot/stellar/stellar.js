// TODO: allow for this to be run on the main network
const axios = require('axios');
var StellarSdk = require('stellar-sdk');
StellarSdk.Network.useTestNetwork();
var server = new StellarSdk.Server('https://horizon-testnet.stellar.org');

var keypair = StellarSdk.Keypair
  .fromSecret(process.env.STELLAR_SECRET_KEY);

// Check if an account exists -- necessary since you can't send XLM to a 
// non-existent account.
const accountExists = async accountId => {
    try {
         const url = `${process.env.STELLAR_BASE_URL}/accounts/${accountId}`
         let resp = await axios.get(url);
         return true;
    } catch (err) {
        return false;
    }
}

const sendXLM = async accountId => {
    try {
        const account = await server.loadAccount(keypair.publicKey())
        let transaction = new StellarSdk.TransactionBuilder(account)
        // The `changeTrust` operation creates (or alters) a trustline
        // The `limit` parameter below is optional
        .addOperation(StellarSdk.Operation.payment({
            destination: accountId,
            asset: StellarSdk.Asset.native(),
            amount: `${process.env.AWARD_AMOUNT}`,
        }))
        .build();
        transaction.sign(keypair);
        return server.submitTransaction(transaction);
    } catch (err) {
        return null;
    }
}

const createAccount = async accountId => {
    try {
        const account = await server.loadAccount(keypair.publicKey())
        let transaction = new StellarSdk.TransactionBuilder(account)
        .addOperation(StellarSdk.Operation.createAccount({
            destination: accountId,
            startingBalance: `${process.env.AWARD_AMOUNT}`,
        }))
        .build();
        transaction.sign(keypair);
        return server.submitTransaction(transaction);
    } catch (err) {
        return null;
    }
}

// Make sure that, before we send any XLM, we actually have enough
const botHasMoney = async () => {
    try {
        const url = `${process.env.STELLAR_BASE_URL}/accounts/${keypair.publicKey()}`
        let resp = await axios.get(url);
        const balance = balances.map(x => x.asset_type == 'native');
        return balance >= Number(process.env.AWARD_AMOUNT);
   } catch (err) {
       return false;
   }
}

module.exports = {
    accountExists,
    sendXLM,
    createAccount,
}