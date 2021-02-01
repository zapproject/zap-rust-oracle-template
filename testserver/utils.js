let ethers=require('ethers')
function convert_strings_to_params(string_args){
    return string_args.map((arg)=>{
          return ethers.utils.formatBytes32String(arg)
      })
      
      
  }
  
   function convert_params_to_strings(params){
      return params.map((arg)=>{
          return ethers.utils.parseBytes32String(arg)
      })
      
  }
  let event=[{name:"id",value:"dd01e0d1e313c493bd8dcb841088d6d6bcbca3b0c3cfe6d0c76df566f0b2577d"},{name:"provider",value:"f39fd6e51aad88f6f4ce6ab8827279cfffb92266"},{name:"subscriber",value:"9a9f2ccfde556a7e9ff0848998aa4a0cfd8863ae"},{name:"query",value:"query"},{name:"endpoint",value:"0da72197e898ebe1814471a76048ed137a089f595c1e7038f2b70e98645e7652"},{name:"endpointParams",value:"[273c58c66704975459b45d17d8d262d7b6de59f5e57291ea5af890e2cf11a2b8,08455b9a244e3f6e53a4d620c3d1261bb351f203ccdf11324cfed2696b108913]"},{name:"onchainSubscriber",value:true}]
  function processIncomingEventJSON(event_object){
    let temp={}
    event_object.forEach(item=>{
      temp[item.name]=item.value;
      
    })
    return temp
  }
let r=convert_strings_to_params(['TEST1','TEST2','TEST3'])
let i= convert_params_to_strings(r)
console.log(r)
console.log(i)
console.log(processIncomingEventJSON(event))
convert_params_to_strings([
    '0x273c58c66704975459b45d17d8d262d7b6de59f5e57291ea5af890e2cf11a2b8'.s,
    '0x08455b9a244e3f6e53a4d620c3d1261bb351f203ccdf11324cfed2696b108913'
  ])