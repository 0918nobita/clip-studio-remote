const Koa = require('koa');
const serve = require('koa-static');
const http = require('http');
const socketIo = require('socket.io');
const sendKeys = require('sendkeys-macos');

const app = new Koa();

app.use(serve('./client'));

const server = http.createServer(app.callback());
const io = socketIo(server);

io.on('connect', (socket) => {
    socket.on('message', (keystrokes) => {
        sendKeys(void 0, keystrokes)
    });
});

server.listen(8080);
