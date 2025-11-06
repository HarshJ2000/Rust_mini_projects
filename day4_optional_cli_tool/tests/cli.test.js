import { test, expect } from "vitest";
import { execSync } from "child_process";
import { Connection } from "@solana/web3.js";

test("CLI doubles number correctly or not?", () => {
  const output = execSync(
    "cargo run --manifest-path ../Cargo.toml -- 5 --double"
  )
    .toString()
    .trim();
  expect(output).toContain("Result: 10");
});

test("Solana connection check", async () => {
  const connection = new Connection("http://127.0.0.1:8899");
  const version = await connection.getVersion();
  expect(version).toBeTruthy();
});
