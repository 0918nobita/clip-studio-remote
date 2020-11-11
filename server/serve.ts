import * as path from 'path';
import { fastify } from 'fastify';
import staticPlugin from 'fastify-static';
import webSocketPlugin from 'fastify-websocket';
import sendKeys = require('sendkeys-macos');

const app = fastify();

app.register(staticPlugin, {
    root: path.join(__dirname, '../client'),
});

app.register(webSocketPlugin);

app.get('/websocket', { websocket: true }, (conn) => {
    conn.socket.on('message', (keystrokes: string) => {
        sendKeys(void 0, keystrokes);
    });
});

app.listen(8080);
