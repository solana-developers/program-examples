// Deploy a solana program by execing a shell.
// Returns stdout when the deploy was successful.
export async function deployProgramShell(programPath: string) {
  const util = require('util');
  const exec = util.promisify(require('child_process').exec);
  try {
    const { stdout, stderr } = await exec(`solana program deploy ${programPath}`);
    if (stderr) {
      return stderr;
    }
    return stdout;
  } catch (e) {
    console.error(e);
    return e;
  }
}

export function parseProgramID(programIdLog: string) : string {
  // The string should of of the following form, else it is an error.
  // Program Id: 5K4yQ8KW2CKsBVRUw2193GMnkKBKXDm6sdXnYY1cB4Hy
  programIdLog = programIdLog.trim();
  let templateString : string = 'Program Id: 5K4yQ8KW2CKsBVRUw2193GMnkKBKXDm6sdXnYY1cB4Hy';
  let logLength = templateString.length;
  if (programIdLog.length != logLength) {
    console.log(`Different lenghts. Expected: ${logLength} vs. ${programIdLog.length}`);
    return null;
  }
  if (!programIdLog.startsWith('Program Id:')){
    console.log('program does not starts with.');
    return null;
  }
  const programId = programIdLog.substring('Program Id: '.length);
  return programId;
}

async function main() {
  const programPath = 'bin/hello_solana_move_program.so';
  const programIdLog = await deployProgramShell(programPath);
  const programId = parseProgramID(programIdLog);
  if (programId) {
    console.log('Program deployed with', programId);
    return 0;
  }
  return -1;
}
