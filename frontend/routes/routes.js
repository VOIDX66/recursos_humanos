import express from "express";
const router = express.Router();

router.get("/", (req, res) => {
  res.render("index"); // o login
});

router.get("/login", (req, res) => {
  res.render("login");
});

router.get("/dashboard", (req, res) => {
  res.render("dashboard");
});

router.get("/dashboard_admin", (req, res) => {
  res.render("dashboard_admin");
});

export default router;
