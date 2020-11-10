import io from 'socket.io-client';

const socket = io('http://localhost:8080');

const pen = document.getElementById('pen');

const eraser = document.getElementById('eraser');

socket.on('connect', () => {
    pen.addEventListener('click', () => {
        socket.send('p');
    });

    eraser.addEventListener('click', () => {
        socket.send('e');
    })
});
