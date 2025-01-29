import { Router, Request, Response } from "express";
import { AddressService } from "../services/addressService";

const router = Router();

router.get("/addresses", (req: Request, res: Response) => {
  try {
    const count = parseInt(req.query.count as string) || 1000;
    const addresses = AddressService.generateAddresses(count);
    res.json(addresses);
  } catch (error) {
    res.status(500).json({
      error: error instanceof Error ? error.message : "Unknown error occurred",
    });
  }
});

export default router;
