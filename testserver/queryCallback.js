let ethers=require('ethers')
let axios=require('axios')
const OffChainClient=require('./contracts/lib/platform/OffChainClient.sol/OffChainClient.json')

require('dotenv').config()



let customHttpProvider = new ethers.providers.JsonRpcProvider(process.env.PROVIDER);


let wallet_provider = new ethers.Wallet(process.env.PRIVATE_KEY_PROVIDER);
let wallet_subscriber = new ethers.Wallet(process.env.PRIVATE_SUBSCRIBER);

wallet_provider=wallet_provider.connect(customHttpProvider)
wallet_subscriber=wallet_subscriber.connect(customHttpProvider)
/**
 * 
 
 * @param {*} path 
 * @param {*} contract_address 
 * @param {*} id 
 */

async function send_callback(query,path,contract_address,id){
    let _OffChainClient= new  ethers.Contract(contract_address,OffChainClient.abi,customHttpProvider)
    let OffChainClient_instance =  _OffChainClient.connect(wallet_provider);
    let result =await axios.get(query)
    let parsed=parsePath(path,result.data)
    console.log(parsed)
    let bignum=ethers.BigNumber.from(`0x${id}`)
    await OffChainClient_instance.Callback(bignum, parsed.toString())
}

 async function initiate_query(contract_address,provider_address, query, spec, params){
    let bytesparams=convert_strings_to_params(params)
    let _OffChainClient= new  ethers.Contract(contract_address,OffChainClient.abi,customHttpProvider)
    let OffChainClient_instance =  _OffChainClient.connect(wallet_subscriber);
    OffChainClient_instance.testQuery(provider_address, query, spec, bytesparams)
}
function parsePath(path,data){

    for(let i=0;i<path.length;i++){
        data=data[path[i]]
    }
    return data;
}


function convert_strings_to_params(string_args){
  return string_args.map((arg)=>{
        return ethers.utils.formatBytes32String(arg)
    })
    
    
}

 function convert_params_to_strings(params){
    console.log(params)
    console.log(params[0].length)
    return params.map((arg)=>{
        console.log(arg)
        ethers.utils.parseBytes32String('0x'+arg)
    })
    
}

module.exports.initiate_query=initiate_query;
module.exports.send_callback=send_callback;
module.exports.convert_strings_to_params=convert_strings_to_params;
module.exports.convert_params_to_strings=convert_params_to_strings;