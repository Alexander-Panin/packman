const dgram = require('dgram');
const client = dgram.createSocket('udp4');
client.bind( 41235, 'localhost');
const message = new Buffer('hello world');

var server = require('http').createServer( (req, res) => {
    res.writeHead(200, { 'Content-Type': 'text/html' });
    var io_str = '<script src="https://cdn.socket.io/socket.io-1.0.0.js"></script>';
    var js_str = `<script>
        var socketio = io.connect("localhost:3000");
        socketio.on('map', data => document.getElementById('m').innerHTML = data);
        document.onkeydown = e => socketio.emit('move', e.code);
    </script>`;

    var data = "";
    var m = `<tt><div id='m'>${data}</div></tt>`;
    res.end(io_str + js_str + m);

});
var io = require('socket.io')(server);
var k = 0;
io.on('connection', function(socket){
    socket.on('move', (data) => {
        client.send(data, 0, data.length, 41234, 'localhost');
    });
    client.on('message', (msg, rinfo) => {
        console.log(`server got: ${msg} ${k++} from ${rinfo.address}:${rinfo.port}`);
        socket.emit('map', `${msg}`);
    });
});
server.listen(3000);
