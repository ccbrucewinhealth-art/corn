print("load ui/router.lua Ver.0.3")

return {
  { method = "GET", path = "/tacos/login", multiPart="false", contentType = "text/html", handler = "controller.auth.login_page" },
  { path= "api", endpoints=[
    { method = "POST", path = "0.1/tacos/login", multiPart="false", contentType = "text/json",handler = "api.auth.login" },
    { method = "GET", path = "0.1/tacos/profile", multiPart="false",  contentType = "text/json",handler = "api.auth.profile" },
    { method = "POST", path = "0.1/tacos/report", multiPart="true",  contentType = "text/json",handler = "api.auth.profile" }
  ]}
}
