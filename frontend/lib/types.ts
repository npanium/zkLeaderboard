export interface GameData {
  address: string;
  score: number;
  verified: boolean;
  bets: number;
}

export interface Step {
  id: number;
  name: string;
  status: string;
  buttonText: string;
}

export interface ZKVerifyAttestation {
  attestationId: number;
  proofDetails: ProofDetails;
}

export interface ProofDetails {
  root: string;
  proof: string[];
  numberOfLeaves: number;
  leafIndex: number;
  leaf: string;
}
