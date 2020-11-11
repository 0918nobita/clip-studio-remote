import { BehaviorSubject, fromEvent, merge } from 'rxjs';
import { filter, mapTo } from 'rxjs/operators';

import { WebSocketState } from './wsState';

const webSocket = new WebSocket('ws://localhost:8080/websocket');

const wsState$ = new BehaviorSubject<WebSocketState>(WebSocketState.Connecting);

fromEvent(webSocket, 'open').subscribe(() =>
    wsState$.next(WebSocketState.Open)
);

fromEvent(webSocket, 'close').subscribe(() =>
    wsState$.next(WebSocketState.Closed)
);

/* eslint-disable @typescript-eslint/no-non-null-assertion */
const penBtn = document.getElementById('pen')!;
const eraserBtn = document.getElementById('eraser')!;
/* eslint-enable @typescript-eslint/no-non-null-assertion */

const whenWsIsOpen = filter(() => wsState$.value === WebSocketState.Open);

const pen$ = fromEvent(penBtn, 'click').pipe(whenWsIsOpen, mapTo('p'));

const eraser$ = fromEvent(eraserBtn, 'click').pipe(whenWsIsOpen, mapTo('e'));

merge(pen$, eraser$).subscribe((c) => webSocket.send(c));
