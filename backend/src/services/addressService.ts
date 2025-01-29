import crypto from "crypto";
import { AddressScore } from "../types/address";

export class AddressService {
  private static generateAddress(): string {
    const bytes = crypto.randomBytes(20);
    return "0x" + bytes.toString("hex");
  }

  private static generateScore(min: number = 100, max: number = 1000): number {
    return Math.floor(Math.random() * (max - min + 1)) + min;
  }

  public static generateAddresses(count: number = 1000): AddressScore[] {
    const pairs: AddressScore[] = [];
    for (let i = 0; i < count; i++) {
      pairs.push({
        address: this.generateAddress(),
        score: this.generateScore(),
      });
    }
    return pairs;
  }
}
