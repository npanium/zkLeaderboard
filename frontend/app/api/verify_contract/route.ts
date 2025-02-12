// app/api/verify/route.ts

import { NextResponse } from "next/server";
import { ethers } from "ethers";

const ZK_VERIFY_ADDRESS = "0x82941a739E74eBFaC72D0d0f8E81B1Dac2f586D5";
const RPC_URL = "https://sepolia-rollup.arbitrum.io/rpc";

interface VerificationRequest {
  attestationId: string;
  leaf: string;
  merklePath: string[];
  leafCount: string;
  index: string;
}

export async function POST(request: Request) {
  try {
    const body: VerificationRequest = await request.json();

    const provider = new ethers.JsonRpcProvider(RPC_URL);

    const iface = new ethers.Interface([
      "function verifyProofAttestation(uint256,bytes32,bytes32[],uint256,uint256) view returns (bool)",
    ]);

    const calldata = iface.encodeFunctionData("verifyProofAttestation", [
      body.attestationId,
      body.leaf,
      body.merklePath,
      body.leafCount,
      body.index,
    ]);

    const result = await provider.call({
      to: ZK_VERIFY_ADDRESS,
      data: calldata,
    });

    const decodedResult = iface.decodeFunctionResult(
      "verifyProofAttestation",
      result
    )[0];

    return NextResponse.json({
      success: true,
      verified: decodedResult,
    });
  } catch (error) {
    console.error("Verification error:", error);
    return NextResponse.json(
      {
        success: false,
        error: error instanceof Error ? error.message : "Unknown error",
      },
      { status: 500 }
    );
  }
}
