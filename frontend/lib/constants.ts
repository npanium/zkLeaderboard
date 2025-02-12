import { Step } from "./types";

export const STEPS: Step[] = [
  {
    id: 1,
    name: "Game Simulation",
    status: "not-started",
    buttonText: "Simulate Gameplay",
  },
  {
    id: 2,
    name: "Betting Window",
    status: "not-started",
    buttonText: "Start Betting Period",
  },
  {
    id: 3,
    name: "Proof Generation",
    status: "not-started",
    buttonText: "Generate Proofs",
  },
  {
    id: 4,
    name: "Bet Resolution",
    status: "not-started",
    buttonText: "Resolve Bets",
  },
];
