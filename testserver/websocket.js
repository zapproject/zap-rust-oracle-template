var net = require('net');
let callBackUtils=require('./queryCallback.js')
// creates the server
console.log(callBackUtils)
var server = net.createServer();

//emitted when server closes ...not emitted until all connections closes.
server.on('close',function(){
  console.log('Server closed !');
});

// emitted when new client connects
server.on('connection',function(socket){

//this property shows the number of characters currently buffered to be written. (Number of characters is approximately equal to the number of bytes to be written, but the buffer may contain strings, and the strings are lazily encoded, so the exact number of bytes is not known.)
//Users who experience large or growing bufferSize should attempt to "throttle" the data flows in their program with pause() and resume().

  console.log('Buffer size : ' + socket.bufferSize);

  console.log('---------server details -----------------');

  var address = server.address();
  var port = address.port;
  var family = address.family;
  var ipaddr = address.address;
  console.log('Server is listening at port' + port);
  console.log('Server ip :' + ipaddr);
  console.log('Server is IP4/IP6 : ' + family);

  var lport = socket.localPort;
  var laddr = socket.localAddress;
  console.log('Server is listening at LOCAL port' + lport);
  console.log('Server LOCAL ip :' + laddr);

  console.log('------------remote client info --------------');

  var rport = socket.remotePort;
  var raddr = socket.remoteAddress;
  var rfamily = socket.remoteFamily;

  console.log('REMOTE Socket is listening at port' + rport);
  console.log('REMOTE Socket ip :' + raddr);
  console.log('REMOTE Socket is IP4/IP6 : ' + rfamily);

  console.log('--------------------------------------------')
//var no_of_connections =  server.getConnections(); // sychronous version
server.getConnections(function(error,count){
  console.log('Number of concurrent connections to the server : ' + count);
});

socket.setEncoding('utf8');

socket.setTimeout(800000,function(){
  // called after timeout -> same as socket.on('timeout')
  // it just tells that soket timed out => its ur job to end or destroy the socket.
  // socket.end() vs socket.destroy() => end allows us to send final data and allows some i/o activity to finish before destroying the socket
  // whereas destroy kills the socket immediately irrespective of whether any i/o operation is goin on or not...force destry takes place
  console.log('Socket timed out');
});


socket.on('data',function(data){
  var bread = socket.bytesRead;
  var bwrite = socket.bytesWritten;
  console.log('Bytes read : ' + bread);
  console.log('Bytes written : ' + bwrite);
  console.log('Data sent to server : ' + data);

  let formatted_event=processIncomingEventJSON(JSON.parse(data))
  console.log(formatted_event)
  let params=formatEndpointParams(formatted_event.endpointParams)
  console.log(params)
  //let path=callBackUtils.convert_params_to_strings(params)

  
  callBackUtils.send_callback(formatted_event.query,["0x514910771af9ca656af840dff83e8264ecf986ca","usd"],`0x${formatted_event.subscriber}`,formatted_event.id)

  var is_kernel_buffer_full = socket.write('Data ::' + data);
  if(is_kernel_buffer_full){
    console.log('Data was flushed successfully from kernel buffer i.e written successfully!');
  }else{
    socket.pause();
  }

});

socket.on('drain',function(){
  console.log('write buffer is empty now .. u can resume the writable stream');
  socket.resume();
});

socket.on('error',function(error){
  console.log('Error : ' + error);
});

socket.on('timeout',function(){
  console.log('Socket timed out !');
  socket.end('Timed out!');
  // can call socket.destroy() here too.
});

socket.on('end',function(data){
  console.log('Socket ended from other end!');
  console.log('End data : ' + data);
});

socket.on('close',function(error){
  var bread = socket.bytesRead;
  var bwrite = socket.bytesWritten;
  console.log('Bytes read : ' + bread);
  console.log('Bytes written : ' + bwrite);
  console.log('Socket closed!');
  if(error){
    console.log('Socket was closed coz of transmission error');
  }
}); 

setTimeout(function(){
  var isdestroyed = socket.destroyed;
  console.log('Socket destroyed:' + isdestroyed);
  socket.destroy();
},1200000);

});

// emits when any error occurs -> calls closed event immediately after this.
server.on('error',function(error){
  console.log('Error: ' + error);
});

//emits when server is bound with server.listen
server.on('listening',function(){
  console.log('Server is listening!');
});

server.maxConnections = 10;

//static port allocation
server.listen(3007);
//OBJECT FORMAT
//"[{"name":"id","value":"dd01e0d1e313c493bd8dcb841088d6d6bcbca3b0c3cfe6d0c76df566f0b2577d"},{"name":"provider","value":"f39fd6e51aad88f6f4ce6ab8827279cfffb92266"},{"name":"subscriber","value":"9a9f2ccfde556a7e9ff0848998aa4a0cfd8863ae"},{"name":"query","value":"query"},{"name":"endpoint","value":"0da72197e898ebe1814471a76048ed137a089f595c1e7038f2b70e98645e7652"},{"name":"endpointParams","value":"[273c58c66704975459b45d17d8d262d7b6de59f5e57291ea5af890e2cf11a2b8,08455b9a244e3f6e53a4d620c3d1261bb351f203ccdf11324cfed2696b108913]"},{"name":"onchainSubscriber","value":"true"}]"
function processIncomingEventJSON(event_object){
  let temp={}
  console.log(event_object[0])
  event_object.forEach(item=>{
    temp[item.name]=item.value;

  })
  return temp
}
function formatEndpointParams(params){
 params=params.split(',')
 console.log(params)
 params[0]=params[0].slice(1)
 console.log(params.length)
 params[params.length-1]=params[params.length-1].slice(0,params[1].length-1)
 return params
}
// for dyanmic port allocation
/*server.listen(function(){
  var address = server.address();
  var port = address.port;
  var family = address.family;
  var ipaddr = address.address;
  console.log('Server is listening at port' + port);
  console.log('Server ip :' + ipaddr);
  console.log('Server is IP4/IP6 : ' + family);
});

**/

var islistening = server.listening;

if(islistening){
  console.log('Server is listening');
}else{
  console.log('Server is not listening');
}

setTimeout(function(){
  server.close();
},5000000);