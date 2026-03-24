import { describe, it, expect } from "vitest";
import { truncateAddress, satoshisToBtc, btcToSatoshis, formatDate } from "../lib/utils";

describe("truncateAddress", () => {
  it("truncates long addresses", () => {
    const addr = "abcdefghijklmnopqrstuvwxyz1234567890";
    const result = truncateAddress(addr, 8);
    expect(result).toBe("abcdefgh...34567890");
  });

  it("returns short addresses unchanged", () => {
    const addr = "short";
    expect(truncateAddress(addr)).toBe("short");
  });
});

describe("satoshisToBtc", () => {
  it("converts satoshis to BTC string", () => {
    expect(satoshisToBtc(100000000)).toBe("1.00000000");
    expect(satoshisToBtc(50000000)).toBe("0.50000000");
    expect(satoshisToBtc(1)).toBe("0.00000001");
    expect(satoshisToBtc(0)).toBe("0.00000000");
  });
});

describe("btcToSatoshis", () => {
  it("converts BTC to satoshis", () => {
    expect(btcToSatoshis(1)).toBe(100000000);
    expect(btcToSatoshis(0.5)).toBe(50000000);
    expect(btcToSatoshis(0.00000001)).toBe(1);
  });
});

describe("formatDate", () => {
  it("returns 'just now' for recent dates", () => {
    const now = new Date().toISOString();
    expect(formatDate(now)).toBe("just now");
  });
});
