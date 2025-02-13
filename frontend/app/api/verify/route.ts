import { NextRequest, NextResponse } from "next/server";
import { submitAddressesForProof, getProofStatus } from "@/lib/risc0";
import { verifyProofWithZkVerify } from "@/lib/zkVerify";
import { Contract, ethers } from "ethers";
import { abi as ContractABI } from "@/lib/VerificationAndPrize_ABI.json";

export async function POST(request: NextRequest) {
  try {
    const { addresses } = await request.json();
    console.log(`Addresses: `, addresses);
    // Submit to Risc0 service
    const jobId = await submitAddressesForProof(addresses);
    console.log("jobId: ", jobId);
    // Poll for proof completion
    let proofResponse;
    let journal = "";
    let results;

    let attempts = 0;
    while (attempts < 30) {
      // 30 attempts with 2s delay = 1min max wait
      proofResponse = await getProofStatus(jobId);
      if (proofResponse.status === "completed") {
        journal = proofResponse.journal;
        results = proofResponse.results;
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
    console.log("Success. Calling Verification Prize contract");

    // Call smart contract
    const provider = new ethers.JsonRpcProvider(process.env.ARBITRUM_RPC_URL);
    const wallet = new ethers.Wallet(process.env.PRIVATE_KEY!, provider);
    const contract = new Contract(
      process.env.VERIFICATION_PRIZE_CONTRACT_ADDRESS!,
      ContractABI,
      wallet
    );

    const attestationId = zkVerifyResult.attestationId;
    const merklePath = zkVerifyResult.proofDetails.proof;
    const leaf = zkVerifyResult.proofDetails.leaf;
    const leafCount = zkVerifyResult.proofDetails.numberOfLeaves;
    const index = zkVerifyResult.proofDetails.leafIndex;
    const winners = proofResponse.results.map((result) => result.is_top_half);

    const tx = await contract.VerifyWinnersAndProcess(
      leaf,
      attestationId,
      merklePath,
      leafCount,
      index,
      winners
    );

    await tx.wait();

    return NextResponse.json({
      status: "success",
      zkVerifyAttestation: zkVerifyResult,
      contractTransaction: tx.hash,
      journal: proofResponse.journal,
      results: proofResponse.results,
    });
  } catch (error) {
    console.error("Verification error:", error);
    return NextResponse.json({ error: "Verification failed" }, { status: 500 });
  }
}
