import express, { Express } from "express";
import cors from "cors";
import addressRoutes from "./routes/addressRoutes";

const app: Express = express();
const port = process.env.PORT || 3000;

// Middleware
app.use(cors());
app.use(express.json());

// Routes
app.use("/api", addressRoutes);

// Start server
app.listen(port, () => {
  console.log(`Server running on port ${port}`);
});
