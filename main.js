const {exec} = require('child_process');


exec('cargo build', () => {
    const run = exec('./target/debug/aoc2021');
    run.stdout.pipe(process.stdout);
});