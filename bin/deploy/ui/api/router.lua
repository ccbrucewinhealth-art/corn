return {
  { method = "GET", path = "/{uipath}/tacos/login", handler = "controller.auth.login_page" },
  { method = "POST", path = "/api/tacos/login", handler = "api.auth.login" },
  { method = "GET", path = "/api/tacos/profile", handler = "api.auth.profile" }
}

