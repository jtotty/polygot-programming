import path from 'path';
import { Config } from './config';
import fs from 'fs';

export type Data = {
    projector: Record<string, Record<string, string>>
}

const defaultData = {
    projector: {},
};

export default class Projector {
    #config: Config;
    #data: Data;

    constructor(config: Config, data: Data) {
        this.#config = config;
        this.#data = data;
    }

    getValueAll(): Record<string, string> {
        let curr = this.#config.pwd;
        let prev = '';
        const paths = [];

        do {
            prev = curr;
            paths.push(curr);
            curr = path.dirname(curr);
        } while (curr !== prev);

        const out = paths.reverse().reduce((acc, path) => {
            const value = this.#data.projector[path];
            if (value) {
                Object.assign(acc, value);
            }

            return acc;
        }, {});

        return out;
    }

    getValue(key: string): string | undefined {
        let curr = this.#config.pwd;
        let prev = '';
        let out: string | undefined = undefined;

        do {
            const value = this.#data.projector[curr]?.[key];
            if (value) {
                out = value;
                break;
            }

            prev = curr;
            curr = path.dirname(curr);
        } while (curr !== prev);

        return out;
    }

    setValue(key: string, value: string) {
        let dir = this.#data.projector[this.#config.pwd];
        if (!dir) {
            dir = this.#data.projector[this.#config.pwd] = {};
        }

        dir[key] = value;
    }

    removeValue(key: string) {
        const dir = this.#data.projector[this.#config.pwd];
        if (dir) {
            delete dir[key];
        }
    }

    static fromConfig(config: Config): Projector {
        if (fs.existsSync(config.config)) {
            let data: Data | undefined = undefined;

            try {
                data = JSON.parse(fs.readFileSync(config.config).toString());
            } catch (e) {
                data = defaultData;
            }

            return new Projector(config, data as Data);
        }

        return new Projector(config, defaultData);
    }
}
