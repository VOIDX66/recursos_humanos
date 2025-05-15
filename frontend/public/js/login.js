document.getElementById("loginForm").addEventListener("submit", async (e) => {
  e.preventDefault();

  const email = e.target.email.value;
  const password = e.target.password.value;

  try {
    const res = await fetch("http://localhost:4500/auth/login_user", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ email, password }),
    });

    const data = await res.json();

    if (!res.ok) {
      alert(data.message || "Error en login");
      return;
    }

    localStorage.setItem("token", data.token);
    alert("Login exitoso");
    window.location.href = "/dashboard";
  } catch (error) {
    alert(error);
  }
});
