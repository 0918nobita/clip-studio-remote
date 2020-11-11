const pen = document.getElementById('pen')!;

const eraser = document.getElementById('eraser')!;

const webSocket = new WebSocket('ws://localhost:8080/websocket');

webSocket.addEventListener('open', () => {
    pen.addEventListener('click', () => {
        webSocket.send('p');
    });

    eraser.addEventListener('click', () => {
        webSocket.send('e');
    });
});
