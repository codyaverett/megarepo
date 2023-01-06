import { runCommand } from './cmd';

export function cloneRepo(repo: string, dir: string) {
    runCommand(
        "git"
        , ["clone", repo, dir]
        , {}
    );
}

export function pullRepo(dir: string) {
    runCommand(
        "git"
        , ["pull"]
        , { cwd: dir }
    );
}

export function pushRepo(dir: string) {
    runCommand(
        "git"
        , ["push"]
        , { cwd: dir }
    );
}

export function commitRepo(dir: string, message: string) {
    runCommand(
        "git"
        , ["commit", "-am", message]
        , { cwd: dir }
    );
}

export function addRepo(dir: string, file: string) {
    runCommand("git", ["add", file],
        { cwd: dir });
}