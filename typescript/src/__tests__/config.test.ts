import setupConfig, { Operation } from '../config';

test('no args', () => {
    const config = setupConfig({});
    expect(config.operation).toEqual(Operation.Print);
    expect(config.args).toEqual([]);
});

test('print operation', () => {
    const config = setupConfig({
        args: ['foo'],
    });
    expect(config.operation).toEqual(Operation.Print);
    expect(config.args).toEqual(['foo']);
});

test('add operation', () => {
    const config = setupConfig({
        args: ['add', 'foo', 'bar'],
    });
    expect(config.operation).toEqual(Operation.Add);
    expect(config.args).toEqual(['foo', 'bar']);
});

test('remove operation', () => {
    const config = setupConfig({
        args: ['remove', 'foo'],
    });
    expect(config.operation).toEqual(Operation.Remove);
    expect(config.args).toEqual(['foo']);
});
