import path from 'path';
import { Options } from './opts';

export enum Operation {
    Print,
    Add,
    Remove,
}

export type Config = {
    args: string[],
    operation: Operation,
    config: string,
    pwd: string,
}

function getOperation(options: Options): Operation {
    switch (options.args?.[0]) {
        case 'add': return Operation.Add;
        case 'remove': return Operation.Remove;
        default: return Operation.Print;
    }
}

function getArgs(options: Options): string[] {
    if (!options.args) {
        return [];
    }

    const argsCount = options.args.length;

    switch (getOperation(options)) {
        case Operation.Print:
            if (argsCount > 1) {
                throw new Error(`Expected 0 or 1 arguements but recieved ${argsCount - 1}`);
            }

            return options.args;

        case Operation.Add:
            if (argsCount !== 3) {
                throw new Error(`Expected 2 arguements but recieved ${argsCount - 1}`);
            }

            return options.args.slice(1);

        case Operation.Remove:
            if (argsCount !== 2) {
                throw new Error(`Expected 1 arguements but recieved ${argsCount}`);
            }
            return options.args.slice(1);

        default:
            return [];
    }
}

function getConfig(options: Options): string {
    if (options.config) {
        return options.config;
    }

    const WIN_HOME = process.env['HOME'];
    const UBUNTU_HOME = process.env['XDG_CONFIG_HOME'];
    const location =  UBUNTU_HOME || WIN_HOME;

    if (!location) {
        throw new Error('Unable to determine config location');
    }

    if (location === WIN_HOME) {
        return path.join(location, '.projector.json');
    }

    return path.join(location, 'projector', 'projector.json');
}

function getPwd(options: Options): string {
    if (options.pwd) {
        return options.pwd;
    }

    return process.cwd();
}

export default function setupConfig(options: Options): Config {
    return {
        args: getArgs(options),
        operation: getOperation(options),
        config: getConfig(options),
        pwd: getPwd(options),
    };
}
