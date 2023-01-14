import fs from "fs";
import path from "path";

type Project = {
    name: string;
    repo: string;
    branch: string;
    path: string;
};

// function to read a project yaml configuration file
export function readProjectConfigFile(projectRoot: string): Project[] {
    let projects: Project[] = [];
    let yaml = fs.readFileSync(path.join(projectRoot, "projects.yaml"), "utf8");

    let lines = yaml.split("\n");
    let project: Project = {
        name: "",
        repo: "",
        branch: "",
        path: ""
    };
    for (let line of lines) {
        if (line.startsWith("- ")) {
            if (project.name !== "") {
                projects.push(project);
            }
            project = {
                name: "",
                repo: "",
                branch: "",
                path: ""
            };
            project.name = line.replace("- ", "");
        }
    }

    return projects;
}

// function to create a project yaml configuration file
export function createProjectConfigFile(projects: Project[], projectRoot: string) {
    let yaml = "";
    for (let project of projects) {
        yaml += `- ${project.name}:\n`;
        yaml += `  repo: ${project.repo}\n`;
        yaml += `  branch: ${project.branch}\n`;
        yaml += `  path: ${project.path}\n`;
        yaml += `\n`;
    }
    fs.writeFileSync(path.join(projectRoot, "projects.yaml"), yaml);
}
