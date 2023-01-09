
import { runCommand } from './cmd';
import { createProjectConfigFile } from './project';

let projects = [{
    name: "test",
    repo: "git@github.com:codyaverett/megarepo.git",
    branch: "main",
    path: "./"
}, {
    name: "metamega",
    repo: "git@github.com:codyaverett/megarepo.git",
    branch: "main",
    path: "./"
}];

// runCommand("ls", ["-lah"], {});
createProjectConfigFile(projects, ".");

