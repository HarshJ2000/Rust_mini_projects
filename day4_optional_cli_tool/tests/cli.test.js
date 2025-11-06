import { test, expect } from "vitest";
import { execSync } from "child_process";

test("CLI doubles number correctly or not?", () => {
  const output = execSync(
    "cargo run --manifest-path ../Cargo.toml -- 5 --double"
  )
    .toString()
    .trim();
  expect(output).toContain("Result: 10");
});
