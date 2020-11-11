import { spawn } from 'child_process';
import { fastify } from 'fastify';
import staticPlugin from 'fastify-static';
import webSocketPlugin from 'fastify-websocket';
import * as path from 'path';

const proc = spawn(`osascript`, ['-l', 'JavaScript', '-i']);

const app = fastify();

app.register(staticPlugin, {
    root: path.join(__dirname, '../client'),
});

app.register(webSocketPlugin);

app.get('/websocket', { websocket: true }, (conn) => {
    conn.socket.on('message', (keystrokes: string) => {
        const char = keystrokes.charAt(0);
        proc.stdin.write(`Application('System Events').keystroke('${char}')\n`);
    });
});

app.listen(8080);
