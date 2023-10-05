import cli from "command-line-args";

export type ProjectorOptions = {
    args?: string[],
    pwd?: string,
    config?: string,
}

export default function getOpts(): ProjectorOptions {
    return cli([
        {
            name: "args",
            defaultOption: true,
            multiple: true,
            type: String,
        }, {
            name: "config",
            alias: "c",
            type: String,
        }, {
            name: "pwd",
            alias: "p",
            type: String,
        },
    ]) as ProjectorOptions;
}
