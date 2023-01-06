import fs from "fs";
import path from "path";

type Project = {
    name: string;
    repo: string;
    branch: string;
    path: string;
};

// function to create a project configuration file
export function createProjectConfigFile(projects: Project[], projectRoot: string) {
    fs.writeFileSync(
        path.join(projectRoot, "project.json"),
        JSON.stringify(projects, null, 2)
    );
}
