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
