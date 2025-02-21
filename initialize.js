import 'dotenv/config';
import { mkdirSync, writeFileSync, rmSync, readFileSync, existsSync } from 'fs';
import { execSync } from 'child_process';
import path from 'path';
import { fileURLToPath } from 'url';
import { sync as glob } from 'glob';

// Load environment variables starting with PUBLIC_ into the environment,
// so we don't need to specify duplicate variables in .env
for (const key in process.env) {
  if (key.startsWith('PUBLIC_')) {
    process.env[key.substring(7)] = process.env[key];
  }
}

console.log('###################### Initializing ########################');

// Get dirname (equivalent to the Bash version)
const __filename = fileURLToPath(import.meta.url);
const dirname = path.dirname(__filename);
const contractsDir = `${dirname}/.stellar/contract-ids`;
const hashToContractIdMapping = `${dirname}/.wasm-hash-to-contract-id.json`;
// variable for later setting pinned version of soroban in "$(dirname/target/bin/soroban)"
const cli = 'stellar';

// Function to execute and log shell commands
function exe(command) {
  console.log(command);
  execSync(command, { stdio: 'inherit' });
}

function exeReturn(command) {
  console.log(command);
  return execSync(command).toString().trim();
}

function fundAll() {
  exe(`${cli} keys generate --fund ${process.env.STELLAR_ACCOUNT} | true`);
  exe(`${cli} keys generate --fund ${process.env.STELLAR_ACCOUNT2} | true`);
  exe(`${cli} keys generate --fund ${process.env.STELLAR_ACCOUNT3} | true`);
}

function removeFiles(pattern) {
  console.log(`remove ${pattern}`);
  glob(pattern).forEach((entry) => rmSync(entry));
}

function buildAll() {
  removeFiles(`${dirname}/target/wasm32-unknown-unknown/release/*.wasm`);
  removeFiles(`${dirname}/target/wasm32-unknown-unknown/release/*.d`);
  exe(`${cli} contract build`);
}

function filenameNoExtension(filename) {
  return path.basename(filename, path.extname(filename));
}

function deploy(wasm) {
  let name = filenameNoExtension(wasm);
  let stellar_args = `--source-account ${process.env.STELLAR_ACCOUNT} --network ${process.env.STELLAR_NETWORK}`;
  let wasm_hash = exeReturn(`${cli} contract install --wasm ${wasm} ${stellar_args}`);
  console.log(`Installed ${name} with hash ${wasm_hash}`);  
  // check if file exists
  let hashToContractId = {};
  let globalHashToContractId = {};
  if (existsSync(hashToContractIdMapping)) {
    globalHashToContractId = JSON.parse(readFileSync(hashToContractIdMapping));
    hashToContractId = globalHashToContractId[process.env.STELLAR_NETWORK_PASSPHRASE];
  }
  if (wasm_hash in hashToContractId) {
    console.log(`Contract ${name} already deployed with contract_id ${hashToContractId[wasm_hash]}`);
  } else {
    let contract_id = exeReturn(`${cli} contract deploy --wasm-hash ${wasm_hash} ${stellar_args} --alias ${name}`);
    console.log(`Deployed ${name} with contract_id ${contract_id}`);
    hashToContractId = {};
    hashToContractId[wasm_hash] = contract_id;
  }
  globalHashToContractId[process.env.STELLAR_NETWORK_PASSPHRASE] = hashToContractId;
  writeFileSync(hashToContractIdMapping, JSON.stringify(globalHashToContractId, null, 2));
}

function deployAll() {
  mkdirSync(contractsDir, { recursive: true });

  const wasmFiles = glob(`${dirname}/target/wasm32-unknown-unknown/release/*.wasm`);
  
  wasmFiles.forEach(deploy);
}

function contracts() {
  const contractFiles = glob(`${contractsDir}/*.json`);

  return contractFiles
    .map(path => ({
      alias: filenameNoExtension(path),
      ...JSON.parse(readFileSync(path))
    }))
    .filter(data => data.ids[process.env.STELLAR_NETWORK_PASSPHRASE])
    .map(data => ({alias: data.alias, id: data.ids[process.env.STELLAR_NETWORK_PASSPHRASE]}));
}

function bind({alias, id}) {
  exe(`${cli} contract bindings typescript --contract-id ${id} --output-dir ${dirname}/packages/${alias} --overwrite`);
  exe(`(cd ${dirname}/packages/${alias} && npm i && npm run build)`);
}

function bindAll() {
  contracts().forEach(bind);
}

function importContract({alias}) {
  const outputDir = `${dirname}/src/contracts/`;

  mkdirSync(outputDir, { recursive: true });

  const importContent =
    `import * as Client from '${alias}';\n` +
    `import { rpcUrl } from './util';\n\n` +
    `export default new Client.Client({\n` +
    `  ...Client.networks.${process.env.STELLAR_NETWORK},\n` +
    `  rpcUrl,\n` +
    `${
      process.env.STELLAR_NETWORK === "local" || "standalone"
        ? `  allowHttp: true,\n`
        : null
    }` +
    `});\n`;

  const outputPath = `${outputDir}/${alias}.ts`;

  writeFileSync(outputPath, importContent);

  console.log(`Created import for ${alias}`);
}

function importAll() {
  contracts().forEach(importContract);
}

function aliceAndBob() {
  let alice = exeReturn(`${cli} keys address ${process.env.STELLAR_ACCOUNT}`);
  let bob = exeReturn(`${cli} keys address ${process.env.STELLAR_ACCOUNT2}`);
  let carol = exeReturn(`${cli} keys address ${process.env.STELLAR_ACCOUNT3}`);

  let contract = JSON.parse(readFileSync(`${contractsDir}/retainer.json`)).ids[process.env.STELLAR_NETWORK_PASSPHRASE];
  let stellar_args = `--network ${process.env.STELLAR_NETWORK}`;
  // alice registers as retainor (with herself and bob as retainees)
  exe(`${cli} contract invoke --id ${contract} --source-account ${process.env.STELLAR_ACCOUNT} ${stellar_args} -- set_retainor_info --retainor ${alice} --name Alice --retainees '[ "${bob}", "${alice}" ]'`);
  // bob registers as retainee
  exe(`${cli} contract invoke --id ${contract} --source-account ${process.env.STELLAR_ACCOUNT2} ${stellar_args} -- set_retainee_info --retainee ${bob} --name Bob --retainors '[ "${alice}" ]'`);
  // alice registers as retainee for herself
  exe(`${cli} contract invoke --id ${contract} --source-account ${process.env.STELLAR_ACCOUNT} ${stellar_args} -- set_retainee_info --retainee ${alice} --name Alice --retainors '[ "${carol}", "${alice}" ]'`);
  // carol registers as a retainor with alice as retainee
  exe(`${cli} contract invoke --id ${contract} --source-account ${process.env.STELLAR_ACCOUNT3} ${stellar_args} -- set_retainor_info --retainor ${carol} --name Carol --retainees '[ "${alice}" ]'`);
}

// Calling the functions (equivalent to the last part of your bash script)
fundAll();
buildAll();
deployAll();
bindAll();
importAll();
aliceAndBob();

console.log('###################### Done ########################');
