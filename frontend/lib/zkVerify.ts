import { ZkVerifyEvents, zkVerifySession } from "zkverifyjs";
import { ZKVERIFY_SEED_PHRASE } from "./config";

export async function verifyProofWithZkVerify(
  proof: string,
  imageId: string,
  pubInputs: string
) {
  const session = await zkVerifySession
    .start()
    .Testnet()
    .withAccount(ZKVERIFY_SEED_PHRASE);

  let attestationId: number, leafDigest: string, blockHash: any;
  return new Promise(async (resolve, reject) => {
    const { events } = await session
      .verify()
      .risc0()
      .waitForPublishedAttestation()
      .execute({
        proofData: {
          proof: proof,
          vk: imageId,
          publicSignals: pubInputs,
          version: "V1_2",
        },
      });

    events.on(ZkVerifyEvents.IncludedInBlock, (eventData) => {
      console.log("Transaction included in block:", eventData);
      leafDigest = eventData.leafDigest;
      blockHash = eventData.blockHash;
      attestationId = eventData.attestationId;
    });

    events.on(ZkVerifyEvents.Finalized, (eventData) => {
      console.log("Transaction finalized:", eventData);
    });

    events.on(ZkVerifyEvents.AttestationConfirmed, async (eventData) => {
      console.log(
        `attestation ID: ${attestationId} \nleafDigest: ${leafDigest} \nblockHash: ${blockHash}`
      );
      const proofDetails = await session.poe(attestationId, leafDigest);
      console.log("attestation: ", JSON.stringify(proofDetails, null, 2));
      console.log("proofDetails", proofDetails);
      resolve({
        attestationId: eventData.id,
        proofDetails,
      });
    });

    events.on("error", (error: any) => {
      reject(error);
    });
  });
}
// Above can take some time
