import { fromEvent, merge } from 'rxjs';
import { filter, mapTo, mergeMap } from 'rxjs/operators';

import { createWsStateStream, WebSocketState } from './webSocket';

const webSocket = new WebSocket('ws://localhost:8080/websocket');

const wsState$ = createWsStateStream(webSocket);

/* eslint-disable @typescript-eslint/no-non-null-assertion */
const penBtn = document.getElementById('pen')!;
const eraserBtn = document.getElementById('eraser')!;
/* eslint-enable @typescript-eslint/no-non-null-assertion */

const whenWsIsOpen = mergeMap(() =>
    wsState$.pipe(filter((s) => s === WebSocketState.Open))
);

const pen$ = fromEvent(penBtn, 'click').pipe(whenWsIsOpen, mapTo('p'));

const eraser$ = fromEvent(eraserBtn, 'click').pipe(whenWsIsOpen, mapTo('e'));

merge(pen$, eraser$).subscribe((c) => webSocket.send(c));
