// Prepare ShareImage for publishing
import { parse, stringify } from "deno.land/x/std@0.204.0/toml/mod.ts";
import { parse } from "https://deno.land/std@0.202.0/flags/mod.ts";

const flags = parse(Deno.args, {
  string: ["version"],
});

const TOML = await Deno.readTextFile("Cargo.toml");
let data = parse(TOML);
// Update version
data.package.version = flags.version;

const str = stringify(data);
await Deno.writeTextFile("Cargo.toml", str);