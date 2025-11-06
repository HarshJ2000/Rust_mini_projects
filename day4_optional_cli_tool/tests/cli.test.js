import { test, expect } from "vitest";
import { execSync } from "child_process";
import { Connection } from "@solana/web3.js";

test("CLI doubles number correctly or not?", () => {
  const output = execSync(
    // execSync is used to execute the mentioned file with some arguments
    "cargo run --manifest-path ../Cargo.toml -- 5 --double"
  )
    .toString()
    .trim();
  expect(output).toContain("Result: 10"); // .toContain checks if the mentioned string is available in the expected output
});

test("Solana connection check", async () => {
  const connection = new Connection("http://127.0.0.1:8899");
  const version = await connection.getVersion(); // getting version to confirm some connection is available
  expect(version).toBeTruthy(); // .toBeTruthy checks if the value is converted to boolean will be true
});
