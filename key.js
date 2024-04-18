const bs58 = require('bs58');
const fs = require('fs');
b = bs58.decode('3W52qfqgqVt3XH5PSFCL8TjEvbPG8kC5S1itUUzVEucnGqeFSGfLM9SARHkP3XxRUH3ahYi4tWbJ6QiHmGHZUGQZ');
j = new Uint8Array(b.buffer, b.byteOffset, b.byteLength / Uint8Array.BYTES_PER_ELEMENT);
fs.writeFileSync('key.json', `[${j}]`);