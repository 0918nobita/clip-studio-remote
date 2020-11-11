import { of } from 'rxjs';

test('addition', () => {
    expect(3 + 4).toBe(7);
});

test('observable', (done) => {
    of(200).subscribe((num) => {
        expect(num).toBe(200);
        done();
    });
});
