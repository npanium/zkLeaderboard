import { NextRequest, NextResponse } from "next/server";
import { submitAddressesForProof, getProofStatus } from "@/lib/risc0";
import { verifyProofWithZkVerify } from "@/lib/zkVerify";

export async function POST(request: NextRequest) {
  try {
    const { addresses } = await request.json();
    console.log(`Addresses: `, addresses);
    // Submit to Risc0 service
    const jobId = await submitAddressesForProof(addresses);
    console.log("jobId: ", jobId);
    // Poll for proof completion
    let proofResponse;
    let attempts = 0;
    while (attempts < 30) {
      // 30 attempts with 2s delay = 1min max wait
      proofResponse = await getProofStatus(jobId);
      if (proofResponse.status === "completed") {
        break;
      }
      await new Promise((r) => setTimeout(r, 2000));
      attempts++;
    }

    if (!proofResponse || proofResponse.status !== "completed") {
      return NextResponse.json(
        { error: "Proof generation timed out" },
        { status: 408 }
      );
    }
    console.log("Proof response received. submitting to zkVerify");
    // Submit to zkVerify
    const zkVerifyResult = await verifyProofWithZkVerify(
      proofResponse.proof,
      proofResponse.image_id,
      proofResponse.journal
    );
    console.log("Success");
    return NextResponse.json({
      status: "success",
      zkVerifyAttestation: zkVerifyResult,
    });
  } catch (error) {
    console.error("Verification error:", error);
    return NextResponse.json({ error: "Verification failed" }, { status: 500 });
  }
}
