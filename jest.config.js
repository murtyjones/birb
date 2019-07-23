// jest.config.js
const { pathsToModuleNameMapper } = require('ts-jest/utils');
// In the following statement, replace `./tsconfig` with the path to your `tsconfig` file
// which contains the path mapping (ie the `compilerOptions.paths` option):
const { compilerOptions } = require('./tsconfig');

module.exports = {
    "preset": 'ts-jest',
    "moduleNameMapper": pathsToModuleNameMapper(compilerOptions.paths, { prefix: '<rootDir>/js/' } ),
    "moduleFileExtensions": [
        "ts",
        "tsx",
        "js"
    ],
        "transform": {
        "\\.(ts|tsx)$": "<rootDir>/node_modules/ts-jest/preprocessor.js"
    },
    "setupFiles": [
        "raf/polyfill"
    ],
        "testRegex": "/__tests__/.*\\.(ts|tsx|js)$",
        "setupFilesAfterEnv": ["<rootDir>/js/setupTests.ts"],
        "snapshotSerializers": [
        "enzyme-to-json"
    ]
};
