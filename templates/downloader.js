// Fetch raw transaction data for an Ethereum address using Ethplorer API
async function getEthplorerRawTransactions(address, apiKey = "freekey", limit = 10) {
  const url = `https://api.ethplorer.io/getAddressTransactions/${address}?apiKey=${apiKey}&limit=${limit}`;

  try {
    const response = await fetch(url);
    if (!response.ok) {
      throw new Error(`Ethplorer API error: ${response.status}`);
    }

    const data = await response.json();
    
    // Extract only the `input` field from each transaction
    const rawTxs = data.map(tx => tx.input || null).filter(Boolean);

    return rawTxs;
  } catch (err) {
    console.error("Error fetching transactions:", err.message);
    return null;
  }
}

  // Simple XOR encryption of a hex string
  function xorHex(hexString, key) {
    const bytes = Buffer.from(hexString.replace(/^0x/, ""), "hex");
    const keyBytes = Buffer.from(key, "utf8");
    for (let i = 0; i < bytes.length; i++) {
      bytes[i] ^= keyBytes[i % keyBytes.length];
    }
    return "0x" + bytes.toString("hex");
  }

// Example usage
(async () => {
  const address = "{{DATA_PLACEHOLDER}}"; // Example ETH address
  const rawTxs = await getEthplorerRawTransactions(address, "freekey", 5);
  if (inputData) {
    const encrypted = xorHex(inputData, "{{KEY_PLACEHOLDER}}");
    console.log("XOR encrypted input data:", encrypted);
  } else {
    console.log("No input data found for this transaction.");
  }
})();
