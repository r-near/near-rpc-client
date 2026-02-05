/**
 * Example: Query NEAR mainnet for various information
 * 
 * Run with: bun run examples/mainnet.ts
 */

import { NearRpcClient } from "../src/client.js";

async function main() {
  const client = NearRpcClient.mainnet();

  console.log("=== NEAR Mainnet RPC Client Test (TypeScript) ===\n");

  // 1. Get node status
  console.log("1. Fetching node status...");
  const status = await client.status();
  console.log(`   Chain ID: ${status.chain_id}`);
  console.log(`   Protocol version: ${status.protocol_version}`);
  console.log(`   Latest block height: ${status.sync_info.latest_block_height}`);
  console.log(`   Latest block hash: ${status.sync_info.latest_block_hash}`);
  console.log(`   Syncing: ${status.sync_info.syncing}`);
  console.log();

  // 2. Get the latest block
  console.log("2. Fetching latest block...");
  const block = await client.block({ finality: "final" });
  console.log(`   Block height: ${block.header.height}`);
  console.log(`   Block hash: ${block.header.hash}`);
  console.log(`   Timestamp: ${block.header.timestamp}`);
  console.log(`   Author: ${block.author}`);
  console.log(`   Chunks: ${block.chunks.length}`);
  console.log();

  // 3. Get gas price
  console.log("3. Fetching gas price...");
  const gas = await client.gasPrice({ block_id: null });
  console.log(`   Gas price: ${gas.gas_price} yoctoNEAR`);
  console.log();

  // 4. Query account
  console.log("4. Querying 'near' foundation account...");
  const accountQuery = await client.query({
    request_type: "view_account",
    finality: "final",
    account_id: "near",
  });
  
  // The response type depends on request_type
  if ("amount" in accountQuery) {
    const balanceYocto = BigInt(accountQuery.amount);
    const balanceNear = balanceYocto / BigInt(10 ** 24);
    console.log(`   Account: near`);
    console.log(`   Balance: ${balanceNear} NEAR`);
    console.log(`   Storage used: ${accountQuery.storage_usage} bytes`);
  }
  console.log();

  // 5. Call a view function on a contract
  console.log("5. Calling view function on wrap.near...");
  const callResult = await client.query({
    request_type: "call_function",
    finality: "final",
    account_id: "wrap.near",
    method_name: "ft_metadata",
    args_base64: btoa("{}"), // base64 encode "{}"
  });

  if ("result" in callResult && Array.isArray(callResult.result)) {
    const decoded = new TextDecoder().decode(new Uint8Array(callResult.result));
    console.log(`   wNEAR metadata: ${decoded}`);
  }
  console.log();

  // 6. Get validators
  console.log("6. Fetching current validators...");
  const validators = await client.validators({ latest: null });
  console.log(`   Current validators: ${validators.current_validators.length}`);
  console.log(`   Next validators: ${validators.next_validators.length}`);
  if (validators.current_validators.length > 0) {
    const first = validators.current_validators[0];
    console.log(`   Top validator: ${first.account_id} (stake: ${first.stake})`);
  }
  console.log();

  // 7. Network info
  console.log("7. Fetching network info...");
  const network = await client.networkInfo();
  console.log(`   Active peers: ${network.active_peers.length}`);
  console.log(`   Known producers: ${network.known_producers.length}`);
  console.log();

  console.log("=== All tests passed! ===");
}

main().catch((err) => {
  console.error("Error:", err);
  process.exit(1);
});
