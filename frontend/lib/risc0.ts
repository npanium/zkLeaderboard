import axios from "axios";
import { RISC0_SERVICE_URL } from "./config";

export interface AddressData {
  address: string;
  score: number;
}

export interface ProofResponse {
  status: string;
  proof: string;
  journal: string;
  image_id: string;
  results: Array<{
    address: string;
    is_top_half: boolean;
  }>;
}

export async function submitAddressesForProof(
  addresses: string[]
): Promise<string> {
  const response = await axios.post(`${RISC0_SERVICE_URL}/check_position/`, {
    addresses,
  });
  return response.data; // job_id
}

export async function getProofStatus(jobId: string): Promise<ProofResponse> {
  const response = await axios.get(`${RISC0_SERVICE_URL}/job/${jobId}`);
  return response.data;
}
