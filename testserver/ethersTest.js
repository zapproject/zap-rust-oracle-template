let ethers=require('ethers')
const offchain=require('./contracts/lib/platform/OffChainClient.sol/OffChainClient.json')
const dispatch=require('./contracts/platform/dispatch/Dispatch.sol/Dispatch.json')
const bondage=require('./contracts/platform/bondage/Bondage.sol/Bondage.json')
require('dotenv').config()
let customHttpProvider = new ethers.providers.JsonRpcProvider(process.env.PROVIDER);
//let contract = new ethers.Contract(process.env.ADDRESS, offchain.abi, customHttpProvider);
let contract = new ethers.Contract("0x9a676e781a523b5d0c0e43731313a708cb607508", bondage.abi, customHttpProvider);
let paths=["m/44'/60'/1'/0/0","m/44'/60'/2'/0/0","m/44'/60'/3'/0/0","m/44'/60'/4'/0/0","m/44'/60'/5'/0/0"]
let wallet = new ethers.Wallet.fromMnemonic("test test test test test test test test test test test junk");
let wallet2 = new ethers.Wallet.fromMnemonic("test test test test test test test test test test test junk",paths[1]);
console.log(wallet._signingKey())
console.log(wallet2)
//let contractWithSigner = contract.connect(wallet);
//console.log(contractWithSigner)