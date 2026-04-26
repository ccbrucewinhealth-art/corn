local M = {}

function M.user_schema()
  return {
    fields = {"username", "password", "permissions", "groups"}
  }
end

return M
