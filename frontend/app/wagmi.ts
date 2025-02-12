import { getDefaultConfig } from "@rainbow-me/rainbowkit";
import { arbitrumSepolia } from "wagmi/chains";

export const config = getDefaultConfig({
  appName: "zkLeaderboard",
  projectId: "a10b532424f0f1617a6c9c35ec0aa1fe",
  chains: [arbitrumSepolia],
  ssr: true,
});
