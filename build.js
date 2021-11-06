const { execSync } = require('child_process');

function buildWasm(pkg, buildCommand, target_dir) {
    buildCommand.push(pkg);

    const underscoredName = pkg.replace(/-/g, '_');

    console.log(`Building ${underscoredName}.wasm`);
    execSync(buildCommand.join(' '));

    const optCommand = [
        'ic-cdk-optimizer',
        `target/wasm32-unknown-unknown/${target_dir}/${underscoredName}.wasm`,
        '-o',
        `target/wasm32-unknown-unknown/${target_dir}/${underscoredName}-opt.wasm`,
    ];

    console.log(`Running ic-cdk-optimizer on ${underscoredName}.wasm`);
    execSync(optCommand.join(' '));
}

let buildType = (process.env.BUILD_TYPE || "Release").toUpperCase();
console.log(`Building in ** ${buildType} ** mode`);

let buildCommand = "";
let target_dir = "";
switch (buildType)
{
    case "DEBUG":
        buildCommand =
        [
            'RUSTFLAGS="--cfg debug_cfg"',
            'cargo',
            'build',
            '--target',
            'wasm32-unknown-unknown',
            '--package',
        ]
        target_dir = "debug"
        break;
    default:
        buildCommand =
        [
            'RUSTFLAGS="--cfg release_cfg"',
            'cargo',
            'build',
            '--target',
            'wasm32-unknown-unknown',
            '--release',
            '--package',
        ]
        target_dir = "release"
}

buildWasm('cap-example', [...buildCommand], target_dir);
