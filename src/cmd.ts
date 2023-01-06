
// function to run command on commandline

export function runCommand(command: string, args: string[], options: any) {
    const spawn = require("child_process").spawn;
    const cmd = spawn(command, args, options);
    cmd.stdout.on("data", (data: any) => {
        console.log(data.toString());
    });
    cmd.stderr.on("data", (data: any) => {
        console.log(data.toString());
    });
    cmd.on("exit", (code: any) => {
        console.log("child process exited with code " + code.toString());
    });
}
