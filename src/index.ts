
import { runCommand } from './cmd';
import { createProjectConfigFile } from './project';

console.log("Hello World!");

let projects = [{
    name: "test",
    repo: "git@github.com:codyaverett/megarepo.git",
    branch: "main",
    path: "./"
}];

// runCommand("ls", ["-lah"], {});
createProjectConfigFile(projects, ".");

