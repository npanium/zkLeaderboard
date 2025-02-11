import { Card, CardContent } from "@/components/ui/card";

interface InstructionPanelProps {
  activeStep: number;
}

export function InstructionPanel({ activeStep }: InstructionPanelProps) {
  const getInstructionText = (step: number) => {
    switch (step) {
      case 0:
        return "Click 'Initialize Game' to start a new game session";
      case 1:
        return "Game data generated! Click 'Start Betting' to select random addresses";
      case 2:
        return "Place bets on selected addresses";
      case 3:
        return "ZK proofs generated! Click 'Resolve Bets' to determine winners";
      case 4:
        return "Bets resolved! Winners can claim their rewards";
      default:
        return "";
    }
  };

  return (
    <div className="p-5 text-sm text-muted-foreground">
      {getInstructionText(activeStep)}
    </div>
  );
}
