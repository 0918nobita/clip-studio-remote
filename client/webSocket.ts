import { BehaviorSubject, fromEvent, Observable } from 'rxjs';

export enum WebSocketState {
    Connecting,
    Open,
    Closed,
}

export const createWsStateStream = (
    ws: WebSocket
): Observable<WebSocketState> => {
    const state$ = new BehaviorSubject<WebSocketState>(
        WebSocketState.Connecting
    );

    fromEvent(ws, 'open').subscribe(() => state$.next(WebSocketState.Open));

    fromEvent(ws, 'close').subscribe(() => state$.next(WebSocketState.Closed));

    return state$.asObservable();
};
