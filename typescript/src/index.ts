import setupConfig, { Operation } from './config';
import getOptions from './opts';
import Projector from './projector';

const options = getOptions();
const config = setupConfig(options);
const proj = Projector.fromConfig(config);

if (config.operation === Operation.Print) {
    if (config.args.length === 0) {
        console.log(JSON.stringify(proj.getValueAll()));
    } else {
        const value = proj.getValue(config.args[0]);
        if (value) {
            console.log(value);
        }
    }
}

if (config.operation === Operation.Add) {
    proj.setValue(config.args[0], config.args[1]);
    proj.save();
}

if (config.operation === Operation.Remove) {
    proj.removeValue(config.args[0]);
    proj.save();
}
