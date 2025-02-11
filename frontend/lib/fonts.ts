import { Poppins, Chakra_Petch } from "next/font/google";

export const poppins = Poppins({
  weight: ["300", "400", "600"],
  subsets: ["latin"],
});
export const chakra = Chakra_Petch({
  weight: ["400", "600"],
  subsets: ["latin"],
  display: "swap",
});
